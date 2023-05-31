use std::{fmt::Debug, panic::AssertUnwindSafe, sync::Arc, thread};
use cucumber::{cli, codegen::WorldInventory, event, parser, step, Event, World};
use derive_more::{Display, From};
use futures::{
    executor::block_on,
    future,
    stream::{self, LocalBoxStream},
    FutureExt, Stream, StreamExt, TryStreamExt,
};

mod executor;

#[derive(Default)]
pub struct SyncRunner<W: World + WorldInventory + Debug + Send> {
    _phantom: std::marker::PhantomData<W>,
}

impl<W> SyncRunner<W>
where
    W: World + WorldInventory + Debug + Clone + Send + Sync + 'static,
    <W as World>::Error: Debug,
{
    fn steps_fns() -> step::Collection<W> {
        W::collection()
    }

    async fn execute_step(mut world: W, step: gherkin::Step) -> (W, SyncStep<W>) {
        let ev = if let Some((step_fn, captures, loc, ctx)) =
            Self::steps_fns().find(&step).expect("Ambiguous match")
        {
            // Panic represents a failed assertion in a step matching
            // function.
            match AssertUnwindSafe(step_fn(&mut world, ctx))
                .catch_unwind()
                .await
            {
                Ok(()) => SyncStep::Passed(captures, loc),
                Err(e) => SyncStep::Failed(
                    Some(captures),
                    loc,
                    Some(Arc::new(world.clone())),
                    SyncStepError::Panic(e.downcast_ref::<String>().cloned().unwrap()),
                ),
            }
        } else {
            SyncStep::Skipped
        };
        (world, ev)
    }

    async fn execute_scenario(
        scenario: gherkin::Scenario,
    ) -> impl Stream<Item = event::Feature<W>> {
        let s = scenario.clone();
        let (tx, rx) = futures::channel::oneshot::channel();

        thread::spawn(move || {
            let steps = block_on(async {
                let mut steps = Vec::with_capacity(s.steps.len());
                let mut world = W::new().await.unwrap();
                for step in s.steps.clone() {
                    let (w, ev) = Self::execute_step(world, step.clone()).await;
                    world = w;
                    let should_stop = matches!(ev, SyncStep::Failed(..));
                    steps.push((step, ev));
                    if should_stop {
                        break;
                    }
                }
                steps
            });
            tx.send(steps).unwrap();
        });

        let steps = rx.await.unwrap();
        let steps: Vec<(gherkin::Step, event::Step<W>)> = steps
            .into_iter()
            .map(|(step, ev)| (step, event::Step::from(ev)))
            .collect();

        let scenario = Arc::new(scenario);
        stream::once(future::ready(event::Scenario::Started))
            .chain(stream::iter(steps.into_iter().flat_map(|(step, ev)| {
                let step = Arc::new(step);
                [
                    event::Scenario::Step(step.clone(), event::Step::Started),
                    event::Scenario::Step(step, ev),
                ]
            })))
            .chain(stream::once(future::ready(event::Scenario::Finished)))
            .map(move |event| {
                event::Feature::Scenario(
                    scenario.clone(),
                    event::RetryableScenario {
                        event,
                        retries: None,
                    },
                )
            })
    }

    async fn execute_rule_scenario(
        scenario: gherkin::Scenario,
        background: Option<gherkin::Background>,
    ) -> impl Stream<Item = event::Rule<W>> {
        let scenario = Arc::new(scenario);
        let s = scenario.clone();
        let (tx, rx) = futures::channel::oneshot::channel();

        thread::spawn(move || {
            let steps = block_on(async {
                let mut steps = Vec::new();
                let mut world = W::new().await.unwrap();

                if let Some(background) = background {
                    for step in background.steps {
                        let (w, ev) = Self::execute_step(world, step.clone()).await;
                        world = w;
                        let should_stop = matches!(ev, SyncStep::Failed(..));
                        steps.push((step, ev));
                        if should_stop {
                            break;
                        }
                    }
                }

                for step in s.steps.clone() {
                    let (w, ev) = Self::execute_step(world, step.clone()).await;
                    world = w;
                    let should_stop = matches!(ev, SyncStep::Failed(..));
                    steps.push((step, ev));
                    if should_stop {
                        break;
                    }
                }
                steps
            });
            tx.send(steps).unwrap();
        });

        let steps = rx.await.unwrap();
        let steps: Vec<(gherkin::Step, event::Step<W>)> = steps
            .into_iter()
            .map(|(step, ev)| (step, event::Step::from(ev)))
            .collect();

        // let scenario = Arc::new(scenario);
        stream::once(future::ready(event::Scenario::Started))
            .chain(stream::iter(steps.into_iter().flat_map(|(step, ev)| {
                let step = Arc::new(step);
                [
                    event::Scenario::Step(step.clone(), event::Step::Started),
                    event::Scenario::Step(step, ev),
                ]
            })))
            .chain(stream::once(future::ready(event::Scenario::Finished)))
            .map(move |event| {
                event::Rule::Scenario(
                    scenario.clone(),
                    event::RetryableScenario {
                        event,
                        retries: None,
                    },
                )
            })
    }

    async fn execute_rule(rule: gherkin::Rule) -> impl Stream<Item = event::Feature<W>> {
        let rule = Arc::new(rule);
        let background = rule.background.clone();
        stream::once(future::ready(event::Rule::Started))
            .chain(
                stream::iter(rule.scenarios.clone())
                    .then(move |s| Self::execute_rule_scenario(s, background.clone()))
                    .flatten(),
            )
            .chain(stream::once(future::ready(event::Rule::Finished)))
            .map(move |ev| event::Feature::Rule(rule.clone(), ev))
    }

    fn execute_feature(feature: gherkin::Feature) -> impl Stream<Item = event::Cucumber<W>> {
        let feature = Arc::new(feature);
        stream::once(future::ready(event::Feature::Started))
            .chain(
                stream::iter(feature.scenarios.clone())
                    .then(Self::execute_scenario)
                    .flatten(),
            )
            .chain(
                stream::iter(feature.rules.clone())
                    .then(Self::execute_rule)
                    .flatten(),
            )
            .chain(stream::once(future::ready(event::Feature::Finished)))
            .map(move |ev| event::Cucumber::Feature(feature.clone(), ev))
    }
}

