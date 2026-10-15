use bevy::prelude::*;

use crate :: {
    GameState, LEVEL_BOTTOM, LEVEL_LEFT, LEVEL_RIGHT, TILE_SIZE,
    loading::{ LoadingAssets, despawn_with },
    stats::{ Health, MoveSpeed, GERALD_HEALTH, GERALD_DAMAGE, GERALD_SPEED },
};

const SPRITE_SCALE: f32 = 2.;
const GERALD_SIZE: Vec2 = Vec2::new(32., 32.);

#[derive(Component, Default)]
pub struct Gerald {
    pub velocity: Vec2,
    pub grounded: bool,
}

#[derive(Resource)]
struct GeraldSheet {
    idle: Handle<Image>,
}

pub struct GeraldPlugin;
impl Plugin for GeraldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_gerald)
            .add_systems(OnEnter(GameState::Playing), spawn_gerald)
            .add_systems(Update, move_gerald)
            .add_systems(OnExit(GameState::Playing), despawn_with::<Gerald>);
    }
}

fn load_gerald(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let idle: Handle<Image> = asset_server.load("enemy/gerald_front.png");
    loading_assets.0.push(idle.clone().untyped());

    commands.insert_resource(GeraldSheet {idle});
}

fn spawn_gerald(mut commands: Commands, sheet: Res<GeraldSheet>) {
    commands.spawn((
        Sprite::from_image(sheet.idle.clone()),
        Transform::from_xyz(128., LEVEL_BOTTOM + TILE_SIZE + GERALD_SIZE.y / 2., 2.)
            .with_scale(Vec3::splat(SPRITE_SCALE)),
        Gerald::default(),
        Health::new(GERALD_HEALTH),
        MoveSpeed(GERALD_SPEED),
    ));
}

fn move_gerald(
    time: Res<Time>,
    gerald: Single<(&mut Transform, &mut Gerald, &MoveSpeed)>,
) {
    let (mut tf, mut gerald, speed) = gerald.into_inner();
    let dt = time.delta_secs().min(1. / 30.);

    gerald.velocity.x = 10.0;

    tf.translation.x += gerald.velocity.x * dt;
}
