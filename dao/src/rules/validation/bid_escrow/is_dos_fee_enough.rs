use std::rc::Rc;
use odra::types::U512;
use macros::Rule;
use crate::configuration::Configuration;
use crate::rules::validation::Validation;
use crate::utils::Error;

/// Makes sure the `Job DOS Fee` is high enough. May return [Error::DosFeeTooLow].
#[derive(Rule)]
pub struct IsDosFeeEnough {
    configuration: Rc<Configuration>,
    dos_fee: U512,
}

impl Validation for IsDosFeeEnough {
    fn validate(&self) -> Result<(), Error> {
        let fiat_value = self.configuration.convert_to_fiat(self.dos_fee)?;
        if self.configuration.is_post_job_dos_fee_too_low(fiat_value) {
            return Err(Error::DosFeeTooLow);
        };

        Ok(())
    }
}
