use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos(pub i32, pub i32);

#[derive(Resource, Default)]
pub struct PosEntityMap(pub HashMap<Pos, Entity>);
