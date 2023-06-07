use std::hash::Hash;

use crate::bid_escrow::bid::ShortenedBid;
use crate::modules::AccessControl;
use crate::utils::Error;
use crate::voting::ballot::ShortenedBallot;
use crate::voting::types::VotingId;
use odra::{
    contract_env,
    types::{event::OdraEvent, Address, Balance, OdraType},
    List, Mapping, UnwrapOrRevert,
};

use super::{
    balances::BalanceStorage,
    token::events::{Stake, Unstake},
};

use crate::bid_escrow::types::BidId;

/// A module that stores information about stakes.
#[odra::module(events = [Stake, Unstake])]
pub struct StakesStorage {
    total_stake: Mapping<Address, Balance>,
    bids: Mapping<Address, List<Option<(Address, BidId)>>>,
    votings: Mapping<Address, List<Option<(Address, VotingId)>>>,
    access_control: AccessControl,
    reputation_storage: BalanceStorage,
}

impl StakesStorage {
    /// Decreases the voter's stake and total stake.
    ///
    /// # Arguments
    ///
    /// * `ballot` - a short version of ballot that has been casted.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn stake_voting(&mut self, voting_id: VotingId, ballot: ShortenedBallot) {
        self.access_control.ensure_whitelisted();
        let ShortenedBallot { voter, stake } = ballot;

        self.assert_stake(stake);
        self.assert_balance(voter, stake);

        let voter_contract = contract_env::caller();
        self.inc_total_stake(voter, stake);
        self.votings
            .push_record(&voter, (voter_contract, voting_id));

        // TODO: Emit Stake event.
    }

    /// Decreases the voter's stake and total stake.
    ///
    /// # Arguments
    ///
    /// * `ballot`- a short version of ballot that has been casted.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn unstake_voting(&mut self, voting_id: VotingId, ballot: ShortenedBallot) {
        self.access_control.ensure_whitelisted();

        let voter_contract = contract_env::caller();
        // Decrement total stake.
        self.dec_total_stake(ballot.voter, ballot.stake);
        self.votings
            .remove_record(&ballot.voter, (voter_contract, voting_id));
    }

    /// Decreases all the voters' stake in voting with the given id, and their total stake.
    ///
    /// # Arguments
    ///
    /// * `voting_id` - the id of voting to unstake tokens.
    /// * `ballots`- a vector of short version of ballots that has been casted.
    ///
    /// # Errors
    ///)
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn bulk_unstake_voting(&mut self, voting_id: VotingId, ballots: Vec<ShortenedBallot>) {
        self.access_control.ensure_whitelisted();

        let voter_contract = contract_env::caller();

        for ballot in ballots {
            self.dec_total_stake(ballot.voter, ballot.stake);
            self.votings
                .remove_record(&ballot.voter, (voter_contract, voting_id));
        }
    }

    /// Increases the voter's stake and total stake.
    ///
    /// # Arguments
    ///
    /// * `bid` - a short version of the bid that has been offered.
    ///
    /// # Events
    ///
    /// # Errors
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn stake_bid(&mut self, bid: ShortenedBid) {
        self.access_control.ensure_whitelisted();
        let ShortenedBid {
            worker,
            reputation_stake,
            bid_id,
        } = bid;

        self.assert_balance(worker, reputation_stake);
        self.assert_stake(reputation_stake);

        let voter_contract = contract_env::caller();
        self.inc_total_stake(worker, reputation_stake);
        self.bids.push_record(&worker, (voter_contract, bid_id));

        Stake {
            worker: bid.worker,
            amount: bid.reputation_stake,
            bid_id: bid.bid_id,
        }
        .emit();
    }

    /// Decreases the bidder's stake and total stake.
    ///
    /// # Arguments
    ///
    /// * `bid` - the original bid that has been offered.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn unstake_bid(&mut self, bid: ShortenedBid) {
        self.access_control.ensure_whitelisted();

        let voter_contract = contract_env::caller();
        // Decrement total stake.
        self.dec_total_stake(bid.worker, bid.reputation_stake);
        self.bids
            .remove_record(&bid.worker, (voter_contract, bid.bid_id));

        Unstake {
            worker: bid.worker,
            amount: bid.reputation_stake,
            bid_id: bid.bid_id,
        }
        .emit();
    }

    // Decreases all the bidders stake and total stake.
    ///
    /// # Arguments
    ///
    /// * `bid` - the original bid that has been offered.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn bulk_unstake_bid(&mut self, bids: Vec<ShortenedBid>) {
        self.access_control.ensure_whitelisted();

        let voter_contract = contract_env::caller();

        for bid in bids {
            // Decrement total stake.
            self.dec_total_stake(bid.worker, bid.reputation_stake);
            self.bids
                .remove_record(&bid.worker, (voter_contract, bid.bid_id));
        }
    }

    /// Returns the total stake of the given account.
    pub fn get_stake(&self, address: Address) -> Balance {
        self.total_stake.get(&address).unwrap_or_default()
    }

    /// Returns all the bids placed by the given account.
    ///
    /// A returned vector is a tuple of (BidEscrow contract address, bid id).
    pub fn get_bids(&self, address: &Address) -> Vec<(Address, BidId)> {
        let bids = self.bids.get_instance(address);
        let mut result = vec![];
        for maybe_bid in bids.iter() {
            if let Some((address, bid_id)) = maybe_bid {
                result.push((address, bid_id));
            }
        }

        result
    }

    /// Returns all the voting the given account participated in.
    ///
    /// A returned vector is a tuple of (voting contract address, voting id).
    pub fn get_votings(&self, address: &Address) -> Vec<(Address, VotingId)> {
        let votings = self.votings.get_instance(address);
        let mut result = vec![];
        for maybe_voting in votings.iter() {
            if let Some((address, voting_id)) = maybe_voting {
                result.push((address, voting_id));
            }
        }

        result
    }
}

impl StakesStorage {
    fn assert_balance(&self, address: Address, stake: Balance) {
        let user_stake = self.get_stake(address);
        let available_balance = self
            .reputation_storage
            .balance_of(address)
            .saturating_sub(user_stake);

        if available_balance < stake {
            contract_env::revert(Error::InsufficientBalance)
        }
    }

    fn assert_stake(&self, stake: Balance) {
        if stake.is_zero() {
            contract_env::revert(Error::ZeroStake)
        }
    }

    fn inc_total_stake(&mut self, account: Address, amount: Balance) {
        let new_value = self.get_stake(account) + amount;
        self.total_stake.set(&account, new_value);
    }

    fn dec_total_stake(&mut self, account: Address, amount: Balance) {
        let new_value = self
            .get_stake(account)
            .checked_sub(amount)
            .unwrap_or_revert_with(Error::ZeroStake);
        self.total_stake.set(&account, new_value);
    }
}

trait UpdatableVec<K, R> {
    fn push_record(&mut self, key: &K, record: R);
    fn remove_record(&mut self, key: &K, record: R);
}

impl<Key> UpdatableVec<Key, (Address, u32)> for Mapping<Key, List<Option<(Address, u32)>>>
where
    Key: OdraType + Hash,
{
    fn push_record(&mut self, key: &Key, record: (Address, u32)) {
        let mut records = self.get_instance(key);
        records.push(Some(record));
    }

    fn remove_record(&mut self, key: &Key, record: (Address, u32)) {
        let mut records = self.get_instance(key);
        if let Some(position) = records.iter().position(|r| r == Some(record)) {
            records.replace(position as u32, None);
        }
    }
}
