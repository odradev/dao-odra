//! Contains CSPR Rate Provider Contract definition and related abstractions.
use modules::Owner;
use odra::{
    contract_env,
    types::{Address, U512},
    Variable,
};

/// CSPR Rate provider contract allows to read and write the current CSPR:Fiat rate.
/// Only the owner is eligible to update the rate, but any account can read the current value.
#[odra::module]
pub struct CSPRRateProviderContract {
    owner: Owner,
    rate: Variable<U512>,
}

#[odra::module]
impl CSPRRateProviderContract {
    ///  Contract constructor.
    ///
    ///  * sets the initial CSPR:Fiat rate.
    ///  * sets the deployer as the owner.
    ///
    ///  [Read more](Owner::init())
    pub fn init(&mut self, rate: U512) {
        let deployer = contract_env::caller();
        self.owner.init(deployer);
        self.set_rate(rate);
    }

    /// Gets the current CSPR:Fiat rate.
    pub fn get_rate(&self) -> U512 {
        self.rate.get().unwrap_or_default()
    }

    /// Updates the current CSPR:Fiat rate.
    ///
    /// # Errors
    /// * [`NotAnOwner`](utils::errors::Error::NotAnOwner) if the caller is not the contract owner.
    pub fn set_rate(&mut self, rate: U512) {
        self.owner.ensure_owner();
        self.rate.set(rate);
    }

    /// Returns the address of the current owner.
    /// [`Read more`](Owner::get_owner()).
    pub fn get_owner(&self) -> Option<Address> {
        self.owner.get_owner()
    }
}
