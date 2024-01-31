use bevy::prelude::*;
use crate::{configs::GL, pos::Pos};
// 棋盘上交点的很纵坐标 (horizon 0..=8, verticle 0..=9)
// return the middle xy cordinate of the grid cross point
pub fn grid2xy(horizon: i32, verticle: i32) -> Vec2 {
    assert!(horizon <= 8 && verticle <= 9);
    return Vec2::new(horizon as f32 * GL - 4.0 * GL, verticle as f32 * GL - 4.5 * GL);
}


pub fn in_bound(world_position: &Vec2) -> bool {
    if world_position.x >= -4.5 * GL && world_position.x <= 4.5 * GL &&
        world_position.y >= -5. * GL && world_position.y <= 5. * GL {
            return true;
    } else {
        return false;
    }
}

pub fn world_to_board(world_position: &Vec2) -> Pos {
    return Pos {
        0: ((world_position.x + 4.5 * GL) / GL) as i32,
        1: ((world_position.y + 5.0 * GL) / GL) as i32,
    }
}

pub fn board_to_world(pos: Pos) -> Transform {
    Transform::from_xyz(
        (pos.0 as f32 - 4.0) * GL, 
        (pos.1 as f32 - 4.5) * GL, 
        1.0
    )
}