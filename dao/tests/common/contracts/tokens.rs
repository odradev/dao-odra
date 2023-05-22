use dao::core_contracts::TokenId;
use odra::types::{U512, Address, U256};

use crate::common::{DaoWorld, params::{Account, Contract}};

#[odra::external_contract]
pub trait TotalSupply {
    fn total_supply(&self) -> U512;
}

#[odra::external_contract]
pub trait NftToken {
    fn balance_of(&self, owner: Address) -> U256;
    fn owner_of(&self, token_id: TokenId) -> Address;
}

impl DaoWorld {
    pub fn total_supply(&self, contract: Contract) -> U512 {
        let contract = self.contract_address(contract);
        TotalSupplyRef::at(contract).total_supply()
    }

    pub fn nft_balance_of(&self, contract: Contract, account: &Account) -> u32 {
        let contract = self.contract_address(contract);

        NftTokenRef::at(contract)
            .balance_of(self.get_address(account))
            .as_u32()
    }

    pub fn nft_owner_of(&self, contract: Contract, token_id: TokenId) -> Address {
        let contract = self.contract_address(contract);

        NftTokenRef::at(contract)
            .owner_of(token_id)
    }

    fn contract_address(&self, contract: Contract) -> Address {
        let account = Account::Contract(contract);
        self.get_address(&account)
    }
}