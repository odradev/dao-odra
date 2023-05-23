use dao::voting_contracts::repo_voter::RepoVoterContractDeployer;
use dao::voting_contracts::simple_voter::SimpleVoterContractDeployer;
use dao::voting_contracts::{RepoVoterContractRef, SimpleVoterContractRef};
use dao::{
    core_contracts::{
        KycNftContractDeployer, KycNftContractRef, ReputationContractDeployer,
        ReputationContractRef, VaNftContractDeployer, VaNftContractRef,
        VariableRepositoryContractDeployer, VariableRepositoryContractRef,
    },
    utils_contracts::{
        CSPRRateProviderContractDeployer, CSPRRateProviderContractRef, DaoIdsContractDeployer,
    },
    voting_contracts::{
        admin::AdminContractDeployer, kyc_voter::KycVoterContractDeployer,
        reputation_voter::ReputationVoterContractDeployer, AdminContractRef, KycVoterContractRef,
        ReputationVoterContractRef,
    },
};
use odra::{
    test_env,
    types::{Bytes, OdraType},
};
use std::fmt::{Debug, Formatter};

use super::{contracts::cspr::VirtualBalances, params::Account};

// 1CSPR ~= 0.02924$
const DEFAULT_CSPR_USD_RATE: u64 = 34_000_000_000;

#[derive(cucumber::World)]
pub struct DaoWorld {
    pub virtual_balances: VirtualBalances,
    pub admin: AdminContractRef,
    pub variable_repository: VariableRepositoryContractRef,
    pub kyc_token: KycNftContractRef,
    pub va_token: VaNftContractRef,
    pub reputation_token: ReputationContractRef,
    pub rate_provider: CSPRRateProviderContractRef,
    pub reputation_voter: ReputationVoterContractRef,
    pub kyc_voter: KycVoterContractRef,
    pub repo_voter: RepoVoterContractRef,
    pub simple_voter: SimpleVoterContractRef,
}

impl DaoWorld {
    pub fn advance_time(&mut self, seconds: u64) {
        test_env::advance_block_time_by(seconds);
    }

    pub fn set_caller(&mut self, caller: &Account) {
        test_env::set_caller(self.get_address(caller));
    }

    // sets variable value
    pub fn set_variable(&mut self, name: String, value: Bytes) {
        self.variable_repository.update_at(name, value, None);
    }

    // gets variable value
    pub fn get_variable<T: OdraType>(&self, name: String) -> T {
        let bytes = self.variable_repository.get(name).unwrap();
        T::deserialize(bytes.as_slice()).unwrap()
    }
}

impl Default for DaoWorld {
    fn default() -> Self {
        let default_account = test_env::get_account(0);
        test_env::set_caller(default_account);

        // TODO: extract it using DAOWorld get_account.
        let multisig_wallet = test_env::get_account(8);
        let rate_provider = CSPRRateProviderContractDeployer::init(DEFAULT_CSPR_USD_RATE.into());
        let ids = DaoIdsContractDeployer::init();
        let variable_repository = VariableRepositoryContractDeployer::init(
            rate_provider.address(),
            multisig_wallet,
            ids.address(),
        );
        let reputation_token = ReputationContractDeployer::init();
        let kyc_token = KycNftContractDeployer::init(
            "kyc_token".to_string(),
            "KYC".to_string(),
            "".to_string(),
        );
        let va_token =
            VaNftContractDeployer::init("va_token".to_string(), "VAT".to_string(), "".to_string());
        let admin = AdminContractDeployer::init(
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );

        // Voters
        let reputation_voter = ReputationVoterContractDeployer::init(
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );
        let kyc_voter = KycVoterContractDeployer::init(
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
            kyc_token.address(),
        );
        let repo_voter = RepoVoterContractDeployer::init(
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );
        let simple_voter = SimpleVoterContractDeployer::init(
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );

        Self {
            virtual_balances: Default::default(),
            admin,
            variable_repository,
            kyc_token,
            va_token,
            reputation_token,
            rate_provider,
            reputation_voter,
            kyc_voter,
            repo_voter,
            simple_voter,
        }
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}
