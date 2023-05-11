use odra::types::U512;
use macros::Rule;
use crate::rules::validation::Validation;
use crate::utils::Error;

/// Verifies if the proposed payment does not exceeds the budget.
/// May return [Error::PaymentExceedsMaxBudget].
#[derive(Rule)]
pub struct DoesProposedPaymentExceedBudget {
    proposed_payment: U512,
    max_budget: U512,
}

impl Validation for DoesProposedPaymentExceedBudget {
    fn validate(&self) -> Result<(), Error> {
        if self.proposed_payment > self.max_budget {
            return Err(Error::PaymentExceedsMaxBudget);
        }

        Ok(())
    }
}
