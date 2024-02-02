use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::{Add, Mul};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn in_bound(&self, bottom_left: Pos, top_right: Pos) -> bool {
        self.0 >= bottom_left.0 && self.0 <= top_right.0 && self.1 >= bottom_left.1 && self.1 <= top_right.1
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        // Self(self.0 + rhs.0, self.1 + rhs.1)
        Self {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}


#[derive(Resource, Default)]
pub struct PosEntityMap(pub HashMap<Pos, Entity>);
