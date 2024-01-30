use bevy::prelude::*;
use crate::{configs::GL, util::grid2xy, Pos, PosMap};

pub fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut piece_ents: ResMut<PosMap>,
) {
    spawn_jiang(&mut commands, &asset_server, &mut piece_ents);
    spawn_shuai(&mut commands, &asset_server, &mut piece_ents);
    spawn_shi(&mut commands, &asset_server, &mut piece_ents);
    spawn_xiang(&mut commands, &asset_server, &mut piece_ents);
    spawn_ma(&mut commands, &asset_server, &mut piece_ents);
    spawn_che(&mut commands, &asset_server, &mut piece_ents);
    spawn_pao(&mut commands, &asset_server, &mut piece_ents);
    spawn_soldier(&mut commands, &asset_server, &mut piece_ents);
}

fn spawn_shuai(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>,
    piece_ents: &mut ResMut<PosMap>,
) {
    let pos = Pos(4, 9);
    piece_ents.0.insert(
        pos,
        commands.spawn(SpriteBundle {
            texture: asset_server.load("red/shuai.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(GL)),
                ..Default::default()
            },
            transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
            ..Default::default()
        }).id()
    );
}

fn spawn_jiang(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>,
    piece_ents: &mut ResMut<PosMap>,
) {
    let pos = Pos(4, 0);
    piece_ents.0.insert(
        pos,
        commands.spawn(SpriteBundle {
            texture: asset_server.load("black/jiang.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(GL)),
                ..Default::default()
            },
            transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
            ..Default::default()
        }).id()
    );
}

fn spawn_shi(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>,
    piece_ents: &mut ResMut<PosMap>,
) {
    let positions = vec![
        Pos(3, 9),
        Pos(5, 9),
        Pos(3, 0),
        Pos(5, 0),
    ];
    for pos in positions {
        let mut path = format!("red/shi.png");
        if pos.1 == 0 {
            path = format!("black/shi.png");
        }
        piece_ents.0.insert(
            pos,
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(GL)),
                    ..Default::default()
                },
                transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
                ..Default::default()
            }).id()
        );
    }
}

fn spawn_xiang(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>,
    piece_ents: &mut ResMut<PosMap>,
) {
    let positions = vec![
        Pos(2, 9),
        Pos(6, 9),
        Pos(2, 0),
        Pos(6, 0),
    ];
    for pos in positions {
        let mut path = format!("red/xiang.png");
        if pos.1 == 0 {
            path = format!("black/xiang.png");
        }
        piece_ents.0.insert(
            pos,
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(GL)),
                    ..Default::default()
                },
                transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
                ..Default::default()
            }).id()
        );
    }
}

fn spawn_ma(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>,
    piece_ents: &mut ResMut<PosMap>,
) {
    let positions = vec![
        Pos(1, 9),
        Pos(7, 9),
        Pos(1, 0),
        Pos(7, 0),
    ];
    for pos in positions {
        let mut path = format!("red/ma.png");
        if pos.1 == 0 {
            path = format!("black/ma.png");
        }
        piece_ents.0.insert(
            pos,
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(GL)),
                    ..Default::default()
                },
                transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
                ..Default::default()
            }).id()
        );
    }
}

fn spawn_che(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>,
    piece_ents: &mut ResMut<PosMap>,
) {
    let positions = vec![
        Pos(0, 9),
        Pos(8, 9),
        Pos(0, 0),
        Pos(8, 0),
    ];
    for pos in positions {
        let mut path = format!("red/che.png");
        if pos.1 == 0 {
            path = format!("black/che.png");
        }
        piece_ents.0.insert(
            pos,
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(GL)),
                    ..Default::default()
                },
                transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
                ..Default::default()
            }).id()
        );
    }
}

fn spawn_pao(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>,
    piece_ents: &mut ResMut<PosMap>,
) {
    let positions = vec![
        Pos(1, 7),
        Pos(7, 7),
        Pos(1, 2),
        Pos(7, 2),
    ];
    for pos in positions {
        let mut path = format!("red/pao.png");
        if pos.1 == 2 {
            path = format!("black/pao.png");
        }
        piece_ents.0.insert(
            pos,
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(GL)),
                    ..Default::default()
                },
                transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
                ..Default::default()
            }).id()
        );
    }
}

fn spawn_soldier(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>,
    piece_ents: &mut ResMut<PosMap>,
) {
    for i in vec![0, 2, 4, 6, 8] {
        let pos = Pos(i, 6);
        piece_ents.0.insert(
            pos,
            commands.spawn(SpriteBundle {
                texture: asset_server.load("red/bing.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(GL)),
                    ..Default::default()
                },
                transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
                ..Default::default()
            }).id()
        );
    }
    for i in vec![0, 2, 4, 6, 8] {
        let pos = Pos(i, 3);
        piece_ents.0.insert(
            pos,
            commands.spawn(SpriteBundle {
                texture: asset_server.load("black/zu.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(GL)),
                    ..Default::default()
                },
                transform: Transform::from_translation(grid2xy(pos.0, pos.1).extend(1.0)),
                ..Default::default()
            }).id()
        );
    }
}