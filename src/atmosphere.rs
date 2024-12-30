use crate::*;
use bevy::{pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};

// Marker for updating the position of the light, not needed unless we have multiple lights
#[derive(Component)]
struct Sun;

// Timer for updating the daylight cycle (updating the atmosphere every frame is slow, so it's better to do incremental changes)
#[derive(Resource)]
struct CycleTimer(Timer);

// We can edit the Atmosphere resource and it will be updated automatically
fn daylight_cycle(
    mut atmosphere: bevy_atmosphere::prelude::AtmosphereMut<bevy_atmosphere::prelude::Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let t = time.elapsed_secs_wrapped() / 2.0;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
        }
    }
}

// Simple environment
fn setup_atmosphere(mut commands: Commands) {
    // Our Sun
    commands.spawn((
        DirectionalLight::default(),
        Sun, // Marks the light as Sun
    ));
}

pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(bevy_atmosphere::model::AtmosphereModel::default()) // Default Atmosphere material, we can edit it to simulate another planet
            .insert_resource(CycleTimer(Timer::new(
                bevy::utils::Duration::from_secs(DAY_NIGHT_CYCLE), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
                TimerMode::Repeating,
            )))
            .add_plugins(bevy_atmosphere::plugin::AtmospherePlugin) // Default AtmospherePlugin
            .add_systems(Startup, setup_atmosphere)
            .add_systems(Update, daylight_cycle);
    }
}
