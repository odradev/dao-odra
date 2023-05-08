use crate::configuration::VotingConfiguration;
use crate::types::VotingId;
use odra::types::{Address, U512};

/// Serializable voting state with a state machine capabilities.
///
/// Stores voting metadata, the configuration and the voting progress (stakes).
pub struct VotingStateMachine {
    voting_id: VotingId,
    state: VotingState,
    voting_type: VotingType,
    informal_stats: Stats,
    formal_stats: Stats,
    created_at: u64,
    creator: Address,
    configuration: VotingConfiguration,
}

/// Voting statistics.
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
pub enum VotingType {
    Informal,
    Formal,
}
