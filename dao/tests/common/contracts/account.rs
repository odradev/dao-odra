
use odra::{types::Address, test_env};

use crate::{
    common::{
        params::{Account, Contract},
        DaoWorld,
    },
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn get_address(&self, account: &Account) -> Address {
        match account {
            Account::Owner => test_env::get_account(0),
            Account::Deployer => test_env::get_account(0),
            Account::Alice => test_env::get_account(1),
            Account::Bob => test_env::get_account(2),
            Account::Holder => test_env::get_account(3),
            Account::Any => test_env::get_account(4),
            Account::JobPoster => test_env::get_account(5),
            Account::ExternalWorker => test_env::get_account(6),
            Account::InternalWorker => test_env::get_account(7),
            Account::MultisigWallet => test_env::get_account(8),
            Account::VA(n) => test_env::get_account(8 + n),
            Account::Contract(contract) => self.get_contract_address(contract),
        }
    }

    pub fn get_contract_address(&self, contract: &Contract) -> Address {
        match contract {
            Contract::Admin => self.admin.address(),
            Contract::KycToken => self.kyc_token.address(),
            Contract::VaToken => self.va_token.address(),
            Contract::ReputationToken => self.reputation_token.address(),
            Contract::VariableRepository => self.variable_repository.address(),
            Contract::KycVoter => self.kyc_voter.address(),
            Contract::RepoVoter => self.repo_voter.address(),
            Contract::SlashingVoter => todo!(),
            Contract::SimpleVoter => todo!(),
            Contract::ReputationVoter => self.reputation_voter.address(),
            Contract::BidEscrow => todo!(),
            Contract::Onboarding => todo!(),
            Contract::CSPRRateProvider => self.rate_provider.address(),
        }
    }
}
