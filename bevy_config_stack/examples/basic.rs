use bevy::{prelude::*, reflect::TypePath};
use clap::Parser;
use serde::Deserialize;
use bevy_config_stack::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Print the config documentation
    #[arg(long)]
    print_config_docs: bool,
}

/// This config is expected to be in the assets folder under the config folder
#[derive(Asset, Resource,TypePath, Debug, Deserialize, ConfigDocs)]
#[config_path = "config/test.ron"]
struct VehicleConfig {
    /// The visual offset
    visual_offset: Vec3,
    /// Maximum speed of the vehicle in m/s
    max_speed: f32,
    /// Acceleration rate in m/s²
    acceleration: f32,
    /// You can configure each wheel too
    wheels: Vec<WheelConfig>,
}

impl Default for VehicleConfig {
    fn default() -> Self {
        Self {
            visual_offset: Vec3::ZERO,
            max_speed: 100.0,
            acceleration: 10.0,
            wheels: vec![],
        }
    }
}

/// This is a nested config struct that sits inside the VehicleConfig struct
#[derive(Debug, Deserialize, Default, ConfigDocs)]
struct WheelConfig {
    /// Wheel radius in meters
    radius: f32,
}

fn main() {
    let cli = Cli::parse();

    if cli.print_config_docs {
        VehicleConfig::print_docs();
        WheelConfig::print_docs();
        return;
    }

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        }))
        // Config assets are expected to be in RON format under the assets folder.
        // Accepted extensions are: .ron, .config, .cfg (recommend using .ron for syntax highlighting in your IDE)
        .add_plugins(ConfigAssetLoaderPlugin::<VehicleConfig>::default())
        .add_systems(Update, vehicle_config_loaded_event)
        .run();
}

/// 1. An event is fired every time the asset is loaded 
/// or modified at runtime (Bevy's file_watcher feature must be enabled for modified events to work)
/// 2. The resource will always be available, by default it loads the Default value of the config struct.
fn vehicle_config_loaded_event(
    vehicle_config: Res<VehicleConfig>,
    mut config_asset_loaded_event: EventReader<ConfigAssetLoadedEvent<VehicleConfig>>,
) {
    for _ in config_asset_loaded_event.read() {
        info!("Vehicle config (loaded from asset): {:?}", vehicle_config);
    }
}

