use odra::OdraType;
use odra::types::{Address, Type, Typed, U512};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::voting_state_machine::VotingType;

/// Represents user's vote.
#[derive(OdraType)]
pub struct Ballot {
    /// The voter's address.
    pub voter: Address,
    /// A unique voting id.
    pub voting_id: VotingId,
    /// Voting type.
    pub voting_type: VotingType,
    /// Selected option.
    pub choice: Choice,
    /// Vote power.
    pub stake: U512,
    /// Indicates if the vote counts in the total voting stake.
    pub unbound: bool,
    /// Indicates if it reverts the previous ballot casted by the voter.
    pub canceled: bool,
}

/// Choice enum, can be converted to bool using `is_in_favor()`
#[derive(OdraType, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    /// `No` vote.
    Against,
    /// `Yes` vote.
    InFavor,
}

impl Typed for Choice {
    fn ty() -> Type {
        Type::Any
    }
}