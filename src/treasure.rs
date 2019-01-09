pub const TREASURE_COUNT: u32 = 8;

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum TreasureType {
    RubyRed,
    NornStone,
    PalePearl,
    OpalEye,
    GreenGem,
    BlueFlame,
    Palantir,
    Silmaril,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Treasure {
    pub treasure_type: TreasureType,
}

impl Treasure {
    pub fn new(treasure_num: u32) -> Treasure {
        Treasure {
            treasure_type: Treasure::get_treasure_by_id(treasure_num),
        }
    }

    /// Return a treasure for a given ID
    fn get_treasure_by_id(id: u32) -> TreasureType {
        match id {
            0 => TreasureType::RubyRed,
            1 => TreasureType::NornStone,
            2 => TreasureType::PalePearl,
            3 => TreasureType::OpalEye,
            4 => TreasureType::GreenGem,
            5 => TreasureType::BlueFlame,
            6 => TreasureType::Palantir,
            7 => TreasureType::Silmaril,
            _ => panic!("get_treasure_by_id: unknown id")
        }
    }

    /// Return treasure identifying number byte type
    fn get_treasure_num_by_type(treasure_type: TreasureType) -> u32 {
        match treasure_type {
            TreasureType::RubyRed => 0,
            TreasureType::NornStone => 1,
            TreasureType::PalePearl => 2,
            TreasureType::OpalEye => 3,
            TreasureType::GreenGem => 4,
            TreasureType::BlueFlame => 5,
            TreasureType::Palantir => 6,
            TreasureType::Silmaril => 7,
        }
    }

    /// Return maximum value for a treasure
    pub fn treasure_max_value(treasure_type: TreasureType) -> u32 {
        let num = Treasure::get_treasure_num_by_type(treasure_type);

        (num + 1) * 1500
    }

    /// Return TreasureType for this Treasure
    pub fn treasure_type(&self) -> &TreasureType {
        &self.treasure_type
    }
}
