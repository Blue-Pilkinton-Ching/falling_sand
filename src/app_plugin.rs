use bevy::prelude::*;
use iyes_perf_ui::entries::PerfUiBundle;
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start);
        app.add_systems(Update, update);
        app.add_systems(FixedUpdate, fixed_update);
    }
}

fn start(mut commands: Commands) {
    commands.spawn(PerfUiBundle::default());
    commands.spawn(Camera2dBundle::default());
}

fn update() {}

fn fixed_update() {}
