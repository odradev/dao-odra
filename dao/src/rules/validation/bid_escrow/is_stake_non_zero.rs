use odra::types::U512;
use macros::Rule;
use crate::rules::validation::Validation;
use crate::utils::Error;

/// Makes sure the stake is non-zero. May return [Error::ZeroStake].
#[derive(Rule)]
pub struct IsStakeNonZero {
    reputation_stake: U512,
    cspr_stake: Option<U512>,
}

impl Validation for IsStakeNonZero {
    fn validate(&self) -> Result<(), Error> {
        if self.cspr_stake.is_none() && self.reputation_stake.is_zero() {
            return Err(Error::ZeroStake);
        }
        Ok(())
    }
}
