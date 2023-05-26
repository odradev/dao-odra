use crate::configuration::Configuration;
use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::types::Balance;
use std::rc::Rc;

/// Makes sure the `Job DOS Fee` is high enough. May return [Error::DosFeeTooLow].
#[derive(Rule)]
pub struct IsDosFeeEnough {
    configuration: Rc<Configuration>,
    dos_fee: Balance,
}

impl Validation for IsDosFeeEnough {
    fn validate(&self) -> Result<(), Error> {
        // TODO: check attached value
        let fiat_value = self.configuration.convert_to_fiat(self.dos_fee)?;
        if self.configuration.is_post_job_dos_fee_too_low(fiat_value) {
            return Err(Error::DosFeeTooLow);
        };

        Ok(())
    }
}
