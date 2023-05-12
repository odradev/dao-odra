//! Utility modules providing references to common contracts that are used by most of the voting contracts.
use odra::types::Address;
use odra::{UnwrapOrRevert, Variable};

use crate::core_contracts::{ReputationContractRef, VaNftContractRef, VariableRepositoryRef};
use crate::utils::Error;

/// A module that stores addresses to common voting_contracts that are used by most of the voting voting_contracts.
#[odra::module]
pub struct ContractRefsStorage {
    variable_repository: Variable<Address>,
    reputation_token: Variable<Address>,
    va_token: Variable<Address>,
}

#[odra::module]
impl ContractRefsStorage {
    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
    ) {
        self.variable_repository.set(variable_repository);
        self.reputation_token.set(reputation_token);
        self.va_token.set(va_token);
    }

    /// Returns the address of [Reputation Token](crate::core_contracts::ReputationContract) contract.
    pub fn reputation_token_address(&self) -> Address {
        self.reputation_token
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    /// Returns the address of [Variable Repository](crate::core_contracts::VariableRepository) contract.
    pub fn variable_repository_address(&self) -> Address {
        self.variable_repository
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    /// Returns the address of [VA Token](crate::core_contracts::VaNftContract) contract.
    pub fn va_token_address(&self) -> Address {
        self.va_token
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }
}

impl ContractRefsStorage {
        /// Returns the Ref of [Reputation Token](crate::core_contracts::ReputationContract) contract.
    pub fn reputation_token(&self) -> ReputationContractRef {
        ReputationContractRef::at(
            self.reputation_token
                .get()
                .unwrap_or_revert_with(Error::VariableValueNotSet),
        )
    }

    /// Returns the Ref of [Variable Repository](crate::core_contracts::VariableRepository) contract.
    pub fn variable_repository(&self) -> VariableRepositoryRef {
        VariableRepositoryRef::at(
            self.variable_repository
                .get()
                .unwrap_or_revert_with(Error::VariableValueNotSet),
        )
    }

    /// Returns the Ref of [VA Token](crate::core_contracts::VaNftContract) contract.
    pub fn va_token(&self) -> VaNftContractRef {
        VaNftContractRef::at(
            self.va_token
                .get()
                .unwrap_or_revert_with(Error::VariableValueNotSet),
        )
    }
}