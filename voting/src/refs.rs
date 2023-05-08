use odra::types::Address;
use odra::Variable;

/// A module that stores addresses to common contracts that are used by most of the voting contracts.
pub struct ContractRefsStorage {
    variable_repository: Variable<Address>,
    reputation_token: Variable<Address>,
    va_token: Variable<Address>,
}
