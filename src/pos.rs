use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos(pub u8, pub u8);

#[derive(Resource, Default)]
pub struct PosEntityMap(pub HashMap<Pos, Entity>);
