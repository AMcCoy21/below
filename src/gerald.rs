use bevy::prelude::*;
use rand::Rng;

use crate :: {
    GameState, LEVEL_BOTTOM, TILE_SIZE,
    combat::HitFlashImage,
    loading::{ LoadingAssets, despawn_with },
    stats::{ GERALD, GERALD_SPEED, HitCooldown, Hitbox, MoveSpeed },
    player::Player,
};

const SPRITE_SCALE: f32 = 2.;
const GERALD_SIZE: Vec2 = Vec2::new(32., 32.);

#[derive(Component, Default)]
pub struct Gerald {
    pub velocity: Vec2,
    pub grounded: bool,
    pub attack_timer: f32,
}

#[derive(Resource)]
struct GeraldSheet {
    sprite_sheet: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    hit: Handle<Image>,
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum GeraldState {
    #[default]
    Idle,
    Patrol,
    Chase,
    Attack,
}

#[derive(Component, Default)]
pub enum GeraldDirection {
    #[default]
    Front,
    Right,
    Back,
    Left,
}

#[derive(Component)]
pub struct GeraldPatrol {
    pub distance_remaining: f32,
    pub direction: f32,
}

pub struct GeraldPlugin;
impl Plugin for GeraldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_gerald)
            .add_systems(OnEnter(GameState::Playing), spawn_gerald)
            .add_systems(Update, (
                gerald_state_change, 
                gerald_ai,
                update_gerald_sprite,
            ))
            .add_systems(OnExit(GameState::Playing), despawn_with::<Gerald>);
    }
}

fn load_gerald(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let sprite_sheet: Handle<Image> = asset_server.load("enemy/gerald_idle_sheet.png");
    let hit: Handle<Image> = asset_server.load("enemy/gerald_hit.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32), // Frame dimensions
        4,                  // Columns
        1,                  // Rows
        None,
        None,
    );
    let layout = texture_atlas_layouts.add(layout);

    loading_assets.0.push(sprite_sheet.clone().untyped());
    loading_assets.0.push(hit.clone().untyped());

    commands.insert_resource(GeraldSheet {sprite_sheet, layout, hit});
}

fn spawn_gerald(mut commands: Commands, sheet: Res<GeraldSheet>) {
    commands.spawn((
        Sprite::from_atlas_image(
            sheet.sprite_sheet.clone(),
            TextureAtlas {
                layout: sheet.layout.clone(),
                index: 1,
            },
        ),
        // What location he spawns at. It's currently hard-coded so he spawns 128 units away on x and standing on the floor.
        Transform::from_xyz(128., LEVEL_BOTTOM + TILE_SIZE + GERALD_SIZE.y / 2., 2.)
            .with_scale(Vec3::splat(SPRITE_SCALE)),
        Gerald::default(),
        GERALD.bundle(),
        Hitbox(GERALD_SIZE),
        HitCooldown::default(),
        HitFlashImage {
            normal: sheet.sprite_sheet.clone(),
            flash: sheet.hit.clone(),
        },
        GeraldDirection::default(),
        GeraldPatrol {
            distance_remaining: 0.,
            direction: 1.,
        },
        GeraldState::default(),
    ));
}

fn update_gerald_sprite(
    mut query: Query<(&GeraldDirection, &mut Sprite)>,
) {
    for (state, mut sprite) in &mut query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = match state {
                GeraldDirection::Front => 0,
                GeraldDirection::Right => 1,
                GeraldDirection::Back => 2,
                GeraldDirection::Left => 3,
            };
        }
    }
}

fn gerald_ai(
    time: Res<Time>,
    player: Single<&Transform, With<Player>>,
    mut gerald: Single<(
        &mut Transform,
        &mut Gerald,
        &mut GeraldDirection,
        &mut GeraldPatrol,
        &mut GeraldState,
    ),
        Without<Player>,
    >,
) {
    let (mut tf, mut gerald, mut direction, mut patrol, state) = gerald.into_inner();
    let dt = time.delta_secs().min(1. / 30.);

    match *state {
        // Idle - Gerald wil stand still. Unlikely that he ever has to go into this state.
        GeraldState::Idle => {
            gerald.velocity = Vec2::ZERO;
            *direction = GeraldDirection::Front;
        }
        // Patrol - Gerald walks back and forth a random amount of blocks.
        // Collison detection not implemented yet, but Gerald will avoid face-planting into the walls
        GeraldState::Patrol => {
            // Regenerates the patrol with a new distance
            if patrol.distance_remaining <= 0. {
                patrol.direction *= -1.;
                let mut rng = rand::thread_rng();

                // Gets a random distance between 2 and 8 tiles.
                patrol.distance_remaining = rng.gen_range(2.0..=8.0) * TILE_SIZE as f32;
            }

            if patrol.direction > 0. {
                *direction = GeraldDirection::Right;
            } else {
                *direction = GeraldDirection::Left;
            }

            let movement = (GERALD_SPEED * dt).min(patrol.distance_remaining);

            gerald.velocity.x = patrol.direction * GERALD_SPEED;
            tf.translation.x += patrol.direction * movement;
            patrol.distance_remaining -= movement;
        }

        // Chase - Gerald will follow player to where he sees them.
        // TODO: Can either be triggered by sight (near) or sound (further)
        GeraldState::Chase => {
            let distance = player.translation.x - tf.translation.x;

            if distance > 0. {
                gerald.velocity.x = GERALD_SPEED;
                *direction = GeraldDirection::Right;
            } else {
                gerald.velocity.x = -GERALD_SPEED;
                *direction = GeraldDirection::Left;
            }

            // Gerald will go 1.5 times faster when he's chasing
            tf.translation.x += gerald.velocity.x * dt * 1.5;
        }

        // Attack - Gerald will attack the player. riveting news.
        // He'll also spin in circles which looks funny but will more likely than not be completely temporary.
        GeraldState::Attack => {
            let distance = player.translation.x - tf.translation.x;
            gerald.velocity.x = 0.;

           gerald.attack_timer += dt;

            if gerald.attack_timer >= 0.08 {
                gerald.attack_timer = 0.;

                *direction = match *direction {
                    GeraldDirection::Front => GeraldDirection::Right,
                    GeraldDirection::Right => GeraldDirection::Back,
                    GeraldDirection::Back => GeraldDirection::Left,
                    GeraldDirection::Left => GeraldDirection::Front,
                };
            }
        }
    }
}

// Temporary state changing for demo
fn gerald_state_change(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gerald: Single<&mut GeraldState>,
) {
    if keyboard.just_pressed(KeyCode::KeyU) {
        **gerald = GeraldState::Idle;
    }

    if keyboard.just_pressed(KeyCode::KeyI) {
        **gerald = GeraldState::Patrol;
    }

    if keyboard.just_pressed(KeyCode::KeyO) {
        **gerald = GeraldState::Chase;
    }

    if keyboard.just_pressed(KeyCode::KeyP) {
        **gerald = GeraldState::Attack;
    }
}
