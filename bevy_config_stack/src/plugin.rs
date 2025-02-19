use bevy::prelude::*;
use crate::{ConfigAsset, ConfigAssetLoader};

#[derive(Resource)]
pub struct Config<TAsset: ConfigAsset> {
    pub data: TAsset,
}

#[derive(Resource)]
struct ConfigState<TAsset: ConfigAsset> {
    handle: Handle<TAsset>,
    name: String,
    path: String,
}

#[derive(Event)]
pub struct ConfigAssetLoadedEvent<TAsset: ConfigAsset> {
    pub name: String,
    pub path: String,
    _marker: std::marker::PhantomData<TAsset>,
}

#[derive(Event)]
pub struct UnloadConfigAsset<TAsset: ConfigAsset> {
    _marker: std::marker::PhantomData<TAsset>,
}

pub struct ConfigAssetLoaderPlugin<T: ConfigAsset> {
    name: String,
    path: String,
    _marker: std::marker::PhantomData<T>,
}

impl<TAsset: ConfigAsset> Default for ConfigAssetLoaderPlugin<TAsset> {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            path: "".to_string(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TAsset: ConfigAsset> Plugin for ConfigAssetLoaderPlugin<TAsset> {
    fn build(&self, app: &mut App) {
        app.init_asset::<TAsset>();
        app.init_asset_loader::<ConfigAssetLoader<TAsset>>();
        app.insert_resource(ConfigState::<TAsset> {
            handle: Handle::<TAsset>::default(),
            name: self.name.clone(),
            path: self.path.clone(),
        });
        app.add_event::<ConfigAssetLoadedEvent<TAsset>>();
        app.add_event::<UnloadConfigAsset<TAsset>>();
        app.add_systems(Startup, Self::setup_system);
        app.add_systems(Update, (
            Self::load_watcher_system,
        ));
    }
}

impl<TAsset: ConfigAsset> ConfigAssetLoaderPlugin<TAsset> {
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            _marker: std::marker::PhantomData,
        }
    }

    fn setup_system(mut state: ResMut<ConfigState<TAsset>>, asset_server: Res<AssetServer>) {
        state.handle = asset_server.load(state.path.clone());
    }
    
    fn load_watcher_system(
        mut commands: Commands,
        state: Res<ConfigState<TAsset>>,
        mut assets: ResMut<Assets<TAsset>>,
        mut asset_events: EventReader<AssetEvent<TAsset>>,
        mut config_asset_loaded_event: EventWriter<ConfigAssetLoadedEvent<TAsset>>,
    ) {
        for event in asset_events.read() {
            match event {
                AssetEvent::LoadedWithDependencies { id } => {
                    if id == &state.handle.id() {
                        if let Some(data) = assets.remove(state.handle.id()) {
                            info!("Config asset '{}' loaded from {}", state.name, state.path);
                            commands.insert_resource(Config { data });
                            config_asset_loaded_event.send(ConfigAssetLoadedEvent {
                                name: state.name.clone(),
                                path: state.path.clone(),
                                _marker: std::marker::PhantomData,
                            });
                        }
                    }
                    break;
                }
                AssetEvent::Modified { id } => {
                    if id == &state.handle.id() {
                        if let Some(data) = assets.remove(state.handle.id()) {
                            info!("Config asset '{}' modified from {}", state.name, state.path);
                            commands.insert_resource(Config { data });
                            config_asset_loaded_event.send(ConfigAssetLoadedEvent {
                                name: state.name.clone(),
                                path: state.path.clone(),
                                _marker: std::marker::PhantomData,
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
