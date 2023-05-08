//! Utility modules providing references to common contracts that are used by most of the voting contracts.
use odra::types::Address;
use odra::Variable;

use crate::{VaNftContractRef, VariableRepositoryRef};

/// Provides references to common contracts that are used by most of the voting contracts.
pub trait ContractRefs {
    /// Returns a reference to [Reputation Token](crate::reputation::ReputationContract) connected to the contract
    fn reputation_token(&self) -> ReputationContractRef;
    /// Returns a reference to [Variable Repository](crate::variable_repository::VariableRepositoryContract) connected to the contract
    fn variable_repository(&self) -> VariableRepositoryRef;
    /// Returns a reference to [VA Token](crate::va_nft::VaNftContract) connected to the contract
    fn va_token(&self) -> VaNftContractRef;
}

/// A module that stores addresses to common voting_contracts that are used by most of the voting voting_contracts.
pub struct ContractRefsStorage {
    variable_repository: Variable<Address>,
    reputation_token: Variable<Address>,
    va_token: Variable<Address>,
}
