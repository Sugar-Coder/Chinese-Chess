use crate::{
    configs::GL,
    pos::{Pos, PosEntityMap},
    chess::ChessGame,
    pieces::{PlayerColor, Action},
};
use bevy::{prelude::*, window::PrimaryWindow};

use std::collections::HashMap;

#[derive(Component)]
pub struct ChessBoardTexture;

#[derive(Component)]
struct MainCamera;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    chess: Res<ChessGame>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("chessboard.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(520.0, 520.0)),
                ..default()
            },
            transform: Transform::from_translation(chess.board.center.extend(0.0)).with_scale(Vec3::new(1.12, 1.1, 1.0)),
            ..default()
        },
        ChessBoardTexture,
    ));
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            bottom: Val::Percent(0.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // background_color: BackgroundColor(Color::RED),
        ..default()
    })
    .with_children(|parent| {
        for button_text in vec!["regret", "restart"] {
            parent.spawn(
                ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        margin: UiRect{left: Val::Px(10.0), ..default()},
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        button_text,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        }
    });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
    mut game: ResMut<Game>,
    mut chess: ResMut<ChessGame>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                info!("Press {}", text.sections[0].value);
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                if text.sections[0].value == "restart" {
                    game.state = GameState::Starting;
                    chess.restart();
                } else if text.sections[0].value == "regret" {
                    if chess.regret() {
                        game.state = GameState::Starting;
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    Starting,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Starting
    }
}

#[derive(Resource, Default)]
struct Game {
    to_play: Option<(Pos, Action)>,
    state: GameState,
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
    chess: Res<ChessGame>,
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
            // info!("World coords: {}/{}", world_position.x, world_position.y);
            if chess.in_bound(&world_position) {
                let pos = chess.world_to_board(&world_position);
                if let Some(old_pos) = selected.0 {
                    if let Some(Some((o_c, _))) = chess.board.get(old_pos) {
                        if let Some(Some((c, _))) = chess.board.get(pos) {
                            // same color piece selection
                            if *o_c == *c {
                                selected.0 = Some(pos);
                                return;
                            }
                        }
                    }
                    if let Some(action) = chess.playable_move(old_pos, pos) {
                        game.to_play = Some((old_pos, action));
                        info!("Goto ({}, {})", pos.0, pos.1);
                        selected.0 = None;
                    } else {
                        selected.0 = None;
                    }
                } else {
                    if let Some(Some((_, p))) = chess.board.get(pos) {
                        selected.0 = Some(pos);
                        info!("selecting {}", p);
                    } else {
                        selected.0 = None;
                    }
                }
            } else {
                selected.0 = None;
            }
        }
    }
}

#[derive(Component)]
struct MoveDisplay;

fn display_moves(
    query: Query<Entity, With<MoveDisplay>>,
    mut commands: Commands,
    selected: Res<SelectedSquare>,
    chess: Res<ChessGame>,
    asset_server: Res<AssetServer>,
) {
    if selected.is_changed() {
        for move_display in query.iter() {
            commands.entity(move_display).despawn();
        }
        if let Some(pos) = selected.0 {
            if let Some(moves) = chess.playable_moves(pos) {
                let sprite = SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0., 0., 0., 0.5),
                        custom_size: Some(Vec2::new(GL / 2.5, GL / 2.5)),
                        ..Default::default()
                    },
                    texture: asset_server.load("circle.png"),
                    ..Default::default()
                };
                for action in moves {
                    if let Action::Go(to) = action {
                        let mut sprite_clone = sprite.clone();
                        sprite_clone.transform = chess.board_to_world(to);
                        commands.spawn(sprite_clone).insert(MoveDisplay);
                    }
                }
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
    mut chess: ResMut<ChessGame>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.last_move_time < 1. {
        return;
    }
    if let Some((from, action)) = game.to_play {
        let ent = *piece_ents.0.get(&from).unwrap(); // use * to copy value, not immutable borrow
        match action {
            Action::Go(to) => {
                commands.entity(ent).insert(MovingTo(chess.board_to_world(to)));
                piece_ents.0.insert(to, ent);
            },
            Action::Take(pos) => {
                commands.entity(ent).insert(MovingTo(chess.board_to_world(pos)));
                if let Some(o_ent) = piece_ents.0.get(&pos) {
                    commands.entity(*o_ent).insert(Die);
                }
                piece_ents.0.insert(pos, ent);
            }
        }
        piece_ents.0.remove_entry(&from);
        game.to_play = None;
        chess.play(from, action);
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
    mut game: ResMut<Game>,
    chess: Res<ChessGame>,
) {
    if game.state != GameState::Starting {
        return;
    }
    for (_, ent) in piece_ents.0.iter() {
        commands.entity(*ent).despawn();
    }
    piece_ents.0.clear();
    for (i, grid) in chess.board.points.iter().enumerate() {
        if let Some((color, piece)) = grid {
            let pos = chess.board.pos(i);
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
                        transform: chess.board_to_world(pos),
                        ..Default::default()
                    })
                    .id(),
            );
        }
    }
    game.state = GameState::Playing;
}

pub struct ChineseChess;

impl Plugin for ChineseChess {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(PosEntityMap(HashMap::<Pos, Entity>::new()))
            .insert_resource(SelectedSquare(None))
            .insert_resource(Game::default())
            .insert_resource(ChessGame::new())
            .add_systems(Startup, setup)
            .add_systems(Update, (place_pieces, mouse_click_system, display_moves, button_system))
            .add_systems(Update, (play_move, move_to, die));
    }
}
