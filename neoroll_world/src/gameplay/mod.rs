pub mod material;
pub mod target;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul},
};

use material::Resource;
use serde::{Deserialize, Serialize};

pub mod behavior;
pub mod build;
pub mod job;
pub mod need;
pub mod progress;
pub mod tribe;

// TODO: Replace by Resource ?
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum CollectType {
    Food,
    RawFlint,
}

impl From<Resource> for CollectType {
    fn from(value: Resource) -> Self {
        match value {
            Resource::Food => Self::Food,
            Resource::RawFlint => Self::RawFlint,
        }
    }
}

/// All in game things have same unit
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Quantity(pub u64);

impl Add for Quantity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Quantity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Sum for Quantity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|q| q.0).sum())
    }
}

/// All in game things have same unit
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Weight(pub u64);

impl Add for Weight {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Weight {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Mul for Weight {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Sum for Weight {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|q| q.0).sum())
    }
}
