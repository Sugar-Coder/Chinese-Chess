use bevy::{prelude::*, transform::commands, window::PrimaryWindow};

mod configs;
mod pieces;
mod util;
use configs::*;
use pieces::*;
use util::*;

use std::collections::HashMap;

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
        .insert_resource(PosMap(HashMap::<Pos, Entity>::new()))
        .insert_resource(SelectedSquare(None))
        .insert_resource(Game::default())
        .add_systems(Startup, (setup, create_pieces))
        .add_systems(Update, (update_board_size, mouse_click_system))
        .add_systems(Update, (play_move, move_to, die))
        .run();
}

#[derive(Component)]
struct ChessBoard;

#[derive(Component)]
struct MainCamera;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos(pub u8, pub u8);

#[derive(Resource, Default)]
struct SelectedSquare(Option<Pos>);

#[derive(Resource, Default)]
pub struct PosMap(HashMap<Pos, Entity>);

#[derive(Resource, Default)]
pub struct Game {
    to_play: Option<(Pos, Pos)>,
    turn: u32,
    last_move_time: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("chessboard.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(520.0, 520.0)),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1.12, 1.1, 1.0)),
            ..default()
        },
        ChessBoard,
    ));
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

fn mouse_click_system(
    buttons: Res<Input<MouseButton>>,
    mut selected: ResMut<SelectedSquare>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut game: ResMut<Game>,
) {
    if buttons.just_released(MouseButton::Left) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            info!("World coords: {}/{}", world_position.x, world_position.y);
            if in_bound(&world_position) {
                let pos = world_to_board(&world_position);
                if let Some(old_pos) = selected.0 {
                    info!("move ({},{}) to ({},{})", old_pos.0, old_pos.1, pos.0, pos.1);
                    if old_pos.0 != pos.0 || old_pos.1 != pos.1 {
                        game.to_play = Some((old_pos, pos));
                        selected.0 = None;
                    } else {
                        selected.0 = Some(pos);
                    }
                } else {
                    info!("select ({},{}), world position:({})",  pos.0, pos.1, board_to_world(pos).translation);
                    selected.0 = Some(pos);
                }
            } else {
                selected.0 = None;
            }
        }
    }
}

fn in_bound(world_position: &Vec2) -> bool {
    if world_position.x >= -4.5 * GL && world_position.x <= 4.5 * GL &&
        world_position.y >= -5. * GL && world_position.y <= 5. * GL {
            return true;
    } else {
        return false;
    }
}

fn world_to_board(world_position: &Vec2) -> Pos {
    return Pos {
        0: ((world_position.x + 4.5 * GL) / GL) as u8,
        1: ((world_position.y + 5.0 * GL) / GL) as u8,
    }
}

fn board_to_world(pos: Pos) -> Transform {
    Transform::from_xyz(
        (pos.0 as f32 - 4.0) * GL, 
        (pos.1 as f32 - 4.5) * GL, 
        1.0
    )
}

#[derive(Component)]
struct MovingTo(Transform);

#[derive(Component)]
struct Die;

fn play_move(
    mut commands: Commands,
    mut piece_ents: ResMut<PosMap>,
    mut game: ResMut<Game>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.last_move_time < 1. {
        return;
    }
    if let Some((from, to)) = game.to_play {
        let ent = *piece_ents.0.get(&from).unwrap(); // 值复制
        commands.entity(ent).insert(MovingTo(board_to_world(to)));
        if let Some(o_ent) = piece_ents.0.get(&to) {
            commands.entity(*o_ent).insert(Die);
        }
        piece_ents.0.remove_entry(&from);
        piece_ents.0.insert(to, ent);
        game.to_play = None;
    }
}

fn move_to(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MovingTo)>,
    time: Res<Time>,
) {
    for (ent, mut transform, moving_to) in query.iter_mut() {
        let mut diff = moving_to.0.translation - transform.translation;
        let mut step = time.delta_seconds() * GL as f32 * 20.;
        // the piece finished moving
        if step >= diff.length() {
            step = diff.length();
            commands.entity(ent).remove::<MovingTo>();
        }
        if step > 0. {
            diff = step * diff / diff.length();
            transform.translation = transform.translation + diff;
        }
    }
}

fn die(
    mut commands: Commands,
    query: Query<Entity, With<Die>>,
) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}