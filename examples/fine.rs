use bevy::prelude::*;
use bevy_labeled_asset_conflict::{ConflictAssetLoaderPlugin, ThatRon, ThisRon};

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
    this_handle: Handle<ThisRon>,
    that_handle: Handle<ThatRon>,
}

fn setup(asset_server: Res<AssetServer>, mut my_asset_handles: ResMut<MyAssetHandles>) {
    let this = asset_server.load("this.ron");
    let that = asset_server.load("that.ron");

    my_asset_handles.this_handle = this;
    my_asset_handles.that_handle = that;
}

fn print_assets(mut done: Local<bool>, my_asset_handles: Res<MyAssetHandles>, this_ron: Res<Assets<ThisRon>>, that_ron: Res<Assets<ThatRon>>) {
    if !*done {
        if let (Some(this), Some(that)) = (this_ron.get(&my_asset_handles.this_handle), that_ron.get(&my_asset_handles.that_handle)) {
            println!("ThisRon: {:?}", this);
            println!("ThatRon: {:?}", that);
            *done = true;
        }
    }

}