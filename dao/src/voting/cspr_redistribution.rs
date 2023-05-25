use odra::contract_env::transfer_tokens;
use odra::types::{Address, Balance, U512};
use crate::configuration::Configuration;
use crate::modules::refs::ContractRefsWithKycStorage;
use crate::utils::withdraw;

/// Transfers CSPRs to all VAs'. Each VA gets the amount of CSPR proportionally to their reputation.
///
/// Interacts with [`Reputation Token Contract`](crate::reputation::ReputationContractInterface) to get balances information.
pub fn redistribute_cspr_to_all_vas(to_redistribute: Balance, refs: &ContractRefsWithKycStorage) {
    let all_balances = refs.reputation_token().all_balances();
    let total_supply = all_balances.total_supply();
    // TODO: Fix the math when Odra will support this or we unify Balances
    let to_redistribute = U512::from(to_redistribute.as_u128());
    for (address, balance) in all_balances.balances() {
        let amount = to_redistribute * balance / total_supply;
        // TODO: Also here
        let amount = Balance::from(amount.as_u128());
        transfer_tokens(*address, amount);
    }
}

/// Transfers some part of a given amount to `Bid Escrow Wallet` and returns the remaining amount.
///
/// See [`Configuration::bid_escrow_wallet_address()`](crate::config::Configuration::bid_escrow_wallet_address()).
pub fn redistribute_to_governance(amount: Balance, configuration: &Configuration) -> Balance {
    let governance_wallet: Address = configuration.bid_escrow_wallet_address();
    // TODO: Fix the math when Odra will support this or we unify Balances
    let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(amount.as_u128().into());
    // TODO: Also here
    transfer_tokens(governance_wallet, Balance::from(governance_wallet_payment.as_u128()));

    // TODO: Also here
    amount - Balance::from(governance_wallet_payment.as_u128())
}
