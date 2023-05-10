use crate::ballot::{Ballot, Choice};
use crate::types::VotingId;
use crate::voting_engine::voting_state_machine::{VotingStateMachine, VotingType};
use configuration::configuration::Configuration;
use odra::contract_env::get_block_time;
use odra::types::{Address, U512};
use odra::{List, Mapping};

pub mod voting_state_machine;

/// Governance voting is a struct that voting_contracts can use to implement voting.
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
    voting_states: Mapping<VotingId, Option<VotingStateMachine>>,
    ballots: Mapping<(VotingId, VotingType, Address), Ballot>,
    voters: Mapping<(VotingId, VotingType), List<Address>>,
}

impl VotingEngine {
    /// Creates new informal [Voting].
    ///
    /// `contract_to_call`, `entry_point` and `runtime_args` parameters define an action that will be performed when formal voting passes.
    ///
    /// It collects configuration from [Variable Repo] and persists it, so they won't change during the voting process.
    ///
    /// Interacts with [Dao Ids Contract] to generate voting id.
    ///
    /// Depending on the configuration may [`cast`] the first vote.
    ///
    /// # Errors
    /// * [`Error::NotEnoughReputation`] when the creator does not have enough reputation to create a voting.
    /// * [`Error::NotOnboarded`] if the configuration requires the creator to be a VA but is not.
    ///
    /// [Voting]: VotingStateMachine
    /// [Variable Repo]: crate::variable_repository::VariableRepositoryContract
    /// [`Error::NotOnboarded`]: casper_dao_utils::Error::NotOnboarded
    /// [Dao Ids Contract]: crate::ids::DaoIdsContractInterface
    /// [`cast`]: Self::cast_ballot()
    pub fn create_voting(&mut self, creator: Address, stake: U512, configuration: Configuration) {
        // ) -> (VotingCreatedInfo, VotingStateMachine) {
        // RulesBuilder::new()
        //     .add_validation(CanCreateVoting::create(
        //         self.is_va(creator),
        //         configuration.only_va_can_create(),
        //     ))
        //     .build()
        //     .validate_generic_validations();
        //
        // let should_cast_first_vote = configuration.should_cast_first_vote();
        //
        // let voting_ids_address = configuration.voting_ids_address();
        // let voting_id = ids::get_next_voting_id(voting_ids_address);
        // let mut voting =
        //     VotingStateMachine::new(voting_id, get_block_time(), creator, configuration);
        //
        // let mut used_stake = None;
        // if should_cast_first_vote {
        //     self.cast_vote(
        //         creator,
        //         VotingType::Informal,
        //         Choice::InFavor,
        //         stake,
        //         &mut voting,
        //     );
        //     used_stake = Some(stake);
        // }
        //
        // let info = VotingCreatedInfo::new(
        //     creator,
        //     voting_id,
        //     used_stake,
        //     voting.voting_configuration(),
        // );
        // self.set_voting(voting.clone());
        // (info, voting)
    }
}
