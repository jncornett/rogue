use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

pub fn tiled_map_loader_plugin(app: &mut App) {
    app.init_asset::<TiledMap>()
        .init_asset_loader::<TiledMapLoader>();
}

#[derive(Asset, Reflect, Debug, Deserialize)]
pub struct TiledMap(pub u8, pub Vec<u8>);

#[derive(Default)]
pub struct TiledMapLoader;

impl AssetLoader for TiledMapLoader {
    type Asset = TiledMap;

    type Settings = ();

    type Error = TiledMapLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = toml::from_slice(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}

#[derive(Error, Debug)]
pub enum TiledMapLoadError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML deserialization error: {0}")]
    Toml(#[from] toml::de::Error),
}
