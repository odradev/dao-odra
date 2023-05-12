use crate::configuration::Configuration;
use crate::rules::validation::voting::{AfterFormalVoting, VoteInTime, VotingNotCompleted};
use crate::rules::RulesBuilder;
use crate::utils::ContractCall;
use crate::voting::ballot::Choice;
use crate::voting::types::VotingId;
use odra::types::{Address, BlockTime, Type, Typed, U512};
use odra::OdraType;

/// Serializable voting state with a state machine capabilities.
///
/// Stores voting metadata, the configuration and the voting progress (stakes).
#[derive(OdraType)]
pub struct VotingStateMachine {
    voting_id: VotingId,
    state: VotingState,
    voting_type: VotingType,
    informal_stats: Stats,
    formal_stats: Stats,
    created_at: u64,
    creator: Address,
    configuration: Configuration,
}

impl VotingStateMachine {
    /// Creates new Voting with immutable [`Configuration`](Configuration).
    pub fn new(
        voting_id: VotingId,
        created_at: u64,
        creator: Address,
        configuration: Configuration,
    ) -> Self {
        VotingStateMachine {
            voting_id,
            state: VotingState::Created,
            voting_type: VotingType::Informal,
            informal_stats: Default::default(),
            formal_stats: Default::default(),
            created_at,
            creator,
            configuration,
        }
    }

    /// Ends the informal phase, verifies if the result is close, updates voting type to [Formal](VotingType::Formal).
    pub fn complete_informal_voting(&mut self) {
        self.state = VotingState::Formal;
        if self.is_result_close() {
            self.configuration.double_time_between_votings();
        }
        self.voting_type = VotingType::Formal;
    }

    /// Ends the voting process gracefully.
    pub fn finish(&mut self) {
        self.state = VotingState::Finished;
    }

    /// Ends the voting process forcefully and cancels the result.
    pub fn cancel(&mut self) {
        self.state = VotingState::Canceled;
    }

    /// Returns the type of Voting.
    pub fn voting_type(&self) -> VotingType {
        self.voting_type
    }

    /// Checks if Voting is of type [Informal](VotingType::Informal) and stakes the reputation.
    pub fn is_informal_without_stake(&self) -> bool {
        !self.voting_configuration().informal_stake_reputation()
            && self.voting_type() == VotingType::Informal
    }

    /// Checks if Voting is still in the voting phase.
    pub fn is_in_time(&self, block_time: u64) -> bool {
        match self.voting_type() {
            VotingType::Informal => {
                let start_time = self.informal_voting_start_time();
                let voting_time = self.configuration.informal_voting_time();
                start_time + voting_time <= block_time
            }
            VotingType::Formal => {
                self.informal_voting_start_time() + self.configuration.formal_voting_time()
                    <= block_time
            }
        }
    }

    /// Gets the informal phase end time.
    pub fn informal_voting_end_time(&self) -> BlockTime {
        self.informal_voting_start_time() + self.configuration.informal_voting_time()
    }

    /// Gets the informal-formal break end time.
    pub fn time_between_votings_end_time(&self) -> BlockTime {
        self.informal_voting_end_time()
            + self.configuration.time_between_informal_and_formal_voting()
    }

    /// Gets the informal phase end time.
    pub fn formal_voting_end_time(&self) -> BlockTime {
        self.time_between_votings_end_time() + self.configuration.formal_voting_time()
    }

    /// Checks is the `in_favor` stake surpasses the `against` stake.
    pub fn is_in_favor(&self) -> bool {
        match self.voting_type() {
            VotingType::Informal => {
                self.informal_stats.stake_in_favor >= self.informal_stats.stake_against
            }
            VotingType::Formal => {
                self.formal_stats.stake_in_favor >= self.formal_stats.stake_against
            }
        }
    }

    /// Depending on the result of the voting, returns the amount of reputation staked on the winning side.
    pub fn get_winning_stake(&self) -> U512 {
        match (self.voting_type(), self.is_in_favor()) {
            (VotingType::Informal, true) => self.informal_stats.stake_in_favor,
            (VotingType::Informal, false) => self.informal_stats.stake_against,
            (VotingType::Formal, true) => self.formal_stats.stake_in_favor,
            (VotingType::Formal, false) => self.formal_stats.stake_against,
        }
    }

