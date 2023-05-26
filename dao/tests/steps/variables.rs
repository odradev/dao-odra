use cucumber::{given, then, when};
use odra::types::{U256, U512};

use crate::common::{
    params::{Account, CsprBalance},
    DaoWorld,
};

// #[then(expr = "{account} Bid {word} posted")]
// fn bid_is_posted(world: &mut DaoWorld, account: Account, is_posted: String) {
//     let is_posted = match is_posted.as_str() {
//         "is" => true,
//         "isn't" => false,
//         _ => panic!("Unknown is_posted option - it should be either is or isn't"),
//     };
//     let bid = world.get_bid(0, account);

//     assert_eq!(bid.is_some(), is_posted);
// }

#[then(expr = "value of {word} is {word}")]
fn assert_variable(world: &mut DaoWorld, key: String, value: String) {
    if let Some(current_value) = world.get_variable_or_none::<U512>(&key) {
        let expected = U512::from_dec_str(&value).unwrap();
        assert_eq!(current_value, expected);
    } else if let Some(current_value) = world.get_variable_or_none::<U256>(&key) {
        let expected = U256::from_dec_str(&value).unwrap();
        assert_eq!(current_value, expected);
    } else {
        panic!("Unknown type of variable {}", key)
    }
}

#[given(expr = "the price of USDT is {balance} CSPR")]
fn set_cspr_rate(world: &mut DaoWorld, rate: CsprBalance) {
    world.set_cspr_rate(rate);
}

#[when(expr = "{account} sets the price of USDT to {balance} CSPR")]
fn set_cspr_rate_by(world: &mut DaoWorld, account: Account, rate: CsprBalance) {
    world.set_caller(&account);
    world.set_cspr_rate(rate);
}

#[then(expr = "the price of USDT is {balance} CSPR")]
fn assert_cspr_rate(world: &mut DaoWorld, expected_rate: CsprBalance) {
    assert_eq!(expected_rate, world.get_cspr_rate());
}
