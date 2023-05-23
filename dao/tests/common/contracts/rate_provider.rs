use crate::common::{params::Balance, DaoWorld};

impl DaoWorld {
    pub fn set_cspr_rate(&mut self, rate: Balance) {
        self.rate_provider.set_rate(rate.0);
    }

    pub fn get_cspr_rate(&self) -> Balance {
        Balance(self.rate_provider.get_rate())
    }
}
