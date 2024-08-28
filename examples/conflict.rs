use bevy::prelude::*;
use bevy_labeled_asset_conflict::{ConflictAssetLoaderPlugin, ThatRonAssetB, ThisRonAssetA};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ConflictAssetLoaderPlugin)
        .init_resource::<MyAssetHandles>()
        .add_systems(Startup, setup)
        .add_systems(Update, print_assets)
        .run();
}

#[derive(Resource, Default)]
pub struct MyAssetHandles {
    this_handle: Handle<ThisRonAssetA>,
    that_handle: Handle<ThatRonAssetB>,
}

fn setup(asset_server: Res<AssetServer>, mut my_asset_handles: ResMut<MyAssetHandles>) {
    let this = asset_server.load("this.ron#a");
    let that = asset_server.load("that.ron#b");

    my_asset_handles.this_handle = this;
    my_asset_handles.that_handle = that;
}

fn print_assets(mut done: Local<bool>, my_asset_handles: Res<MyAssetHandles>, this_ron: Res<Assets<ThisRonAssetA>>, that_ron: Res<Assets<ThatRonAssetB>>) {
    if !*done {
        if let (Some(this), Some(that)) = (this_ron.get(&my_asset_handles.this_handle), that_ron.get(&my_asset_handles.that_handle)) {
            println!("ThisRonA: {:?}", this);
            println!("ThatRonB: {:?}", that);
            *done = true;
        }
    }

}