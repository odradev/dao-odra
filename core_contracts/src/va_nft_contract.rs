use odra::types::U512;
use odra::Variable;

/// A VaNftContract module storage definition.
#[odra::module]
pub struct VaNftContract {
    value: Variable<bool>,
}

/// Module entrypoints implementation.
#[odra::module]
impl VaNftContract {
    /// VaNftContract constructor.
    /// Initializes the contract with the value of value.
    #[odra(init)]
    pub fn initial_settings(&mut self) {
        self.value.set(false);
    }

    /// Replaces the current value with the passed argument.
    pub fn set(&mut self, value: bool) {
        self.value.set(value);
    }

    /// Retrieves value from the storage.
    /// If the value has never been set, the default value is returned.
    pub fn get(&self) -> bool {
        self.value.get_or_default()
    }

    pub fn total_supply(&self) -> U512 {
        U512::from(0)
    }
}

#[cfg(test)]
mod tests {
    use super::VaNftContractDeployer;

    #[test]
    fn it_works() {
        let mut contract = VaNftContractDeployer::initial_settings();
        assert!(!contract.get());
        contract.set(true);
        assert!(contract.get());
    }
}
