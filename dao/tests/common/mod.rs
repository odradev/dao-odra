use dao::{core_contracts::{VariableRepositoryDeployer, VariableRepositoryRef, KycNftContractRef, VaNftContractRef, KycNftContractDeployer, VaNftContractDeployer, ReputationContractRef, ReputationContractDeployer}, utils_contracts::{CSPRRateProviderContractRef, CSPRRateProviderContractDeployer}, voting_contracts::{AdminContractRef, admin::AdminContractDeployer}};
use odra::{test_env, types::{Address, OdraType}};
use std::fmt::{Debug, Formatter};

use self::{params::Account, contracts::cspr::VirtualBalances};

// 1CSPR ~= 0.02924$
const DEFAULT_CSPR_USD_RATE: u64 = 34_000_000_000;

#[derive(cucumber::World)]
pub struct DaoWorld {
    admin: AdminContractRef,
    variable_repository: VariableRepositoryRef,
    kyc_token: KycNftContractRef,
    va_token: VaNftContractRef,
    reputation_token: ReputationContractRef,
    rate_provider: CSPRRateProviderContractRef,
    virtual_balances: VirtualBalances
}

impl DaoWorld {
    pub fn set_caller(&mut self, caller: &Account) {
        test_env::set_caller(self.get_address(caller));
    }

    // gets variable value
    pub fn get_variable<T: OdraType>(&self, name: String) -> T {
        let bytes = self.variable_repository.get(name).unwrap();
        T::deserialize(bytes.as_slice()).unwrap()
    }

    pub fn variable_repository_address(&self) -> Address {
        self.variable_repository.address()
    }

    pub fn kyc_token_address(&self) -> Address {
        self.kyc_token.address()
    }

    pub fn va_token_address(&self) -> Address {
        self.va_token.address()
    }

    pub fn reputation_token_address(&self) -> Address {
        self.reputation_token.address()
    }
}

impl Default for DaoWorld {
    fn default() -> Self {
        let default_account = test_env::get_account(0);
        test_env::set_caller(default_account);
        Self {
            admin: AdminContractDeployer::default(),
            variable_repository: VariableRepositoryDeployer::init(
                test_env::get_account(0),
                test_env::get_account(1),
                test_env::get_account(2),
            ),
            kyc_token: KycNftContractDeployer::init("kyc_token".to_string(), "KYC".to_string(), "".to_string()),
            va_token: VaNftContractDeployer::init("va_token".to_string(), "VAT".to_string(), "".to_string()),
            reputation_token: ReputationContractDeployer::init(),
            rate_provider: CSPRRateProviderContractDeployer::init(DEFAULT_CSPR_USD_RATE.into()),
            virtual_balances: Default::default()
        }
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}


pub mod params;
pub mod helpers;
pub mod contracts;
pub mod config;