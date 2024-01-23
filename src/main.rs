use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// window
const WW: usize = 500;
const WH: usize = 500;

// board
const GL: f32 = 50.0; // grid length
const BW: f32 = GL * 8.0; // board width
const BH: f32 = GL * (10 + 1) as f32; // board height
const LINE_WIDTH: f32 = GL / 50.0;
// board target mark
const SP: f32 = GL / 10.0; // space
const EL: f32 = GL / 5.0; // edge length

// color
const COLOR_BOARD: (u8, u8, u8) = (201, 114, 32);

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
        .add_systems(Startup, (setup, create_board))
        .add_systems(Update, draw_lines_system)
        .run();
}

fn setup(
    mut commands: Commands,
    // mut gizmos: Gizmos,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
    // gizmos.line_2d(Vec2::new(-BW / 2.0, BW / 2.0), Vec2::splat(-BW / 2.0), Color::RED);
    // gizmos.line_2d(Vec2::splat(BW / 2.0), Vec2::new(BW / 2.0, -BW / 2.0), Color::RED);
    // commands.spawn(SpriteBundle{
    //     texture: asset_server.load("chessboard.png"),
    //     sprite: Sprite {
    //         custom_size: Some(Vec2::new(500.0, 500.0)),
    //         ..default()
    //     },
    //     // transform: Transform::from_scale(Vec3::new(0.25, 0.25, -1.0)),
    //     ..default()
    // });
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
    //     ..default()
    // });
}

fn create_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawn the chessboard
    for row in 0..8 {
        for col in 0..8 {
            let pos_x = col as f32 * GL - 3.5 * GL;
            let mut pos_y = row as f32 * GL - 4.0 * GL;
            if row > 3 {
                pos_y += GL;
            }
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 0., 0.), // black broader
                    custom_size: Some(Vec2::new(GL, GL)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
                ..default()
            });
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(COLOR_BOARD.0, COLOR_BOARD.1, COLOR_BOARD.2),
                    custom_size: Some(Vec2::new(GL - LINE_WIDTH, GL - LINE_WIDTH)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
                ..default()
            });
        }
    }
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0., 0.),
            custom_size: Some(Vec2::new(BW, GL)),
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(COLOR_BOARD.0, COLOR_BOARD.1, COLOR_BOARD.2),
            custom_size: Some(Vec2::new(BW - LINE_WIDTH, GL - LINE_WIDTH * 2.0)),
            ..default()
        },
        ..default()
    });
    commands.spawn(TextBundle::from_section(
        "Chuhe Hanjie",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));
}

// row 0..=8, col 0..=9
// return the middle xy cordinate of the grid
fn grid2xy(row: usize, col: usize) -> Vec2 {
    return Vec2::new(row as f32 * GL - 4.0 * GL, col as f32 * GL - 4.5 * GL);
}

fn draw_lines_system(mut gizmos: Gizmos) {
    gizmos.line_2d(grid2xy(3, 0), grid2xy(5, 2), Color::BLACK);
    gizmos.line_2d(grid2xy(3, 2), grid2xy(5, 0), Color::BLACK);
    gizmos.line_2d(grid2xy(3, 7), grid2xy(5, 9), Color::BLACK);
    gizmos.line_2d(grid2xy(3, 9), grid2xy(5, 7), Color::BLACK);
    for row in vec![1, 7] {
        for dir in 1..=4 {
            gizmos.linestrip_2d(get_jing_position(row, 2, dir), Color::BLACK);
            gizmos.linestrip_2d(get_jing_position(row, 7, dir), Color::BLACK);
        }
    }
    for row in vec![0, 2, 4, 6, 8] {
        if row == 0 {
            for dir in vec![1, 4] { // only plot first and fourth quadrant
                gizmos.linestrip_2d(get_jing_position(row, 3, dir), Color::BLACK);
                gizmos.linestrip_2d(get_jing_position(row, 6, dir), Color::BLACK);
            }
        } else if row == 8 {
            for dir in vec![2, 3] { // only plot first and fourth quadrant
                gizmos.linestrip_2d(get_jing_position(row, 3, dir), Color::BLACK);
                gizmos.linestrip_2d(get_jing_position(row, 6, dir), Color::BLACK);
            }
        } else {
            for dir in 1..=4 {
                gizmos.linestrip_2d(get_jing_position(row, 3, dir), Color::BLACK);
                gizmos.linestrip_2d(get_jing_position(row, 6, dir), Color::BLACK);
            }
        }
    }
}

// return positions for gizmos
// dir (1..=4)  1 -- the first quadrant; 2 -- the second quadrant
fn get_jing_position(row: usize, col: usize, dir: usize) -> Vec<Vec2> {
    let cc = grid2xy(row, col); // center coordinate
    let positions = vec![
        Vec2::new(SP, SP + EL),
        Vec2::new(SP, SP),
        Vec2::new(SP + EL, SP)
    ];
    if dir == 1 {
        return positions.iter().map(|p| { Vec2::new(cc.x + p.x, cc.y + p.y) }).collect();
    } else if dir == 2 {
        return positions.iter().map(|p| { Vec2::new(cc.x - p.x, cc.y + p.y) }).collect();
    } else if dir == 3 {
        return positions.iter().map(|p| { Vec2::new(cc.x - p.x, cc.y - p.y)}).collect();
    } else {
        return positions.iter().map(|p| { Vec2::new(cc.x + p.x, cc.y - p.y)}).collect();
    }
}