pub const MONSTER_COUNT: usize = 13;

#[derive(Debug,PartialEq,Copy,Clone)]
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

#[derive(Debug,PartialEq,Clone)]
pub struct Monster {
    monster_type: MonsterType,
    hp: usize,
    damage: usize,
    break_weapon: bool,
    has_runestaff: bool,
}

impl Monster {
    pub fn new(monster_num: usize, has_runestaff: bool) -> Monster {
        let monster_type = Monster::get_monster_by_id(monster_num);

        let m1 = monster_num + 1; // Change to 1-based

        let hp = m1 + 2;
        let damage = 1 + m1 / 2;

        let break_weapon = monster_type == MonsterType::Gargoyle || monster_type == MonsterType::Dragon;

        Monster{
            monster_type,
            hp,
            damage,
            break_weapon,
            has_runestaff
        }
    }

    /// Return a MonsterType for a given ID
    fn get_monster_by_id(id: usize) -> MonsterType {
        match id {
            0 => MonsterType::Kobold,
            1 => MonsterType::Orc,
            2 => MonsterType::Wolf,
            3 => MonsterType::Goblin,
            4 => MonsterType::Ogre,
            5 => MonsterType::Troll,
            6 => MonsterType::Bear,
            7 => MonsterType::Minotaur,
            8 => MonsterType::Gargoyle,
            9 => MonsterType::Chimera,
            10 => MonsterType::Balrog,
            11 => MonsterType::Dragon,
            12 => MonsterType::Vendor,
            _ => panic!("get_monster_by_id: unknown id")
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
    pub fn take_damage(&mut self, damage:usize) -> bool {
        if damage < self.hp {
            self.hp -= damage;
            return false;
        }

        self.hp = 0;
        true
    }

    /// Return damage inflicted
    pub fn damage(&self) -> usize {
        self.damage
    }
}