use error::Error;
use armor::{Armor, ArmorType};
use weapon::{Weapon, WeaponType};

#[derive(PartialEq)]
pub enum Stat {
    Strength,
    Dexterity,
    Intelligence,
}

#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(PartialEq)]
pub enum Race {
    Hobbit,
    Elf,
    Human,
    Dwarf,
}

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub race: Race,
    pub gender: Gender,

    pub gp: usize,

    pub additional_points: usize,
    pub st: usize,
    pub dx: usize,
    pub iq: usize,

    pub armor: Armor,
    pub weapon: Weapon,
    pub lamp: bool,

    //blind: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 0,
            y: 0,
            z: 0,

            race: Race::Hobbit,
            gender: Gender::Male,

            gp: 0,

            additional_points: 0,

            st: 0,
            dx: 0,
            iq: 0,

            //blind: false

            armor: Armor::new(ArmorType::None),
            weapon: Weapon::new(WeaponType::None),
            lamp: false,
        }
    }

    pub fn set_position(&mut self, x: usize, y: usize, z: usize) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Set the race and all the corresponding points
    pub fn init(&mut self, race: Race) {

        let race_id = Player::get_id_by_race(&race);

        self.st = 2 + (race_id + 1) * 2;
        self.dx = 14 - (race_id + 1) * 2;
        self.iq = 8;

        if race == Race::Hobbit {
            self.additional_points = 4;
        } else {
            self.additional_points = 8;
        }

        self.race = race;

        self.gp = 60;
    }

    /// Get a race number by race type
    fn get_id_by_race(race: &Race) -> usize {
        match race {
            Race::Hobbit => 0,
            Race::Elf => 1,
            Race::Human => 2,
            Race::Dwarf => 3,
        }
    }

    /// Set character gender
    pub fn set_gender(&mut self, gender:Gender) {
        self.gender = gender;
    }

    /// Allocate points to a stat
    /// 
    /// TODO: support deallocation of points
    pub fn allocate_points(&mut self, stat:&Stat, points:usize) -> Result<usize, Error> {
        if points > self.additional_points {
            return Err(Error::NotEnoughPoints);
        }

        match stat {
            Stat::Strength => self.st += points,
            Stat::Dexterity => self.dx += points,
            Stat::Intelligence => self.iq += points,
        };

        self.additional_points -= points;

        Ok(self.additional_points)
    }

    // Give the player some armor
    pub fn purchase_armor(&mut self, a:ArmorType, is_vendor:bool) -> Result<(), Error> {
        let armor_cost = Armor::cost(a, is_vendor);

        if armor_cost > self.gp {
            return Err(Error::NotEnoughGP);
        }

        self.armor = Armor::new(a);

        self.gp -= armor_cost;

        Ok(())
    }

    // Give the player a weapon
    pub fn purchase_weapon(&mut self, w:WeaponType, is_vendor:bool) -> Result<(), Error> {
        let weapon_cost = Weapon::cost(w, is_vendor);

        if weapon_cost > self.gp {
            return Err(Error::NotEnoughGP);
        }

        self.weapon = Weapon::new(w);

        self.gp -= weapon_cost;

        Ok(())
    }

    /// True if the player can afford a lamp
    pub fn can_purchase_lamp(&self) -> bool {
        self.gp >= 20
    }

    /// Purchase a lamp
    pub fn purchase_lamp(&mut self, lamp:bool) -> Result<(), Error> {
        if !self.can_purchase_lamp() {
            return Err(Error::NotEnoughGP);
        }

        self.lamp = lamp;
        self.gp -= 20;

        Ok(())
    }
}