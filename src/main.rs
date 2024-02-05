mod configs;
mod pieces;
mod util;
mod game;
mod pos;
mod chess;
use configs::*;
use game::{ChineseChess, ChessBoardTexture};

use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window{
                    resolution: (WW as f32, WH as f32).into(),
                    title: "chineses-chess".to_string(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(ChineseChess)
        // .add_systems(Update, update_board_size)
        .run();
}


// for debug adjusting the board picture size
fn update_board_size(
    mut query: Query<&mut Transform, With<ChessBoardTexture>>,
    keycode: Res<Input<KeyCode>>,
) {
    let mut chessboard_transform = query.single_mut();
    // todo
    if keycode.pressed(KeyCode::Left) {
        chessboard_transform.scale.x -= 0.01;
    } else if keycode.pressed(KeyCode::Right) {
        chessboard_transform.scale.x += 0.01;
    } else if keycode.pressed(KeyCode::Up) {
        chessboard_transform.scale.y += 0.01;
    } else if keycode.pressed(KeyCode::Down) {
        chessboard_transform.scale.y -= 0.01;
    } else if keycode.pressed(KeyCode::P) {
        println!("chessboard scale:{}", chessboard_transform.scale);
    }
}