use ears::Sound;

pub struct Sounds {
    pub basic_meow: Sound,
    pub fat_meow: Sound,
    pub kitten_meow: Sound,
    pub angry_meow_1: Sound,
    pub angry_meow_2: Sound,
    pub angry_meow_3: Sound,
    pub angry_meow_4: Sound,
}

impl Sounds {
    pub fn new() -> Self {
        Sounds {
            basic_meow: Sound::new("assets/sounds/basic_cat_meow_1.wav").expect("Error on loading basic_meow_1."),
            fat_meow: Sound::new("assets/sounds/fat_cat_meow_1.wav").expect("Error on loading fat_meow_1."),
            kitten_meow: Sound::new("assets/sounds/kitten_meow_1.wav").expect("Error on loading kitten_meow_1."),
            angry_meow_1: Sound::new("assets/sounds/angry_cat_meow_1.wav").expect("Error on loading angry_meow_1."),
            angry_meow_2: Sound::new("assets/sounds/angry_cat_meow_2.wav").expect("Error on loading angry_meow_2."),
            angry_meow_3: Sound::new("assets/sounds/angry_cat_meow_3.wav").expect("Error on loading angry_meow_3."),
            angry_meow_4: Sound::new("assets/sounds/angry_cat_meow_4.wav").expect("Error on loading angry_meow_4."),
        }
    }
}