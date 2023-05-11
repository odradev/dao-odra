use odra::types::Address;
use macros::Rule;
use crate::rules::validation::Validation;
use crate::utils::Error;

/// Makes sure the job poster is the one who picks the [`Bid`](crate::bid_escrow::bid::Bid).
/// May return [Error::OnlyJobPosterCanPickABid].
#[derive(Rule)]
pub struct CanPickBid {
    address: Address,
    job_poster: Address,
}

impl Validation for CanPickBid {
    fn validate(&self) -> Result<(), Error> {
        if self.job_poster != self.address {
            return Err(Error::OnlyJobPosterCanPickABid);
        }
        Ok(())
    }
}
