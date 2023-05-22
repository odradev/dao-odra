use std::{ops::{Deref, Add}, str::FromStr};
use cucumber::Parameter;
use odra::types::{U512, U256};

#[derive(
    Copy, Clone, Debug, Default, derive_more::Deref, PartialEq, Eq, PartialOrd, Ord, Parameter,
)]
#[param(name = "balance", regex = r"\d+")]
pub struct Balance(pub U512);

impl FromStr for Balance {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let value = U512::from((s.parse::<f32>().unwrap() * 1_000f32) as u32) * 1_000_000;
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
        let mut bytes_vec = value.0.to_vec();
        bytes_vec.resize(8, 0);

        let mut bytes = [0u64; 8];
        bytes.copy_from_slice(bytes_vec.as_slice());
        U512(bytes).into()
    }
}

impl Add<Balance> for Balance {
    type Output = Balance;

    fn add(self, rhs: Balance) -> Self::Output {
        let result = self.0 + rhs.0;
        Balance(result)
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

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter, PartialEq)]
#[param(name = "token_id", regex = r"\d+")]
pub struct TokenId(pub dao::core_contracts::TokenId);

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
