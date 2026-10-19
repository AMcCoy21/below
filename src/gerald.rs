use bevy::prelude::*;

use crate :: {
    GameState, LEVEL_BOTTOM, TILE_SIZE,
    combat::HitFlashImage,
    loading::{ LoadingAssets, despawn_with },
    stats::{ GERALD, HitCooldown, Hitbox, MoveSpeed },
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
    hit: Handle<Image>,
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
    let hit: Handle<Image> = asset_server.load("enemy/gerald_hit.png");
    loading_assets.0.push(idle.clone().untyped());
    loading_assets.0.push(hit.clone().untyped());

    commands.insert_resource(GeraldSheet {idle, hit});
}

fn spawn_gerald(mut commands: Commands, sheet: Res<GeraldSheet>) {
    commands.spawn((
        Sprite::from_image(sheet.idle.clone()),
        Transform::from_xyz(128., LEVEL_BOTTOM + TILE_SIZE + GERALD_SIZE.y / 2., 2.)
            .with_scale(Vec3::splat(SPRITE_SCALE)),
        Gerald::default(),
        GERALD.bundle(),
        Hitbox(GERALD_SIZE),
        HitCooldown::default(),
        HitFlashImage {
            normal: sheet.idle.clone(),
            flash: sheet.hit.clone(),
        },
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