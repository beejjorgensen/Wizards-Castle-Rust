use error::Error;
use armor::{Armor, ArmorType};
use weapon::{Weapon, WeaponType};
use treasure::TreasureType;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Stat {
    Strength,
    Dexterity,
    Intelligence,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Race {
    Hobbit,
    Elf,
    Human,
    Dwarf,
}

pub struct Player {
    x: u32,
    y: u32,
    z: u32,

    race: Race,
    gender: Gender,

    gp: u32,

    additional_points: u32,
    pub st: u32, // TODO make these a map
    pub dx: u32,
    pub iq: u32,

    pub armor: Armor,
    pub weapon: Weapon,
    pub lamp: bool,
    pub treasures: Vec<TreasureType>,

    pub runestaff: bool,
    pub orb_of_zot: bool,

    pub flares: u32,

    pub blind: bool,
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

            blind: false,

            armor: Armor::new(ArmorType::None),
            weapon: Weapon::new(WeaponType::None),
            lamp: false,
            treasures: Vec::new(),

            orb_of_zot: false,
            runestaff: false,

            flares: 0,
        }
    }

    /// Set player position
    pub fn set_position(&mut self, x: u32, y: u32, z: u32) {
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

        self.flares = 0;

        self.treasures.clear();
    }

    /// Get a race number by race type
    fn get_id_by_race(race: &Race) -> u32 {
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
    pub fn allocate_points(&mut self, stat:&Stat, points:u32) -> Result<u32, Error> {
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

    pub fn add_stat(&mut self, stat:&Stat, points:u32) -> u32 {
        let new_total;

        match stat {
            Stat::Strength => {
                self.st += points;
                self.st = std::cmp::min(18, self.st);
                new_total = self.st;
            },
            Stat::Dexterity => {
                self.dx += points;
                self.dx = std::cmp::min(18, self.dx);
                new_total = self.dx;
            },
            Stat::Intelligence => {
                self.iq += points;
                self.iq = std::cmp::min(18, self.iq);
                new_total = self.iq;
            },
        };

        new_total
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

        if lamp {
            self.gp -= 20;
        }

        Ok(())
    }

    /// Return the max number of flares a player can afford
    pub fn max_flares(&self) -> u32 {
        self.gp
    }

    /// Purchase flares
    pub fn purchase_flares(&mut self, flares:u32) -> Result<(), Error> {
        if flares > self.max_flares() {
            return Err(Error::NotEnoughGP);
        }

        self.flares += flares;
        self.gp -= flares;

        Ok(())
    }

    pub fn is_blind(&self) -> bool {
        self.blind
    }

    /// Return a player stat
    pub fn stat(&self, stat:Stat) -> u32 {
        match stat {
            Stat::Strength => self.st,
            Stat::Intelligence => self.iq,
            Stat::Dexterity => self.dx,
        }
    }

    /// Return flare count
    pub fn flares(&self) -> u32 {
        self.flares
    }

    /// Return number of gold pieces
    pub fn gp(&self) -> &u32 {
        &self.gp
    }

    /// Add gold pieces
    pub fn add_gp(&mut self, amount: u32) {
        self.gp += amount;
    }

    /// Return player's weapon
    pub fn weapon(&self) -> &Weapon {
        &self.weapon
    }

    /// Return player's armor
    pub fn armor(&self) -> &Armor {
        &self.armor
    }

    /// Return player's armor mutably
    pub fn armor_mut(&mut self) -> &mut Armor {
        &mut self.armor
    }

    /// True if the player has a lamp
    pub fn has_lamp(&self) -> bool {
        self.lamp
    }

    /// Damage the player
    ///
    /// Returns true if the player has died
    pub fn damage_st(&mut self, damage:u32) -> bool {
        let defeated;

        if damage >= self.st {
            self.st = 0;
            defeated = true;
        } else {
            self.st -= damage;
            defeated = false;
        }

        defeated
    }

    /// Damage armor
    /// 
    /// Return true if the armor is destroyed
    pub fn damage_armor(&mut self, damage:u32) -> bool {
        let armor_destroyed = self.armor_mut().damage(damage) && self.armor().armor_type() != ArmorType::None;

        if armor_destroyed {
            self.armor = Armor::new(ArmorType::None);
        }

        armor_destroyed
    }

    /// Returns true if the player is dead
    pub fn is_dead(&self) -> bool {
        self.st == 0 || self.iq == 0 || self.dx == 0
    }

    /// True if the player has the Orb of Zot
    pub fn has_orb_of_zot(&self) -> bool {
        self.orb_of_zot
    }

    /// Give the Orb of Zot to the player
    pub fn give_orb_of_zot(&mut self, has: bool) {
        self.orb_of_zot = has;
    }

    /// True if the player has the Runestaff
    pub fn has_runestaff(&self) -> bool {
        self.runestaff
    }

    /// Give the runestaff to the player
    pub fn give_runestaff(&mut self, has: bool) {
        self.runestaff = has;
    }

    /// Return a list of players treasures
    pub fn get_treasures(&self) -> &Vec<TreasureType> {
        &self.treasures
    }

    /// Remove a specific treasure from the list
    /// 
    /// Returns true on success (if the player had the treasure)
    pub fn remove_treasure(&mut self, treasure_type:TreasureType) -> bool {

        // Find the element
        match self.treasures.iter().position(|&t| t == treasure_type) {
            Some(i) => {
                self.treasures.remove(i);
                true
            },
            None => false
        }
    }

    // Spend some GP
    pub fn spend(&mut self, amount:u32) -> Result<(), Error> {
        if amount > self.gp {
            Err(Error::NotEnoughGP)
        } else {
            self.gp -= amount;
            Ok(())
        }
    }

    // Go down
    pub fn down(&mut self) -> u32 {
        self.z += 1;

        self.z
    }

    // Go up
    pub fn up(&mut self) -> u32 {
        self.z -= 1;

        self.z
    }

    // Get X coord
    pub fn x(&self) -> &u32 {
        &self.x
    }

    // Get Y coord
    pub fn y(&self) -> &u32 {
        &self.y
    }

    // Get Z coord
    pub fn z(&self) -> &u32 {
        &self.z
    }

    // Set X coord
    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    // Set Y coord
    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    // Set Z coord
    pub fn set_z(&mut self, z: u32) {
        self.z = z;
    }

    // Get race
    pub fn race(&self) -> &Race {
        &self.race
    }

    // Get additional stat points
    pub fn additional_points(&self) -> &u32 {
        &self.additional_points
    }
}