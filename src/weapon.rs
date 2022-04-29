#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WeaponType {
    None,
    Dagger,
    Mace,
    Sword,
}

pub struct Weapon {
    weapon_type: WeaponType,
}

impl Weapon {
    pub fn new(w: WeaponType) -> Weapon {
        Weapon { weapon_type: w }
    }

    pub fn cost(w: WeaponType, is_vendor: bool) -> u32 {
        if is_vendor {
            match w {
                WeaponType::None => 0,
                WeaponType::Dagger => 1250,
                WeaponType::Mace => 1500,
                WeaponType::Sword => 2000,
            }
        } else {
            match w {
                WeaponType::None => 0,
                WeaponType::Dagger => 10,
                WeaponType::Mace => 20,
                WeaponType::Sword => 30,
            }
        }
    }

    pub fn damage_by_type(w: WeaponType) -> u32 {
        match w {
            WeaponType::None => 0,
            WeaponType::Dagger => 1,
            WeaponType::Mace => 2,
            WeaponType::Sword => 3,
        }
    }

    /// Get a value for comparison
    pub fn get_enum_value(w: WeaponType) -> u32 {
        match w {
            WeaponType::None => 0,
            WeaponType::Dagger => 1,
            WeaponType::Mace => 2,
            WeaponType::Sword => 3,
        }
    }

    pub fn damage(&self) -> u32 {
        Weapon::damage_by_type(self.weapon_type)
    }

    pub fn weapon_type(&self) -> WeaponType {
        self.weapon_type
    }
}
