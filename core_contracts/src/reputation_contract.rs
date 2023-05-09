use odra::Variable;

/// A ReputationContract module storage definition.
#[odra::module]
pub struct ReputationContract {
    value: Variable<bool>,
}

/// Module entrypoints implementation.
#[odra::module]
impl ReputationContract {
    /// ReputationContract constructor.
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
}

#[cfg(test)]
mod tests {
    use super::ReputationContractDeployer;

    #[test]
    fn it_works() {
        let mut contract = ReputationContractDeployer::initial_settings();
        assert!(!contract.get());
        contract.set(true);
        assert!(contract.get());
    }
}
