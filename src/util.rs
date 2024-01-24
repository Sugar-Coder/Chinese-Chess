use bevy::prelude::*;
use crate::configs::GL;
// 棋盘上交点的很纵坐标 (horizon 0..=8, verticle 0..=9)
// return the middle xy cordinate of the grid cross point
pub fn grid2xy(horizon: u8, verticle: u8) -> Vec2 {
    assert!(horizon <= 8 && verticle <= 9);
    return Vec2::new(horizon as f32 * GL - 4.0 * GL, verticle as f32 * GL - 4.5 * GL);
}