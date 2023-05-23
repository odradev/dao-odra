use crate::bid_escrow::events::{CSPRTransfer, TransferReason};
use crate::utils::Error;
use odra::contract_env::{revert, self_address, transfer_tokens};
use odra::types::{event, event::OdraEvent, Address, Balance};

pub fn withdraw(to: Address, amount: Balance, reason: TransferReason) {
    // TODO: withdraw
    if !transfer_tokens(to, amount) {
        revert(Error::TransferError);
    }

    CSPRTransfer {
        from: self_address(),
        to,
        amount,
        reason: reason.to_string(),
    }
    .emit();
}
