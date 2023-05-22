use odra::test_env;

use crate::common::{
    params::{Account, TokenId},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn is_va_account(&self, account: &Account) -> bool {
        let address = self.get_address(account);
        !self.va_token.balance_of(address).is_zero()
    }

    pub fn mint_va_token(
        &mut self,
        minter: &Account,
        recipient: &Account,
    ) {
        let minter = self.get_address(minter);
        let recipient = self.get_address(recipient);

        test_env::set_caller(minter);
        self.va_token.mint(recipient)
    }

    // pub fn mint_va_token(&mut self, minter: &Account, recipient: &Account) {
    //     self.checked_mint_va_token(minter, recipient)
    //         .expect("A VA Token should be minted successfully");
    // }

    pub fn burn_va_token(
        &mut self,
        burner: &Account,
        holder: &Account,
    ) {
        let burner = self.get_address(burner);
        let holder = self.get_address(holder);

        test_env::set_caller(burner);
        self.va_token.burn(holder)
    }

    // pub fn burn_va_token(&mut self, burner: &Account, holder: &Account) {
    //     self.checked_burn_va_token(burner, holder)
    //         .expect("VA Token should burned successfully");
    // }

    // pub fn va_token_balance_of(&self, account: &Account) -> Balance {
    //     let address = self.get_address(account);

    //     self.va_token.balance_of(address).into()
    // }

    pub fn get_va_token_id(&self, holder: &Account) -> TokenId {
        let holder = self.get_address(holder);
        let id = self
            .va_token
            .token_id(holder)
            .expect("Holder should own a token");
        TokenId(id)
    }

    pub fn is_va(&self, account: &Account) -> bool {
        let address = self.get_address(account);
        !self.va_token.balance_of(address).is_zero()
    }
}
