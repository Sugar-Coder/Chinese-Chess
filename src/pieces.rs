use bevy::prelude::*;
use crate::{configs::GL, util::grid2xy};

pub fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    spawn_jiang(&mut commands, &asset_server);
    spawn_shuai(&mut commands, &asset_server);
    spawn_shi(&mut commands, &asset_server);
    spawn_xiang(&mut commands, &asset_server);
    spawn_ma(&mut commands, &asset_server);
    spawn_che(&mut commands, &asset_server);
    spawn_pao(&mut commands, &asset_server);
    spawn_soldier(&mut commands, &asset_server);
}

fn spawn_shuai(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/shuai.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(4, 9).extend(1.0)),
        ..Default::default()
    });
}

fn spawn_jiang(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/jiang.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(4, 0).extend(1.0)),
        ..Default::default()
    });
}

fn spawn_shi(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/shi.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(3, 9).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/shi.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(5, 9).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/shi.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(3, 0).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/shi.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(5, 0).extend(1.0)),
        ..Default::default()
    });
}

fn spawn_xiang(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/xiang.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(2, 9).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/xiang.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(6, 9).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/xiang.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(2, 0).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/xiang.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(6, 0).extend(1.0)),
        ..Default::default()
    });
}

fn spawn_ma(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/ma.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(1, 9).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/ma.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(7, 9).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/ma.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(1, 0).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/ma.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(7, 0).extend(1.0)),
        ..Default::default()
    });
}

fn spawn_che(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/che.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(0, 9).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/che.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(8, 9).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/che.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(0, 0).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/che.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(8, 0).extend(1.0)),
        ..Default::default()
    });
}

fn spawn_pao(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/pao.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(1, 7).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("red/pao.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(7, 7).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/pao.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(1, 2).extend(1.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("black/pao.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(GL)),
            ..Default::default()
        },
        transform: Transform::from_translation(grid2xy(7, 2).extend(1.0)),
        ..Default::default()
    });
}

fn spawn_soldier(
    commands: &mut Commands,
    asset_server: & Res<AssetServer>
) {
    for i in vec![0, 2, 4, 6, 8] {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("red/bing.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(GL)),
                ..Default::default()
            },
            transform: Transform::from_translation(grid2xy(i, 6).extend(1.0)),
            ..Default::default()
        });
    }
    for i in vec![0, 2, 4, 6, 8] {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("black/zu.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(GL)),
                ..Default::default()
            },
            transform: Transform::from_translation(grid2xy(i, 3).extend(1.0)),
            ..Default::default()
        });
    }
}