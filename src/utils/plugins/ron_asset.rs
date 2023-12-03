use std::marker::PhantomData;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt as _;
use bevy::prelude::*;
use bevy::utils::BoxedFuture;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Default)]
pub struct RonLoader<T, const E: &'static str>(PhantomData<T>);

impl<T: Asset + DeserializeOwned, const E: &'static str> AssetLoader for RonLoader<T, E> {
    type Asset = T;
    type Settings = ();
    type Error = RonLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a (),
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<Self::Asset>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RonLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not read file: {0}")]
    Io(#[from] std::io::Error),
    
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}
