use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

use crate::{
    AppState,
    level::Levels,
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                AsepriteUltraPlugin,
                RonAssetPlugin::<Levels>::new(&["level.ron"]),
            ))
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .continue_to_state(AppState::StartMenu)
                    .load_collection::<GameAssets>()
                    .load_collection::<SfxAssets>()
            )
            .add_systems(OnExit(AppState::Loading), assets_loaded);
    }
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "fonts/Kenney Pixel.ttf")]
    pub _font: Handle<Font>,

    // Menu assets.
    #[asset(path = "ui/start_menu_background.png")]
    pub _start_menu: Handle<Image>,
    #[asset(path = "ui/how_to_play.png")]
    pub how_to_play: Handle<Image>,

    // HUD assets.
    #[asset(path = "ui/cat_face.png")]
    pub _cat_face: Handle<Image>,

    // Game world assets.
    #[asset(path = "level/hardwood_floor.png")]
    pub floor: Handle<Image>,
    #[asset(path = "level/cat_box.png")]
    pub cat_box: Handle<Image>,

    // Doggo!
    #[asset(path = "sprites/wizard_dog.aseprite")]
    pub wizard_dog: Handle<Aseprite>,

    // Cat assets.
    #[asset(path = "sprites/basic_cat.aseprite")]
    pub basic_cat: Handle<Aseprite>,
    #[asset(path = "sprites/fat_cat.aseprite")]
    pub fat_cat: Handle<Aseprite>,
    #[asset(path = "sprites/kitten.aseprite")]
    pub kitten: Handle<Aseprite>,

    #[asset(path = "ui/cat_face.aseprite")]
    pub fox: Handle<Aseprite>,

    // Level data.
    #[asset(path = "all_levels.level.ron")]
    pub levels: Handle<Levels>,
}

#[derive(Resource, AssetCollection)]
pub struct SfxAssets {
    // Dog sounds.
    #[asset(path = "sounds/dog_yip_1.wav")]
    pub dog_yip: Handle<AudioSource>,
    #[asset(path = "sounds/dog_woof_1.wav")]
    pub dog_woof: Handle<AudioSource>,

    // Cat meows.
    #[asset(path = "sounds/basic_cat_meow_1.wav")]
    pub basic_cat_meow: Handle<AudioSource>,
    #[asset(path = "sounds/kitten_meow_1.wav")]
    pub kitten_meow: Handle<AudioSource>,
    #[asset(path = "sounds/fat_cat_meow_1.wav")]
    pub fat_cat_meow: Handle<AudioSource>,

    #[asset(paths(
        "sounds/angry_cat_meow_1.ogg",
        "sounds/angry_cat_meow_2.ogg",
        "sounds/angry_cat_meow_3.ogg",
        "sounds/angry_cat_meow_4.ogg"
    ), collection(typed))]
    pub angry_cat: Vec<Handle<AudioSource>>,

    #[asset(path = "sounds/trolling_doggo.ogg")]
    pub bgm: Handle<AudioSource>,
}

fn assets_loaded(
    mut level_assets: ResMut<Assets<Levels>>,
    audio: Res<Audio>,
    mut assets: ResMut<GameAssets>,
    mut levels: ResMut<Levels>,
    sfx: Res<SfxAssets>,
) {
    debug!("Loaded assets!");

    // Move loaded Levels to a resource and remove the asset/clear the handle.
    if let Some(loaded_levels) = level_assets.remove(&assets.levels) {
        *levels = loaded_levels;
        assets.levels = Handle::default();
    }

    audio.play(sfx.bgm.clone())
        .loop_from(24.0)
        .with_volume(0.2);
}