    /// Gets the current voting result.
    pub fn get_result(&self, voters_number: u32) -> VotingResult {
        if self.get_quorum() > voters_number {
            VotingResult::QuorumNotReached
        } else if self.is_in_favor() {
            VotingResult::InFavor
        } else {
            VotingResult::Against
        }
    }

    /// Adds the `stake` to the total bound stake.
    pub fn add_stake(&mut self, stake: U512, choice: Choice) {
        // overflow is not possible due to reputation token having U512 as max
        match (self.voting_type(), choice) {
            (VotingType::Informal, Choice::InFavor) => self.informal_stats.stake_in_favor += stake,
            (VotingType::Informal, Choice::Against) => self.informal_stats.stake_against += stake,
            (VotingType::Formal, Choice::InFavor) => self.formal_stats.stake_in_favor += stake,
            (VotingType::Formal, Choice::Against) => self.formal_stats.stake_against += stake,
        }
    }

    /// Adds the `stake` to the total unbound stake.
    pub fn add_unbound_stake(&mut self, stake: U512, choice: Choice) {
        // overflow is not possible due to reputation token having U512 as max
        match (self.voting_type(), choice) {
            (VotingType::Informal, Choice::InFavor) => {
                self.informal_stats.unbound_stake_in_favor += stake
            }
            (VotingType::Informal, Choice::Against) => {
                self.informal_stats.unbound_stake_against += stake
            }
            (VotingType::Formal, Choice::InFavor) => {
                self.formal_stats.unbound_stake_in_favor += stake
            }
            (VotingType::Formal, Choice::Against) => {
                self.formal_stats.unbound_stake_against += stake
            }
        }
    }

    /// Removes the `stake` from the total bound stake.
    pub fn remove_stake(&mut self, stake: U512, choice: Choice) {
        // overflow is not possible due to reputation token having U512 as max
        match (self.voting_type(), choice) {
            (VotingType::Informal, Choice::InFavor) => self.informal_stats.stake_in_favor -= stake,
            (VotingType::Informal, Choice::Against) => self.informal_stats.stake_against -= stake,
            (VotingType::Formal, Choice::InFavor) => self.formal_stats.stake_in_favor -= stake,
            (VotingType::Formal, Choice::Against) => self.formal_stats.stake_against -= stake,
        }
    }

    /// Removes the `stake` from the total unbound stake.
    pub fn remove_unbound_stake(&mut self, stake: U512, choice: Choice) {
        // overflow is not possible due to reputation token having U512 as max
        match (self.voting_type(), choice) {
            (VotingType::Informal, Choice::InFavor) => {
                self.informal_stats.unbound_stake_in_favor -= stake
            }
            (VotingType::Informal, Choice::Against) => {
                self.informal_stats.unbound_stake_against -= stake
            }
            (VotingType::Formal, Choice::InFavor) => {
                self.formal_stats.unbound_stake_in_favor -= stake
            }
            (VotingType::Formal, Choice::Against) => {
                self.formal_stats.unbound_stake_against -= stake
            }
        }
    }

    /// Removes the unbound stake and adds it to the bound stake.
    pub fn bind_stake(&mut self, stake: U512, choice: Choice) {
        self.remove_unbound_stake(stake, choice);
        self.add_stake(stake, choice);
    }

    /// Gets the sum of bound and unbound stake.
    pub fn total_stake(&self) -> U512 {
        // overflow is not possible due to reputation token having U512 as max
        self.total_bound_stake() + self.total_unbound_stake()
    }

    /// Gets the total bound stake.
    pub fn total_bound_stake(&self) -> U512 {
        // overflow is not possible due to reputation token having U512 as max
        match self.voting_type() {
            VotingType::Informal => {
                self.informal_stats.stake_in_favor + self.informal_stats.stake_against
            }
            VotingType::Formal => {
                self.formal_stats.stake_in_favor + self.formal_stats.stake_against
            }
        }
    }

