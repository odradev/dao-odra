//! Contains Reputation Token Contract definition and related abstractions.
//!
//! New reputation is minted as a result of engagement in the DAO - voting or doing jobs.

use odra::{
    types::{Address, U512},
    OdraType,
};
mod agg;
mod balances;
mod stakes;
pub mod token;

// #[cfg(feature = "test-support")]
// pub use token::ReputationContractTest;
// pub use token::{
//     add_event_schemas,
//     event_schemas,
//     events::*,
//     ReputationContract,
//     ReputationContractCaller,
//     ReputationContractInterface,
// };

// /// Building blocks of Reputation Token.
// pub mod submodules {
//     pub use super::{agg::*, balances::BalanceStorage, stakes::StakesStorage};
// }

// TODO: remove once BidEscrow module is done.
pub type BidId = u32;
pub type VotingId = u32;

// TODO: remove once BidEscrow module is done.
#[derive(OdraType)]
pub struct ShortenedBid {
    pub bid_id: BidId,
    pub reputation_stake: U512,
    pub worker: Address,
}

#[derive(OdraType)]
pub struct ShortenedBallot {
    /// The voter's address.
    pub voter: Address,
    /// Vote power.
    pub stake: U512,
}
