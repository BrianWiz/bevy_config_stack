use bevy::{asset::{io::Reader, AssetLoader, LoadContext}, prelude::*};
use thiserror::Error;

use crate::ConfigAsset;

#[derive(Default)]
pub struct ConfigAssetLoader<T: ConfigAsset> {
    _marker: std::marker::PhantomData<T>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl<T: ConfigAsset> AssetLoader for ConfigAssetLoader<T> {
    type Asset = T;
    type Settings = ();
    type Error = CustomAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<T>(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron", "config", "cfg"]
    }
}
