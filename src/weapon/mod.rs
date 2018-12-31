#[derive(Debug,PartialEq,Copy,Clone)]
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
    pub fn new(w:WeaponType) -> Weapon {
        Weapon {
            weapon_type: w,
        }
    }

    pub fn cost(w:WeaponType, is_vendor:bool) -> usize {
        let value;

        if is_vendor {
            value = match w {
                WeaponType::None => 0,
                WeaponType::Dagger => 1250,
                WeaponType::Mace => 1500,
                WeaponType::Sword => 2000,
            }
        } else {
            value = match w {
                WeaponType::None => 0,
                WeaponType::Dagger => 10,
                WeaponType::Mace => 20,
                WeaponType::Sword => 30,
            }
        }

        value
    }

    pub fn damage_by_type(w:WeaponType) -> usize {
        match w {
            WeaponType::None => 0,
            WeaponType::Dagger => 1,
            WeaponType::Mace => 2,
            WeaponType::Sword => 3,
        }
    }

    pub fn damage(&self) -> usize {
        Weapon::damage_by_type(self.weapon_type)
    }

    pub fn weapon_type(&self) -> WeaponType {
        self.weapon_type
    }
}