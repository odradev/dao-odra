use odra::test_env;

use crate::common::{
    params::{Account, TokenId},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn mint_kyc_token(
        &mut self,
        minter: &Account,
        recipient: &Account,
    ) {
        let minter = self.get_address(minter);
        let recipient = self.get_address(recipient);

        test_env::set_caller(minter);
        self.kyc_token.mint(recipient)
    }

    // pub fn mint_kyc_token(&mut self, minter: &Account, recipient: &Account) {
    //     self.checked_mint_kyc_token(minter, recipient)
    //         .expect("A KYC Token should be minted successfully");
    // }

    pub fn burn_kyc_token(
        &mut self,
        burner: &Account,
        holder: &Account,
    ) {
        let burner = self.get_address(burner);
        let holder = self.get_address(holder);
        test_env::set_caller(burner);
        self.kyc_token.burn(holder)
    }

    // pub fn burn_kyc_token(&mut self, minter: &Account, recipient: &Account) {
    //     self.checked_burn_kyc_token(minter, recipient)
    //         .expect("A token should be burned");
    // }

    pub fn get_kyc_token_id(&self, holder: &Account) -> TokenId {
        let holder = self.get_address(holder);
        let id = self
            .kyc_token
            .token_id(holder)
            .expect("Holder should own a token");
        TokenId(id)
    }

    pub fn is_account_kyced(&self, account: &Account) -> bool {
        let address = self.get_address(account);

        !self.kyc_token.balance_of(address).is_zero()
    }
}
