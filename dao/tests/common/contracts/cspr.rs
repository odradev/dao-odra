use std::collections::HashMap;

use odra::{test_env, types::Address};

use crate::common::{
    helpers::is_balance_close_enough,
    params::{Account, Balance},
    DaoWorld,
};

#[derive(Default)]
pub struct VirtualBalances {
    current: HashMap<Address, Balance>,
    initial: HashMap<Address, Balance>,
}

impl VirtualBalances {
    pub fn init(&mut self, account: Address, amount: Balance) {
        assert!(
            !self.current.contains_key(&account),
            "Cannot set cspr balance twice"
        );

        self.current.insert(account, amount.into());

        self.initial
            .insert(account, test_env::token_balance(account).into());
    }

    pub fn get(&self, address: Address) -> Balance {
        let balance = self.current.get(&address).unwrap() + test_env::token_balance(address);
        let result = balance
            .checked_sub(self.initial.get(&address).unwrap().0)
            .unwrap();
        result.into()
    }
}

#[allow(dead_code)]
impl DaoWorld {
    // sets relative amount of motes to the account
    pub fn set_cspr_balance(&mut self, account: &Account, amount: Balance) {
        let account = self.get_address(account);

        self.virtual_balances.init(account, amount);
    }

    // gets relative amount of motes of the account
    pub fn get_cspr_balance(&self, account: &Account) -> Balance {
        let account = self.get_address(account);
        self.virtual_balances.get(account)
    }

    pub fn assert_cspr_balance(&self, account: &Account, expected_balance: Balance) {
        let real_cspr_balance = self.get_cspr_balance(account);

        assert!(
            is_balance_close_enough(expected_balance, real_cspr_balance),
            "For account {:?} CSPR balance should be {:?} but is {:?}",
            account,
            expected_balance,
            real_cspr_balance
        );
    }
}
