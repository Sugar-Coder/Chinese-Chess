use crate::{
    configs::GL,
    pos::{Pos, PosEntityMap},
    chess::{ChessGame, PlayerColor},
    util::*,
};
use bevy::{prelude::*, window::PrimaryWindow};

use std::collections::HashMap;

#[derive(Component)]
pub struct ChessBoardTexture;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("chessboard.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(520.0, 520.0)),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1.12, 1.1, 1.0)),
            ..default()
        },
        ChessBoardTexture,
    ));
}

#[derive(Resource, Default)]
struct Game {
    to_play: Option<(Pos, Pos)>,
    turn: u32,
    last_move_time: f32,
}

#[derive(Resource, Default)]
struct SelectedSquare(Option<Pos>);

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
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            info!("World coords: {}/{}", world_position.x, world_position.y);
            if in_bound(&world_position) {
                let pos = world_to_board(&world_position);
                if let Some(old_pos) = selected.0 {
                    info!(
                        "move ({},{}) to ({},{})",
                        old_pos.0, old_pos.1, pos.0, pos.1
                    );
                    if old_pos.0 != pos.0 || old_pos.1 != pos.1 {
                        game.to_play = Some((old_pos, pos));
                        selected.0 = None;
                    } else {
                        selected.0 = Some(pos);
                    }
                } else {
                    info!(
                        "select ({},{}), world position:({})",
                        pos.0,
                        pos.1,
                        board_to_world(pos).translation
                    );
                    selected.0 = Some(pos);
                }
            } else {
                selected.0 = None;
            }
        }
    }
}

#[derive(Component)]
struct MovingTo(Transform);

#[derive(Component)]
struct Die;

fn play_move(
    mut commands: Commands,
    mut piece_ents: ResMut<PosEntityMap>,
    mut game: ResMut<Game>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.last_move_time < 1. {
        return;
    }
    if let Some((from, to)) = game.to_play {
        let ent = *piece_ents.0.get(&from).unwrap(); // use * to copy value, not immutable borrow
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

fn die(mut commands: Commands, query: Query<Entity, With<Die>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}

fn place_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut piece_ents: ResMut<PosEntityMap>,
    chess_game: Res<ChessGame>,
) {
    for (i, grid) in chess_game.board.points.iter().enumerate() {
        if let Some((color, piece)) = grid {
            let pos = chess_game.board.pos(i);
            let texture = match color {
                PlayerColor::Red => format!("red/{}.png", piece),
                PlayerColor::Black => format!("black/{}.png", piece),
            };
            piece_ents.0.insert(
                pos,
                commands
                    .spawn(SpriteBundle {
                        texture: asset_server.load(texture),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(GL)),
                            ..Default::default()
                        },
                        transform: board_to_world(pos),
                        ..Default::default()
                    })
                    .id(),
            );
        }
    }
}

pub struct ChineseChess;

impl Plugin for ChineseChess {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(PosEntityMap(HashMap::<Pos, Entity>::new()))
            .insert_resource(SelectedSquare(None))
            .insert_resource(Game::default())
            .insert_resource(ChessGame::new())
            .add_systems(Startup, (setup, place_pieces))
            .add_systems(Update, mouse_click_system)
            .add_systems(Update, (play_move, move_to, die));
    }
}
