use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod configs;
mod pieces;
mod util;
use configs::*;
use pieces::*;
use util::*;

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
        .add_systems(Startup, (setup, create_pieces))
        .add_systems(Update, update_board_size)
        .run();
}

#[derive(Component)]
struct ChessBoard;

#[derive(Component)]
struct GridPoint {
    pub x: u8,
    pub y: u8,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("chessboard.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(520.0, 520.0)),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1.11, 1.11, 1.0)),
            ..default()
        },
        ChessBoard,
    ));
    // implement clicking on board to move pieces
    // for i in 0..=8 {
    //     for j in 0..=9 {
    //         let pos = grid2xy(i, j);
    //         commands.spawn((
    //             SpriteBundle {
    //                 sprite: Sprite {
    //                     custom_size: Some(Vec2::splat(GL)),
    //                     ..default()
    //                 },
    //                 transform: Transform::from_translation(pos.extend(0.0)),
    //                 ..default()
    //             },
    //             GridPoint{x: i, y: j},
    //         ));
    //     }
    // }
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
    //     ..default()
    // });
}


// for debug adjusting the board picture size
fn update_board_size(
    mut query: Query<&mut Transform, With<ChessBoard>>,
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

