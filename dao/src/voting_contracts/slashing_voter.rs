use odra::{
    contract_env::{caller, revert},
    types::{event::OdraEvent, Address, BlockTime, U512},
    Composer, Event, Instance, Mapping, OdraType, UnwrapOrRevert, Variable,
};

use crate::{
    bid_escrow::types::BidId,
    configuration::ConfigurationBuilder,
    modules::{refs::ContractRefsStorage, AccessControl},
    utils::Error,
    voting::{
        ballot::{Ballot, Choice},
        types::VotingId,
        voting_engine::{
            events::VotingCreatedInfo,
            voting_state_machine::{VotingResult, VotingStateMachine, VotingType},
            VotingEngine, VotingEngineComposer,
        },
    },
};

/// Slashing Voter contract uses [VotingEngine](VotingEngine) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Slashing Voter contract needs to have permissions to perform those actions.
///
/// For details see [SlashingVoterContractInterface](SlashingVoterContractInterface)
#[odra::module(skip_instance, events = [SlashingVotingCreated])]
pub struct SlashingVoterContract {
    refs: ContractRefsStorage,
    voting_engine: VotingEngine,
    tasks: Mapping<VotingId, SlashTask>,
    bid_escrows: Variable<Vec<Address>>,
    access_control: AccessControl,
}

impl Instance for SlashingVoterContract {
    fn instance(namespace: &str) -> Self {
        let refs = Composer::new(namespace, "refs").compose();
        let voting_engine = VotingEngineComposer::new(namespace, "voting_engine")
            .with_refs(&refs)
            .compose();

        Self {
            refs,
            voting_engine,
            tasks: Composer::new(namespace, "tasks").compose(),
            bid_escrows: Composer::new(namespace, "bid_escrows").compose(),
            access_control: Composer::new(namespace, "access_control").compose(),
        }
    }
}

#[odra::module]
impl SlashingVoterContract {
    delegate! {
        to self.voting_engine {
            pub fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            pub fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            pub fn get_voting(
                &self,
                voting_id: VotingId,
            ) -> Option<VotingStateMachine>;
            pub fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                voter: Address,
            ) -> Option<Ballot>;
        }

        to self.access_control {
            pub fn change_ownership(&mut self, owner: Address);
            pub fn add_to_whitelist(&mut self, address: Address);
            pub fn remove_from_whitelist(&mut self, address: Address);
            pub fn is_whitelisted(&self, address: Address) -> bool;
            pub fn get_owner(&self) -> Option<Address>;
        }

        to self.refs {
            pub fn variable_repository_address(&self) -> Address;
            pub fn reputation_token_address(&self) -> Address;
        }
    }

    #[odra(init)]
    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
    ) {
        self.refs
            .init(variable_repository, reputation_token, va_token);
        self.access_control.init(caller());
    }

    pub fn update_bid_escrow_list(&mut self, bid_escrows: Vec<Address>) {
        self.access_control.ensure_whitelisted();
        self.bid_escrows.set(bid_escrows);
    }

    pub fn create_voting(&mut self, address_to_slash: Address, slash_ratio: u32, stake: U512) {
        // TODO: constraints
        let current_reputation = self.refs.reputation_token().balance_of(address_to_slash);

        let voting_configuration = ConfigurationBuilder::new(
            self.refs.reputation_token().total_supply(),
            &self.refs.variable_repository().all_variables(),
        )
        .build();

        let creator = caller();
        let (info, _) = self
            .voting_engine
            .create_voting(creator, stake, voting_configuration);

        let task = SlashTask {
            subject: address_to_slash,
            ratio: slash_ratio,
            reputation_at_voting_creation: current_reputation,
        };
        self.tasks.set(&info.voting_id, task);

        SlashingVotingCreated::new(address_to_slash, slash_ratio, info).emit();
    }

    pub fn vote(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: U512,
    ) {
        // Check if the caller is not a subject for the voting.
        let task = self.tasks.get(&voting_id).unwrap_or_revert();
        if caller() == task.subject {
            revert(Error::SubjectOfSlashing);
        }
        self.voting_engine
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let summary = self.voting_engine.finish_voting(voting_id, voting_type);
        if summary.is_formal() && summary.result() == VotingResult::InFavor {
            self.slash(voting_id);
        }
    }

    pub fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter, voting_id);
    }
}

impl SlashingVoterContract {
    fn slash(&mut self, voting_id: VotingId) {
        let slash_task = self.tasks.get(&voting_id).unwrap_or_revert();

        // Burn VA token.
        self.refs.va_token().burn(slash_task.subject);

        let mut reputation = self.refs.reputation_token();
        // If partial slash only burn reputation.
        if slash_task.ratio != 1000 {
            let slash_amount = (slash_task.reputation_at_voting_creation
                * U512::from(slash_task.ratio))
                / U512::from(1000);
            reputation.burn(slash_task.subject, slash_amount);
            return;
        }

        // If full slash burn all reputation
        reputation.burn_all(slash_task.subject);

        // Load account stakes.
        let stakes = reputation.stakes_info(slash_task.subject);

        // Slash all open offers in bid escrows.
        let bid_escrows = self.bid_escrows.get().unwrap_or_default();
        for bid_escrow_address in bid_escrows {
            SlashableRef::at(bid_escrow_address).slash_all_active_job_offers(slash_task.subject);
        }

        // Slash all bids.
        for (bid_escrow_address, bid_id) in stakes.get_bids_stakes_origins() {
            SlashableRef::at(*bid_escrow_address).slash_bid(*bid_id);
        }

        // Slash subject in all voter contracts.
        for (contract_address, voting_id) in stakes.get_voting_stakes_origins() {
            SlashableRef::at(*contract_address).slash_voter(slash_task.subject, *voting_id);
        }
    }
}

#[odra::external_contract]
trait Slashable {
    fn slash_all_active_job_offers(&mut self, bidder: Address);
    fn slash_bid(&mut self, bid_id: BidId) -> bool;
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
}

#[derive(Debug, OdraType)]
pub struct SlashTask {
    pub subject: Address,
    pub ratio: u32,
    pub reputation_at_voting_creation: U512,
}

/// Informs slashing voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct SlashingVotingCreated {
    address_to_slash: Address,
    slash_ratio: u32,
    creator: Address,
    stake: Option<U512>,
    voting_id: VotingId,
    config_informal_quorum: u32,
    config_informal_voting_time: u64,
    config_formal_quorum: u32,
    config_formal_voting_time: u64,
    config_total_onboarded: U512,
    config_double_time_between_votings: bool,
    config_voting_clearness_delta: U512,
    config_time_between_informal_and_formal_voting: BlockTime,
}

impl SlashingVotingCreated {
    pub fn new(address_to_slash: Address, slash_ratio: u32, info: VotingCreatedInfo) -> Self {
        Self {
            address_to_slash,
            slash_ratio,
            creator: info.creator,
            stake: info.stake,
            voting_id: info.voting_id,
            config_informal_quorum: info.config_informal_quorum,
            config_informal_voting_time: info.config_informal_voting_time,
            config_formal_quorum: info.config_formal_quorum,
            config_formal_voting_time: info.config_formal_voting_time,
            config_total_onboarded: info.config_total_onboarded,
            config_double_time_between_votings: info.config_double_time_between_votings,
            config_voting_clearness_delta: info.config_voting_clearness_delta,
            config_time_between_informal_and_formal_voting: info
                .config_time_between_informal_and_formal_voting,
        }
    }
}

// TODO: Setup Composer, events