impl<W> cucumber::Runner<W> for SyncRunner<W>
where
    W: World + WorldInventory + Debug + Clone + Send + Sync,
    <W as World>::Error: Debug,
{
    type Cli = cli::Empty; // we provide no CLI options
    type EventStream = LocalBoxStream<'static, parser::Result<Event<event::Cucumber<W>>>>;

    fn run<S>(self, features: S, _: Self::Cli) -> Self::EventStream
    where
        S: Stream<Item = parser::Result<gherkin::Feature>> + 'static,
    {
        stream::once(future::ok(event::Cucumber::Started))
            .chain(
                features
                    .map_ok(|f| Self::execute_feature(f).map(Ok))
                    .try_flatten(),
            )
            .chain(stream::once(future::ok(event::Cucumber::Finished)))
            .map_ok(Event::new)
            .boxed_local()
    }
}

#[derive(Debug)]
pub enum SyncStep<World> {
    Started,
    Skipped,
    Passed(regex::CaptureLocations, Option<step::Location>),
    Failed(
        Option<regex::CaptureLocations>,
        Option<step::Location>,
        Option<Arc<World>>,
        SyncStepError,
    ),
}

#[derive(Clone, Debug, Display, From)]
pub enum SyncStepError {
    #[display(fmt = "Step doesn't match any function")]
    NotFound,
    #[display(fmt = "Step match is ambiguous: {}", _0)]
    AmbiguousMatch(step::AmbiguousMatchError),
    #[display(fmt = "Step panicked. Captured output: {}", _0)]
    Panic(String),
}

impl<W> From<SyncStep<W>> for cucumber::event::Step<W> {
    fn from(value: SyncStep<W>) -> Self {
        match value {
            SyncStep::Started => cucumber::event::Step::Started,
            SyncStep::Skipped => cucumber::event::Step::Skipped,
            SyncStep::Passed(capture_location, maybe_location) => {
                cucumber::event::Step::Passed(capture_location, maybe_location)
            }
            SyncStep::Failed(capture_location, maybe_location, world, err) => {
                cucumber::event::Step::Failed(capture_location, maybe_location, world, err.into())
            }
        }
    }
}

impl From<SyncStepError> for cucumber::event::StepError {
    fn from(value: SyncStepError) -> Self {
        match value {
            SyncStepError::NotFound => cucumber::event::StepError::NotFound,
            SyncStepError::AmbiguousMatch(err) => cucumber::event::StepError::AmbiguousMatch(err),
            SyncStepError::Panic(msg) => cucumber::event::StepError::Panic(Arc::new(msg)),
        }
    }
}
