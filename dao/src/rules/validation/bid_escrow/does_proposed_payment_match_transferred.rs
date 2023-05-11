use odra::types::U512;
use macros::Rule;
use crate::rules::validation::Validation;
use crate::utils::Error;

/// Verifies if the actual payment matches the proposed payment. May return [Error::PurseBalanceMismatch].
#[derive(Rule)]
pub struct DoesProposedPaymentMatchTransferred {
    proposed_payment: U512,
    transferred: U512,
}

impl Validation for DoesProposedPaymentMatchTransferred {
    fn validate(&self) -> Result<(), Error> {
        if self.proposed_payment != self.transferred {
            return Err(Error::PurseBalanceMismatch);
        }

        Ok(())
    }
}
