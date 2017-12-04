use rand::{self, Rng};
use rand::distributions::{IndependentSample, Range};

use config;
use entities::CAT_COLORS;

const NUM_ITEMS: u32 = 30;

#[derive(Rand)]
pub enum PartyItemKind {
    BasicCat,
    FatCat,
    Kitten,
}

pub struct PartyItem {
    kind: PartyItemKind,
    color: [f32; 3],
    rotation: f32,
}

impl PartyItem {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rotation_range = Range::new(0.0, 359.0);

        PartyItem {
            kind: rng.gen::<PartyItemKind>(),
            color: *rng.choose(CAT_COLORS).unwrap(),
            rotation: rotation_range.ind_sample(&mut rng),
        }
    }
}

pub struct Party {
    party_items: Vec<PartyItem>,
}

impl Party {
    pub fn new() -> Self {
        let mut party_items = Vec::with_capacity(NUM_ITEMS as usize);
        for _ in 0..NUM_ITEMS {
            party_items.push(PartyItem::new());
        }

        Party {
            party_items,
        }
    }
}
