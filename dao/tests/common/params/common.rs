use std::{ops::Deref, str::FromStr};
use cucumber::Parameter;

#[derive(
    Copy, Clone, Debug, Default, derive_more::Deref, PartialEq, Eq, PartialOrd, Ord, Parameter,
)]
#[param(name = "balance", regex = r"\d+")]
pub struct Balance(pub odra::types::Balance);

impl FromStr for Balance {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let value = odra::types::Balance::from((s.parse::<f32>().unwrap() * 1_000f32) as u32) * 1_000_000;
        Ok(Balance(value))
    }
}

impl From<odra::types::Balance> for Balance {
    fn from(value: odra::types::Balance) -> Self {
        Balance(value)
    }
}

#[allow(dead_code)]
impl Balance {
    pub fn zero() -> Balance {
        odra::types::Balance::zero().into()
    }

    pub fn one() -> Balance {
        odra::types::Balance::from(1_000_000_000).into()
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