    /// Gets the total unbound stake.
    pub fn total_unbound_stake(&self) -> U512 {
        // overflow is not possible due to reputation token having U512 as max
        match self.voting_type() {
            VotingType::Informal => {
                self.informal_stats.unbound_stake_in_favor
                    + self.informal_stats.unbound_stake_against
            }
            VotingType::Formal => {
                self.formal_stats.unbound_stake_in_favor + self.formal_stats.unbound_stake_against
            }
        }
    }

    /// Get the voting's voting id.
    pub fn voting_id(&self) -> VotingId {
        self.voting_id
    }

    /// Get the voting's stake in favor.
    pub fn stake_in_favor(&self) -> U512 {
        match self.voting_type() {
            VotingType::Informal => self.informal_stats.stake_in_favor,
            VotingType::Formal => self.formal_stats.stake_in_favor,
        }
    }

    /// Get the voting's stake against.
    pub fn stake_against(&self) -> U512 {
        match self.voting_type() {
            VotingType::Informal => self.informal_stats.stake_against,
            VotingType::Formal => self.formal_stats.stake_against,
        }
    }

    /// Gets the voting's contract call reference.
    pub fn contract_calls(&self) -> &Vec<ContractCall> {
        self.configuration.contract_calls()
    }

    /// Gets a reference to the voting's voting configuration.
    pub fn voting_configuration(&self) -> &Configuration {
        &self.configuration
    }

    /// Gets the voting creator.
    pub fn creator(&self) -> &Address {
        &self.creator
    }

    /// Gets the current voting state.
    pub fn state(&self) -> &VotingState {
        &self.state
    }

    /// Indicates if Voting is finished or canceled.
    pub fn completed(&self) -> bool {
        self.state() == &VotingState::Finished || self.state() == &VotingState::Canceled
    }

    /// Returns the voting state depending on a given `block_time`.
    pub fn state_in_time(&self, block_time: BlockTime) -> VotingState {
        let informal_voting_start = self.informal_voting_start_time();
        let informal_voting_end = self.informal_voting_end_time();
        let between_voting_end = self.time_between_votings_end_time();
        let voting_end = self.formal_voting_end_time();

        if block_time < informal_voting_start {
            VotingState::Created
        } else if block_time >= informal_voting_start && block_time <= informal_voting_end {
            VotingState::Informal
        } else if block_time > informal_voting_end && block_time <= between_voting_end {
            VotingState::BetweenVotings
        } else if block_time > between_voting_end && block_time <= voting_end {
            VotingState::Formal
        } else {
            VotingState::Finished
        }
    }

    /// Gets the `Informal Voting` statistics.
    pub fn informal_stats(&self) -> &Stats {
        &self.informal_stats
    }

    /// Gets the `Formal Voting` statistics.
    pub fn formal_stats(&self) -> &Stats {
        &self.formal_stats
    }

