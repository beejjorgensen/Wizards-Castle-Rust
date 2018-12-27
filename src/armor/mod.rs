#[derive(Debug,PartialEq,Copy,Clone)]
pub enum ArmorType {
    None,
    Leather,
    Chainmail,
    Plate,
}

pub struct Armor {
    armor_type: ArmorType,
    health: isize,
}

impl Armor {
    pub fn new(a: ArmorType) -> Armor {
        let health = match a {
            ArmorType::None => 0 * 7,
            ArmorType::Leather => 1 * 7,
            ArmorType::Chainmail => 2 * 7,
            ArmorType::Plate => 3 * 7,
        };

        Armor {
            armor_type: a,
            health,
        }
    }

    pub fn cost(a:ArmorType, is_vendor:bool) -> usize {
        let value;

        if is_vendor {
            value = match a {
                ArmorType::None => 0,
                ArmorType::Leather => 1250,
                ArmorType::Chainmail => 1500,
                ArmorType::Plate => 2000,
            }

        } else {
            if a == ArmorType::None {
                value = 0;

            } else {
                let id = Armor::to_id(a);

                value = (id + 1) * 10;
            }
        }

        value
    }

    fn to_id(a:ArmorType) -> usize {
        match a {
            ArmorType::None => 9999,
            ArmorType::Leather => 0,
            ArmorType::Chainmail => 1,
            ArmorType::Plate => 2,
        }
    }

    pub fn armor_type(&self) -> ArmorType {
        self.armor_type
    }
}
