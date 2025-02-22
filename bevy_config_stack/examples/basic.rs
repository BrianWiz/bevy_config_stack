use bevy::{prelude::*, reflect::TypePath};
use serde::Deserialize;

use bevy_config_stack::prelude::*;

#[allow(dead_code)]
#[derive(Asset, TypePath, Debug, Deserialize)]
struct VehicleConfig {
    max_speed: f32,
    acceleration: f32,
    turn_speed: f32,
    wheels: Vec<WheelConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WheelConfig {
    radius: f32,
    width: f32,
}

impl Default for VehicleConfig {
    fn default() -> Self {
        Self {
            max_speed: 100.0, 
            acceleration: 10.0, 
            turn_speed: 10.0,
            wheels: vec![],
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        }))
        // Config assets are expected to be in RON format under the assets folder.
        // Accepted extensions are: .ron, .config, .cfg (recommend using .ron for syntax highlighting in your IDE)
        .add_plugins(ConfigAssetLoaderPlugin::<VehicleConfig>::new("config/test.ron"))
        .add_systems(Update, vehicle_config_loaded_event)
        .run();
}

/// An event is fired every time the asset is loaded 
/// or modified at runtime (Bevy's file_watcher feature must be enabled for modified events to work)
fn vehicle_config_loaded_event(
    vehicle_config: Option<Res<Config<VehicleConfig>>>,
    mut config_asset_loaded_event: EventReader<ConfigAssetLoadedEvent<VehicleConfig>>,
) {
    for _ in config_asset_loaded_event.read() {
        if let Some(ref config) = vehicle_config {
            info!("Vehicle config: {:?}", config.data);
        }
    }
}

