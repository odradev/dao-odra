use odra::types::{Address, Balance, CallArgs};
use odra::{call_contract, OdraType};

/// A serializable data structure that represent a contract call.
#[derive(OdraType)]
pub struct ContractCall {
    pub address: Address,
    pub entry_point: String,
    pub call_args: CallArgs,
    pub amount: Option<Balance>,
}

impl ContractCall {
    /// Get the contract call's address' reference.
    pub fn address(&self) -> &Address {
        &self.address
    }

    /// Get the contract call's entry point.
    pub fn entry_point(&self) -> String {
        self.entry_point.clone()
    }

    /// Get a reference to the contract call's runtime args.
    pub fn call_args(&self) -> &CallArgs {
        &self.call_args
    }

    /// Get a reference to the contract call's amount.
    pub fn amount(&self) -> Option<Balance> {
        self.amount
    }

    /// Calls the contract.
    pub fn call(&self) {
        call_contract(
            self.address().clone(),
            self.entry_point().as_str(),
            self.call_args(),
            self.amount(),
        )
    }
}
