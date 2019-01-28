pub const MONSTER_COUNT: u32 = 13;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MonsterType {
    Kobold,
    Orc,
    Wolf,
    Goblin,
    Ogre,
    Troll,
    Bear,
    Minotaur,
    Gargoyle,
    Chimera,
    Balrog,
    Dragon,
    Vendor,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Monster {
    monster_type: MonsterType,
    hp: u32,
    damage: u32,
    break_weapon: bool,
    has_runestaff: bool,
    webbed: u32, // How many turns left stuck in a web
}

impl Monster {
    pub fn new(monster_type: MonsterType, has_runestaff: bool) -> Monster {
        let monster_num = Monster::get_monster_num(monster_type);

        let m1 = monster_num + 1; // Change to 1-based

        let hp = m1 + 2;
        let damage = 1 + m1 / 2;

        let break_weapon =
            monster_type == MonsterType::Gargoyle || monster_type == MonsterType::Dragon;

        Monster {
            monster_type,
            hp,
            damage,
            break_weapon,
            has_runestaff,
            webbed: 0,
        }
    }

    /// Return a MonsterType for a given ID
    fn get_monster_num(monster_type: MonsterType) -> u32 {
        match monster_type {
            MonsterType::Kobold => 0,
            MonsterType::Orc => 1,
            MonsterType::Wolf => 2,
            MonsterType::Goblin => 3,
            MonsterType::Ogre => 4,
            MonsterType::Troll => 5,
            MonsterType::Bear => 6,
            MonsterType::Minotaur => 7,
            MonsterType::Gargoyle => 8,
            MonsterType::Chimera => 9,
            MonsterType::Balrog => 10,
            MonsterType::Dragon => 11,
            MonsterType::Vendor => 12,
        }
    }

    /// Return the monster's type
    pub fn monster_type(&self) -> MonsterType {
        self.monster_type
    }

    /// Return true if the monster can break a weapon
    pub fn can_break_weapon(&self) -> bool {
        self.break_weapon
    }

    /// Damage the monster
    ///
    /// Return true if defeated
    pub fn take_damage(&mut self, damage: u32) -> bool {
        if damage < self.hp {
            self.hp -= damage;
            return false;
        }

        self.hp = 0;
        true
    }

    /// Return damage inflicted
    pub fn damage(&self) -> u32 {
        self.damage
    }

    /// True if the monster has the runestaff
    pub fn has_runestaff(&self) -> bool {
        self.has_runestaff
    }

    /// Return monster webbed status
    pub fn webbed(&self) -> &u32 {
        &self.webbed
    }

    /// Set webbed status
    pub fn set_webbed(&mut self, count: u32) {
        self.webbed = count;
    }

    /// Weaken a web, true if broke
    pub fn weaken_web(&mut self) -> bool {
        if self.webbed > 0 {
            self.webbed -= 1;
            if self.webbed == 0 {
                return true;
            }
        }

        false
    }

    /// Return hitpoints
    pub fn hp(&self) -> u32 {
        self.hp
    }
}
