pub use ears::{Sound, AudioController, Music};
use rand::{self, Rng};
use rand::distributions::{IndependentSample, Range};

pub struct Sounds {
    pub intro_music: Music,
    pub background_music: Music,

    assets_path: String,
}

impl Sounds {
    pub fn new(assets_path: &str) -> Self {
        let assets_path = String::from(assets_path);
        let mut background_music = Music::new(&format!("{}/sounds/trolling_doggo_loop.wav", assets_path)).expect("Error on loading trolling_doggo_loop.");
        background_music.set_looping(true);
        Sounds {
            intro_music: Music::new(&format!("{}/sounds/trolling_doggo.wav", assets_path)).expect("Error on loading trolling_doggo."),
            background_music,

            assets_path,
        }
    }

    pub fn angry_meow(&self) -> Sound {
        let mut rng = rand::thread_rng();
        let range = Range::new(1, 4);
        let i = range.ind_sample(&mut rng); 
        match i {
            1 => Sound::new(&format!("{}/sounds/angry_cat_meow_1.wav", self.assets_path)).expect("Error on loading angry_meow_1."),
            2 => Sound::new(&format!("{}/sounds/angry_cat_meow_2.wav", self.assets_path)).expect("Error on loading angry_meow_2."),
            3 => Sound::new(&format!("{}/sounds/angry_cat_meow_3.wav", self.assets_path)).expect("Error on loading angry_meow_3."),
            4 => Sound::new(&format!("{}/sounds/angry_cat_meow_4.wav", self.assets_path)).expect("Error on loading angry_meow_4."),
            _ => Sound::new(&format!("{}/sounds/angry_cat_meow_1.wav", self.assets_path)).expect("Error on loading angry_meow_1."),
        }
    }

    pub fn basic_meow(&self) -> Sound {
        Sound::new(&format!("{}/sounds/basic_cat_meow_1.wav", self.assets_path)).expect("Error on loading basic_meow_1.")
    }

    pub fn kitten_meow(&self) -> Sound {
        Sound::new(&format!("{}/sounds/kitten_meow_1.wav", self.assets_path)).expect("Error on loading kitten_meow_1.")
    }

    pub fn fat_meow(&self) -> Sound {
        Sound::new(&format!("{}/sounds/fat_cat_meow_1.wav", self.assets_path)).expect("Error on loading fat_meow_1.")
    }

    pub fn dog_yip(&self) -> Sound {
        Sound::new(&format!("{}/sounds/dog_yip_1.wav", self.assets_path)).expect("Error on loading dog_yip_1.")
    }

    pub fn dog_woof(&self) -> Sound {
        Sound::new(&format!("{}/sounds/dog_woof_1.wav", self.assets_path)).expect("Error on loading dog_woof_1.")
    }
}
