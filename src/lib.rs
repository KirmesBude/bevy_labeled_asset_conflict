use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use serde::Deserialize;

pub struct ConflictAssetLoaderPlugin;

impl Plugin for ConflictAssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ThisRon>()
            .init_asset::<ThisRonAssetA>()
            .init_asset::<ThisRonAssetB>()
            .init_asset_loader::<ThisRonLoader>();

        app.init_asset::<ThatRon>()
            .init_asset::<ThatRonAssetA>()
            .init_asset::<ThatRonAssetB>()
            .init_asset_loader::<ThatRonLoader>();
    }
}

#[derive(Debug, Reflect, Deserialize)]
pub struct ThisRonRaw {
    pub this_a: ThisRonAssetA,
    pub this_b: ThisRonAssetB,
}

#[derive(Debug, Asset, Reflect)]
pub struct ThisRon {
    pub this_a: Handle<ThisRonAssetA>,
    pub this_b: Handle<ThisRonAssetB>,
}

#[derive(Debug, Asset, Reflect, Deserialize)]
pub struct ThisRonAssetA {
    pub name: String,
}

#[derive(Debug, Asset, Reflect, Deserialize)]
pub struct ThisRonAssetB {
    pub value: u32,
}

#[derive(Debug, Default)]
struct ThisRonLoader;

impl AssetLoader for ThisRonLoader {
    type Asset = ThisRon;
    type Settings = ();
    type Error = ron::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let this_ron_raw = ron::de::from_bytes::<ThisRonRaw>(&bytes)?;

        let this_a_handle = load_context.add_loaded_labeled_asset("a", this_ron_raw.this_a.into());
        let this_b_handle = load_context.add_loaded_labeled_asset("b", this_ron_raw.this_b.into());

        Ok(ThisRon {
            this_a: this_a_handle,
            this_b: this_b_handle,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(Debug, Reflect, Deserialize)]
pub struct ThatRonRaw {
    pub that_a: ThatRonAssetA,
    pub that_b: ThatRonAssetB,
}

#[derive(Debug, Asset, Reflect)]
pub struct ThatRon {
    pub that_a: Handle<ThatRonAssetA>,
    pub that_b: Handle<ThatRonAssetB>,
}

#[derive(Debug, Asset, Reflect, Deserialize)]
pub struct ThatRonAssetA {
    pub flag: bool,
}

#[derive(Debug, Asset, Reflect, Deserialize)]
pub struct ThatRonAssetB {
    pub percent: f32,
}

#[derive(Debug, Default)]
struct ThatRonLoader;

impl AssetLoader for ThatRonLoader {
    type Asset = ThatRon;
    type Settings = ();
    type Error = ron::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let that_ron_raw = ron::de::from_bytes::<ThatRonRaw>(&bytes)?;

        let that_a_handle = load_context.add_loaded_labeled_asset("a", that_ron_raw.that_a.into());
        let that_b_handle = load_context.add_loaded_labeled_asset("b", that_ron_raw.that_b.into());

        Ok(ThatRon {
            that_a: that_a_handle,
            that_b: that_b_handle,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
