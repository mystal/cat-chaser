use bevy::prelude::*;
use bevy::render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
use bevy_asset_loader::prelude::*;

use crate::{
    AppState,
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .continue_to_state(AppState::Playing)
                    .load_collection::<GameAssets>()
            )
            .add_systems(OnExit(AppState::Loading), assets_loaded);
    }
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "fonts/Kenney Pixel.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "start_menu_background.png")]
    pub start_menu: Handle<Image>,
    #[asset(path = "how_to_play.png")]
    pub how_to_play: Handle<Image>,
    #[asset(path = "hardwood_floor.png")]
    pub floor: Handle<Image>,
    #[asset(path = "cat_box.png")]
    pub cat_box: Handle<Image>,
}

fn assets_loaded(
    assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
) {
    debug!("Loaded assets!");

    // Set repeat address mode on tiling textures.
    if let Some(image) = images.get_mut(&assets.floor) {
        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        });
    }
}
