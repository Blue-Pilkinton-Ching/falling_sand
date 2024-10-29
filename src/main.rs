use bevy::prelude::*;
use iyes_perf_ui::PerfUiPlugin;

use app_plugin::AppPlugin;

pub mod app_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(AppPlugin)
        .run();
}
