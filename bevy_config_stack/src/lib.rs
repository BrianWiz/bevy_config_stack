mod plugin;
mod load;
pub mod prelude {
    pub use super::ConfigAsset;
    pub use super::plugin::ConfigAssetLoaderPlugin;
    pub use super::plugin::ConfigAssetLoadedEvent;
    pub use super::plugin::Config;
}

use bevy::prelude::*;
use serde::de::DeserializeOwned;

pub trait ConfigAsset: Asset + Default + std::fmt::Debug + DeserializeOwned {}
impl<T: Asset + Default + std::fmt::Debug + DeserializeOwned> ConfigAsset for T {}
