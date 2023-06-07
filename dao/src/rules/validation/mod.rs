//! Groups validations.
mod is_user_kyced;

pub mod bid_escrow;
pub mod voting;

use crate::configuration::Configuration;
use crate::utils::Error;
use crate::voting::voting_engine::voting_state_machine::VotingStateMachine;
pub use is_user_kyced::IsUserKyced;

/// A generic validation.
pub trait Validation {
    /// Returns the result of validation.
    fn validate(&self) -> Result<(), Error>;
}

/// A validation in the voting context.
pub trait VotingValidation {
    /// Returns the result of validation.
    fn validate(
        &self,
        voting_state_machine: &VotingStateMachine,
        configuration: &Configuration,
    ) -> Result<(), Error>;
}
