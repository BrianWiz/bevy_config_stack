mod plugin;
pub(crate) mod load;

pub mod prelude {
    pub use super::ConfigAsset;
    pub use super::plugin::ConfigAssetLoaderPlugin;
    pub use super::plugin::ConfigAssetLoadedEvent;
    pub use bevy_config_docs::ConfigDocs;
}

use bevy::prelude::*;
use serde::de::DeserializeOwned;

pub trait ConfigAsset: Asset + Default + Resource+ std::fmt::Debug + DeserializeOwned {
    const CONFIG_PATH: &'static str = "";
}

pub trait ConfigDocs {
    fn print_docs();
    fn config_path() -> &'static str {
        ""
    }
}
