pub const TREASURE_COUNT:usize = 8;

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum TreasureType {
    RubyRed,
    NornStone,
    PalePearl,
    OpalEye,
    GreenGem,
    BlueFlame,
    Palintir,
    Silmaril,
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Treasure {
    treasure_type: TreasureType,
    max_value: usize,
}

impl Treasure {
    pub fn new(treasure_num: usize) -> Treasure {
        let max_value = (treasure_num + 1) * 1500;

        Treasure {
            treasure_type: Treasure::get_treasure_by_id(treasure_num),
            max_value,
        }
    }

    /// Return a treasure for a given ID
    fn get_treasure_by_id(id: usize) -> TreasureType {
        match id {
            0 => TreasureType::RubyRed,
            1 => TreasureType::NornStone,
            2 => TreasureType::PalePearl,
            3 => TreasureType::OpalEye,
            4 => TreasureType::GreenGem,
            5 => TreasureType::BlueFlame,
            6 => TreasureType::Palintir,
            7 => TreasureType::Silmaril,
            _ => panic!("get_treasure_by_id: unknown id")
        }
    }

    pub fn treasure_type(&self) -> &TreasureType {
        &self.treasure_type
    }
}
