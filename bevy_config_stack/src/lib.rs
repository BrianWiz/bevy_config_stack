mod plugin;
mod load;
pub mod prelude {
    pub use super::ConfigAsset;
    pub use super::plugin::ConfigAssetLoaderPlugin;
    pub use super::plugin::ConfigAssetLoadedEvent;
}

use bevy::prelude::*;
use serde::de::DeserializeOwned;

pub trait ConfigAsset: Asset + Default + Resource + std::fmt::Debug + DeserializeOwned {}
impl<T: Asset + Default + Resource + std::fmt::Debug + DeserializeOwned> ConfigAsset for T {}
