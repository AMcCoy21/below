use bevy::prelude::*;

use crate::{
    GameState,
    loading::despawn_with,
    player::Player,
    stats::{Health, PLAYER_HEALTH},
};

const SEGMENT_COUNT: usize = 3;
const SEGMENT_SIZE: Vec2 = Vec2::new(40., 16.);// size of each segment
const SEGMENT_GAP: f32 = 6.;// gap between segments
const BAR_MARGIN: f32 = 20.;// margin from the edge of the screen
const FULL_COLOR: Color = Color::srgb(1., 0.3, 0.6);// pink
const EMPTY_COLOR: Color = Color::srgb(0.25, 0.15, 0.15);//empty health is dark red

#[derive(Component)]// tag for each segment of the health bar
struct HealthSegment(usize);// index of this segment in the health bar

//the player starts off with full health, so we can spawn the full health bar at the start of the game
//when the player takes damage, we can update the health bar to reflect the new health value.
pub struct HealthBarPlugin;
impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_health_bar.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_health_bar)
            .add_systems(OnExit(GameState::Playing), despawn_with::<HealthBar>);
    }

}
//spawn the health bar when the player spawns, and despawn it when the player dies
fn spawn_health_bar(mut commands: Commands) {
        commands// this spawns the health bar as a node with absolute positioning, and add the HealthBar component to it
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(BAR_MARGIN),
                top: Val::Px(BAR_MARGIN),
                column_gap: Val::Px(SEGMENT_GAP),//spacing for the segments
                ..default()
            },
            HealthBar,
        ))
        // this spawns each segment of the health bar as a child of the health bar node
        .with_children(|bar| {
            for i in 0..SEGMENT_COUNT {
                bar.spawn((
                    Node {
                        width: Val::Px(SEGMENT_SIZE.x),
                        height: Val::Px(SEGMENT_SIZE.y),
                        ..default()
                    },
                    BackgroundColor(FULL_COLOR),
                    HealthSegment(i),
                ));
            }
    });
}

fn update_health_bar(
    player: Single<&Health, With<Player>>,
    mut segments: Query<(&HealthSegment, &mut BackgroundColor)>,
)
{
    let per_segment = PLAYER_HEALTH / SEGMENT_COUNT as f32;
    for (segment, mut color) in &mut segments {
        let filled = player.current > segment.0 as f32 * per_segment;// if the player's current health is greater than the health represented by this segment, it should be filled
        color.0 = if filled { FULL_COLOR } else { EMPTY_COLOR };// this will set the color of the segment based on whether it should be filled or empty

    }
}