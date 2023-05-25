use std::{collections::BTreeMap, slice::Iter};

use crate::bid_escrow::types::BidId;
use crate::voting::types::VotingId;
use odra::{
    types::{Address, Balance},
    OdraType,
};

use super::balances::BalanceStorage;
use super::stakes::StakesStorage;

/// A module that provides aggregated data about reputation tokens.
#[odra::module]
pub struct BalanceAggregates {
    reputation_storage: BalanceStorage,
    stakes_storage: StakesStorage,
}

impl BalanceAggregates {
    /// Gets balances of all the token holders.
    pub fn all_balances(&self) -> AggregatedBalance {
        let mut balances = BTreeMap::<Address, Balance>::new();
        self.reputation_storage.holders().for_each(|address| {
            balances.insert(address, self.reputation_storage.balance_of(address));
        });

        AggregatedBalance::new(balances, self.reputation_storage.total_supply())
    }

    /// Gets balances of the given account addresses.
    pub fn partial_balances(&self, addresses: Vec<Address>) -> AggregatedBalance {
        let mut balances = BTreeMap::<Address, Balance>::new();
        let mut partial_supply = Balance::zero();
        for address in addresses {
            let balance = self.reputation_storage.balance_of(address);
            balances.insert(address, balance);
            partial_supply += balance;
        }
        AggregatedBalance {
            balances,
            total_supply: partial_supply,
        }
    }

    /// Gets all the data about the given user stakes.
    pub fn stakes_info(&self, address: Address) -> AggregatedStake {
        let bids = self.stakes_storage.get_bids(&address);
        let votings = self.stakes_storage.get_votings(&address);
        AggregatedStake::new(address, votings, bids)
    }
}

/// Stores information about balances and the total supply.
#[derive(OdraType)]
pub struct AggregatedBalance {
    balances: BTreeMap<Address, Balance>,
    total_supply: Balance,
}

impl AggregatedBalance {
    pub fn new(balances: BTreeMap<Address, Balance>, total_supply: Balance) -> Self {
        Self {
            balances,
            total_supply,
        }
    }

    pub fn balances(&self) -> &BTreeMap<Address, Balance> {
        &self.balances
    }

    pub fn total_supply(&self) -> Balance {
        self.total_supply
    }
}

/// Stores information about the user's voting and bids.
#[derive(OdraType)]
pub struct AggregatedStake {
    voter: Address,
    stakes_from_voting: Vec<(Address, VotingId)>,
    stakes_from_bid: Vec<(Address, BidId)>,
}

impl AggregatedStake {
    pub fn new(
        voter: Address,
        stakes_from_voting: Vec<(Address, VotingId)>,
        stakes_from_bid: Vec<(Address, BidId)>,
    ) -> Self {
        Self {
            voter,
            stakes_from_voting,
            stakes_from_bid,
        }
    }

    pub fn get_voting_stakes_origins(&self) -> Iter<(Address, VotingId)> {
        self.stakes_from_voting.iter()
    }

    pub fn get_bids_stakes_origins(&self) -> Iter<(Address, BidId)> {
        self.stakes_from_bid.iter()
    }
}
