use crate::ballot::Ballot;
use crate::refs::ContractRefsStorage;
use crate::types::VotingId;
use crate::voting_engine::voting_state_machine::{VotingStateMachine, VotingType};
use odra::types::Address;
use odra::{List, Mapping};

pub mod voting_state_machine;

/// Governance voting is a struct that contracts can use to implement voting.
///
/// It consists of two phases:
/// 1. Informal voting
/// 2. Formal voting
///
/// Whether formal voting starts depends on informal voting results.
///
/// When formal voting passes, an action can be performed - a contract can be called with voted arguments.
///
/// Governance voting uses:
/// 1. [Reputation Token](crate::reputation::ReputationContract) to handle reputation staking.
/// 2. [Variable Repo](crate::variable_repository::VariableRepositoryContract) for reading voting configuration.
///
/// For example implementation see [AdminContract](crate::admin::AdminContract).
pub struct VotingEngine {
    refs: ContractRefsStorage,
    voting_states: Mapping<VotingId, Option<VotingStateMachine>>,
    ballots: Mapping<(VotingId, VotingType, Address), Ballot>,
    voters: Mapping<(VotingId, VotingType), List<Address>>,
}
