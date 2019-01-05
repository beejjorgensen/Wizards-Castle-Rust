#[derive(Debug,PartialEq,Copy,Clone)]
pub enum ArmorType {
    None,
    Leather,
    Chainmail,
    Plate,
}

pub struct Armor {
    armor_type: ArmorType,
    health: u32,
}

impl Armor {

    /// Create a new armor
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

    /// Return the cost in GP of some armor in a given context
    pub fn cost(a:ArmorType, is_vendor:bool) -> u32 {
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

    /// Convert an armor type to its internal ID
    fn to_id(a:ArmorType) -> u32 {
        match a {
            ArmorType::None => 9999,
            ArmorType::Leather => 0,
            ArmorType::Chainmail => 1,
            ArmorType::Plate => 2,
        }
    }

    /// Return protection value of this armor
    pub fn armor_value(&self) -> u32 {
        match self.armor_type {
            ArmorType::None => 0,
            ArmorType::Leather => 1,
            ArmorType::Chainmail => 2,
            ArmorType::Plate => 3,
        }
    }

    /// Return armor type
    pub fn armor_type(&self) -> ArmorType {
        self.armor_type
    }

    /// Damage the armor
    /// 
    /// Return true if the armor is destroyed
    pub fn damage(&mut self, damage:u32) -> bool {

        if damage > self.health {
            self.health = 0;
            return true;
        }

        self.health -= damage;

        false
    }
}
