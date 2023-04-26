use crate::repository::events::ValueUpdated;
use dao_utils::consts;
use dao_utils::errors::Error::{ActivationTimeInPast, KeyValueStorageError};
use odra::contract_env::{get_block_time, revert};
use odra::types::event::OdraEvent;
use odra::types::{Address, OdraType, U512};
use odra::{List, Mapping, UnwrapOrRevert};

/// A data struct stored in the repository.
///
/// The first value represents the current value.
///
/// The second value is an optional tuple consisting of the future value and its activation time.
pub type Record = (Vec<u8>, Option<(Vec<u8>, u64)>);

/// A module that stores the DAO configuration.
///
/// The modules stores key-value pairs and a set of keys.
/// The repository is initialized with the default values.
#[odra::module]
pub struct Repository {
    pub storage: Mapping<String, Record>,
    pub keys: List<String>,
}

#[odra::module]
impl Repository {
    #[odra(init)]
    pub fn init(
        &mut self,
        fiat_conversion: Address,
        bid_escrow_wallet: Address,
        voting_ids: Address,
    ) {
        let mut config = RepositoryDefaults::default();
        config.push(consts::FIAT_CONVERSION_RATE_ADDRESS, fiat_conversion);
        config.push(consts::BID_ESCROW_WALLET_ADDRESS, bid_escrow_wallet);
        config.push(consts::VOTING_IDS_ADDRESS, voting_ids);
        for (key, value) in config.items() {
            self.set(key, value);
        }
    }

    pub fn update_at(&mut self, key: String, value: Vec<u8>, activation_time: Option<u64>) {
        let now = get_block_time();
        let value_for_event = value.clone();
        let new_value: Record = match activation_time {
            // If no activation_time provided update the record to the value from argument.
            None => (value, None),

            // If activation_time is in the past, raise an error.
            Some(activation_time) if activation_time < now => revert(ActivationTimeInPast),

            // If activation time is in future.
            Some(activation_time) => {
                // Load the record.
                let (current_value, current_next_value) = self
                    .storage
                    .get(&key)
                    .unwrap_or_revert_with(KeyValueStorageError);
                match current_next_value {
                    // If current_next_value is not set, update it to the value from arguments.
                    None => (current_value, Some((value, activation_time))),

                    // If current_next_value is set, but it is in the past, make it a current
                    // value and set next_value to values from arguments.
                    Some((current_next_value, current_activation_time))
                        if current_activation_time < now =>
                    {
                        (current_next_value, Some((value, activation_time)))
                    }

                    // If current_next_value is set in future, update it.
                    Some(_) => (current_value, Some((value, activation_time))),
                }
            }
        };
        self.storage.set(&key, new_value);
        self.keys.push(key.clone());
        ValueUpdated {
            key,
            value: value_for_event,
            activation_time,
        }
        .emit();
    }

    pub fn get(&self, key: String) -> Option<Vec<u8>> {
        let (current, future) = self.storage.get(&key)?;
        let now = get_block_time();
        if let Some((value, activation_time)) = future {
            if now > activation_time {
                return Some(value);
            }
        }
        Some(current)
    }

    pub fn get_full_value(&self, key: String) -> Option<Record> {
        self.storage.get(&key)
    }

    fn set(&mut self, key: String, value: Vec<u8>) {
        self.update_at(key, value, None);
    }
}

struct RepositoryDefaults {
    pub items: Vec<(String, Vec<u8>)>,
}

impl RepositoryDefaults {
    pub fn push<T: OdraType>(&mut self, key: &str, value: T) {
        self.items.push((key.to_string(), value.as_bytes().unwrap()));
    }

    pub fn items(self) -> Vec<(String, Vec<u8>)> {
        self.items
    }
}

impl Default for RepositoryDefaults {
    fn default() -> Self {
        let mut items = RepositoryDefaults { items: vec![] };
        items.push(consts::POST_JOB_DOS_FEE, U512::from(10000));
        items.push(consts::INTERNAL_AUCTION_TIME, 604800000u64);
        items.push(consts::PUBLIC_AUCTION_TIME, 864000000u64);
        items.push(consts::DEFAULT_POLICING_RATE, U512::from(300));
        items.push(consts::REPUTATION_CONVERSION_RATE, U512::from(100));
        items.push(consts::FORUM_KYC_REQUIRED, true);
        items.push(consts::BID_ESCROW_INFORMAL_QUORUM_RATIO, U512::from(500));
        items.push(consts::BID_ESCROW_FORMAL_QUORUM_RATIO, U512::from(500));
        items.push(consts::INFORMAL_QUORUM_RATIO, U512::from(500));
        items.push(consts::FORMAL_QUORUM_RATIO, U512::from(500));
        items.push(consts::BID_ESCROW_INFORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::BID_ESCROW_FORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::INFORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::FORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::INFORMAL_STAKE_REPUTATION, true);
        items.push(consts::TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING, 86400000u64);
        items.push(consts::VA_BID_ACCEPTANCE_TIMEOUT, 172800000u64);
        items.push(consts::VA_CAN_BID_ON_PUBLIC_AUCTION, false);
        items.push(consts::DISTRIBUTE_PAYMENT_TO_NON_VOTERS, true);
        items.push(consts::DEFAULT_REPUTATION_SLASH, U512::from(100));
        items.push(consts::VOTING_CLEARNESS_DELTA, U512::from(8));
        items.push(
            consts::VOTING_START_AFTER_JOB_WORKER_SUBMISSION,
            259200000u64,
        );
        items.push(consts::BID_ESCROW_PAYMENT_RATIO, U512::from(100));
        items
    }
}

pub mod events {
    use odra::Event;

    /// Informs the repository value has been changed.
    #[derive(Event, PartialEq, Eq, Debug)]
    pub struct ValueUpdated {
        pub key: String,
        pub value: Vec<u8>,
        pub activation_time: Option<u64>,
    }
}