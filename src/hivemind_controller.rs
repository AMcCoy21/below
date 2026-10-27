use bevy::prelude::*;

use crate::{
    GameState, LEVEL_BOTTOM, TILE_SIZE,
    loading::{LoadingAssets, despawn_with},
    stats::Hitbox,
};

const HIVE_SCALE: f32 = 2.;
const HIVE_SIZE: Vec2 = Vec2::new(32., 32.);

#[derive(Component)]
pub struct Hive {
    pub health: f32,
    pub max_health: f32,
}

#[derive(Resource)]
struct HiveAssets {
    image: Handle<Image>,
}

pub struct HivemindControllerPlugin;

impl Plugin for HivemindControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_hive)
            .add_systems(OnEnter(GameState::Playing), spawn_hive)
            .add_systems(
                OnExit(GameState::Playing),
                despawn_with::<Hive>,
            );
    }
}

fn load_hive(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let image = asset_server.load("enemy/hive.png");

    loading_assets.0.push(image.clone().untyped());

    commands.insert_resource(HiveAssets { image });
}

fn spawn_hive(
    mut commands: Commands,
    assets: Res<HiveAssets>,
) {
    commands.spawn((
        Sprite::from_image(assets.image.clone()),

        Transform::from_xyz(256., LEVEL_BOTTOM + TILE_SIZE + HIVE_SIZE.y, 1.)
            .with_scale(Vec3::splat(HIVE_SCALE)),
        Hive {
            health: 100.,
            max_health: 100.,
        },

        Hitbox(HIVE_SIZE),
    ));
}