    fn informal_voting_start_time(&self) -> u64 {
        self.created_at() + self.configuration.voting_delay()
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    fn is_result_close(&self) -> bool {
        let stake_in_favor = self.stake_in_favor() + self.unbound_stake_in_favor();
        let stake_against = self.stake_against() + self.unbound_stake_against();
        let stake_diff = stake_in_favor.abs_diff(stake_against);
        let stake_diff_percent = stake_diff.saturating_mul(U512::from(100)) / self.total_stake();
        stake_diff_percent <= self.configuration.voting_clearness_delta()
    }

    fn get_quorum(&self) -> u32 {
        match self.voting_type() {
            VotingType::Informal => self.configuration.informal_voting_quorum(),
            VotingType::Formal => self.configuration.formal_voting_quorum(),
        }
    }

    fn unbound_stake_in_favor(&self) -> U512 {
        match self.voting_type() {
            VotingType::Informal => self.informal_stats.unbound_stake_in_favor,
            VotingType::Formal => self.formal_stats.unbound_stake_in_favor,
        }
    }

    fn unbound_stake_against(&self) -> U512 {
        match self.voting_type() {
            VotingType::Informal => self.informal_stats.unbound_stake_against,
            VotingType::Formal => self.formal_stats.unbound_stake_against,
        }
    }

    /// Verifies if a ballot can be casted.
    ///
    /// Stops contract execution if validation fails. See [`VoteInTime`].
    pub fn guard_vote(&self, block_time: BlockTime) {
        RulesBuilder::new()
            .add_voting_validation(VoteInTime::create(block_time))
            .build()
            .validate(self);
    }

    /// Verifies if the formal voting can be finished.
    ///
    /// Stops contract execution if validation fails. See [`AfterFormalVoting`] and [`VotingNotCompleted`].
    pub fn guard_finish_formal_voting(&self, block_time: BlockTime) {
        RulesBuilder::new()
            .add_voting_validation(AfterFormalVoting::create(block_time))
            .add_voting_validation(VotingNotCompleted::create())
            .build()
            .validate(self);
    }
}

/// Voting statistics.
#[derive(OdraType, Default)]
pub struct Stats {
    /// The total `in favor` stake.
    pub stake_in_favor: U512,
    /// The total `against` stake.
    pub stake_against: U512,
    /// The total unbounded `in favor` stake.
    pub unbound_stake_in_favor: U512,
    /// The total unbounded `against` stake.
    pub unbound_stake_against: U512,
    /// The number of VA's voted `in favor`.
    pub votes_in_favor: u32,
    /// The number of VA's voted `against`.
    pub votes_against: u32,
}

/// State of Voting.
#[derive(OdraType, PartialEq, Eq)]
pub enum VotingState {
    /// Voting created but informal voting is not started.
    Created,
    /// Informal voting started.
    Informal,
    /// Informal voting ended, but the formal one hasn't started yet.
    BetweenVotings,
    /// Formal voting started.
    Formal,
    /// Formal voting ended.
    Finished,
    /// The voting interrupted.
    Canceled,
}

/// Type of Voting (Formal or Informal).
#[derive(OdraType, Copy, Hash, PartialEq, Eq, Debug)]
pub enum VotingType {
    Informal,
    Formal,
}

impl Typed for VotingType {
    fn ty() -> Type {
        Type::Any
    }
}

/// The Voting process progression.
#[derive(OdraType)]
pub enum VotingStateInTime {
    BeforeInformal,
    Informal,
    BetweenVotings,
    Formal,
    AfterFormal,
}

/// Serializable finished Voting summary.
#[allow(dead_code)]
#[derive(OdraType)]
pub struct VotingSummary {
    result: VotingResult,
    ty: VotingType,
    voting_id: VotingId,
}

impl VotingSummary {
    /// Creates a new instance of [`VotingSummary`].
    pub fn new(result: VotingResult, ty: VotingType, voting_id: VotingId) -> Self {
        Self {
            result,
            ty,
            voting_id,
        }
    }

    /// Indicates if the voting process is completed.
    pub fn is_voting_process_finished(&self) -> bool {
        match self.ty {
            VotingType::Informal => self.is_rejected(),
            VotingType::Formal => true,
        }
    }

    /// Indicates if summary refers to formal voting .
    pub fn is_formal(&self) -> bool {
        self.voting_type() == VotingType::Formal
    }

    /// Returns voting result.
    pub fn result(&self) -> VotingResult {
        self.result.clone()
    }

    /// Gets the voting type.
    pub fn voting_type(&self) -> VotingType {
        self.ty
    }

    fn is_rejected(&self) -> bool {
        vec![VotingResult::Against, VotingResult::QuorumNotReached].contains(&self.result)
    }
}

/// Result of Voting.
#[derive(OdraType, PartialEq, Eq, Debug)]
pub enum VotingResult {
    /// Voting passed.
    InFavor,
    /// Voting rejected.
    Against,
    /// Too few VA's voted.
    QuorumNotReached,
    /// Voting canceled.
    Canceled,
}

impl Typed for VotingResult {
    fn ty() -> Type {
        Type::Any
    }
}
