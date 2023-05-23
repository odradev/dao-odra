use cucumber::Parameter;
use odra::types::{U256, U512};
use std::{
    ops::{Add, Deref},
    str::FromStr,
};

#[derive(
    Copy, Clone, Debug, Default, derive_more::Deref, PartialEq, Eq, PartialOrd, Ord, Parameter,
)]
#[param(name = "balance", regex = r"\d+")]
pub struct Balance(pub U512);

impl FromStr for Balance {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let value =
            U512::from((s.parse::<f32>().unwrap() * 1_000f32) as u32) * U512::from(1_000_000);
        Ok(Balance(value))
    }
}

impl From<U512> for Balance {
    fn from(value: U512) -> Self {
        Balance(value)
    }
}

impl From<U256> for Balance {
    fn from(value: U256) -> Self {
        Balance(value.to_u512().unwrap())
    }
}

impl Add<Balance> for Balance {
    type Output = Balance;

    fn add(self, rhs: Balance) -> Self::Output {
        let result = self.0 + rhs.0;
        Balance(result)
    }
}

impl Add<Balance> for &Balance {
    type Output = Balance;

    fn add(self, rhs: Balance) -> Self::Output {
        let result = self.0 + rhs.0;
        Balance(result)
    }
}

impl Add<U512> for Balance {
    type Output = Balance;

    fn add(self, rhs: U512) -> Self::Output {
        let result = self.0 + rhs;
        Balance(result)
    }
}

impl Add<U512> for &Balance {
    type Output = Balance;

    fn add(self, rhs: U512) -> Self::Output {
        let result = self.0 + rhs;
        Balance(result)
    }
}

impl Add<U256> for Balance {
    type Output = Balance;

    fn add(self, rhs: U256) -> Self::Output {
        let rhs: Balance = rhs.into();
        self + rhs
    }
}

impl Add<U256> for &Balance {
    type Output = Balance;

    fn add(self, rhs: U256) -> Self::Output {
        let rhs: Balance = rhs.into();
        self + rhs
    }
}

#[allow(dead_code)]
impl Balance {
    pub fn zero() -> Balance {
        U512::zero().into()
    }

    pub fn one() -> Balance {
        U512::from(1_000_000_000).into()
    }
}

#[derive(Clone, Copy, Debug, Default, derive_more::Deref, Parameter, PartialEq)]
#[param(name = "token_id", regex = r"\d+")]
pub struct TokenId(pub dao::core_contracts::TokenId);

impl FromStr for TokenId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let num: u32 = s.parse().map_err(|_| format!("Invalid token id: {}", s))?;
        Ok(Self(dao::core_contracts::TokenId::from(num)))
    }
}

#[derive(Debug, Parameter)]
#[param(name = "time_unit", regex = r".*")]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl FromStr for TimeUnit {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(match s {
            "seconds" | "second" => Self::Seconds,
            "minutes" | "minute" => Self::Minutes,
            "hours" | "hour" => Self::Hours,
            "days" | "day" => Self::Days,
            invalid => {
                panic!("Unknown unit {:?} option - it should be either seconds, minutes, hours or days", invalid)
            }
        })
    }
}

#[derive(Debug, Parameter)]
#[param(name = "result", regex = r"succeeds|fails")]
pub enum Result {
    Success,
    Failure,
}

impl FromStr for Result {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(match s {
            "succeeds" => Self::Success,
            "fails" => Self::Failure,
            _ => panic!("Unknown result option - it should be either succeeds or fails"),
        })
    }
}

impl Deref for Result {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        match self {
            Result::Success => &true,
            Result::Failure => &false,
        }
    }
}
