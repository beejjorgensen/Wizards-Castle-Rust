extern crate rand;

use crate::armor::ArmorType;
use crate::curse::CurseType;
use crate::dungeon::Dungeon;
use crate::error::Error;
use crate::monster::{Monster, MonsterType};
use crate::player::{Gender, Player, Race, Stat};
use crate::room::{Room, RoomType};
use crate::treasure::{Treasure, TreasureType};
use crate::weapon::{Weapon, WeaponType};

use self::rand::thread_rng;
use self::rand::Rng;

#[derive(Debug, Clone)]
pub enum Event {
    None,
    FoundGold(u32),
    FoundFlares(u32),
    Sinkhole,
    Warp,
    Treasure(Treasure),
    Combat(MonsterType),
    Vendor,
}

#[derive(Debug, Clone, Copy)]
pub enum CombatEvent {
    NoWeapon,
    BookHands,
    Miss,
    Hit(HitResult),
    Died,
    MonsterWebbed,
    MonsterMiss,
    MonsterHit(u32, bool, bool, bool),
}

#[derive(Debug, Clone, Copy)]
pub struct HitResult {
    pub damage: u32,
    pub broke_weapon: bool,
    pub defeated: bool,
    pub treasure: u32,
    pub got_runestaff: bool,
    pub killed_vendor: bool,
    pub got_lamp: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum DrinkEvent {
    Stronger,
    Weaker,
    Smarter,
    Dumber,
    Nimbler,
    Clumsier,
    ChangeRace,
    ChangeGender,
}

#[derive(Debug, Clone)]
pub enum OrbEvent {
    BloodyHeap,
    Polymorph(MonsterType),
    GazeBack(MonsterType),
    Item(RoomType, u32, u32, u32),
    OrbOfZot(u32, u32, u32),
    SoapOpera,
}

#[derive(Debug, Clone, Copy)]
pub enum ChestEvent {
    Explode,
    Gas,
    Treasure(u32),
}

#[derive(Debug, Clone, Copy)]
pub enum BookEvent {
    Blind,
    Poetry,
    PlayMonster(MonsterType),
    Dexterity,
    Strength,
    Sticky,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AttackType {
    Melee,
    Fireball,
    Deathspell,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stairs {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RandomMessage {
    None,
    SeeBat,
    HearSound,
    Sneeze,
    StepFrog,
    MonsterFrying,
    Watched,
    Playing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Init,

    Move,

    Vendor,
    VendorAttack, // True just after a player has initiated an attack on a vendor

    PlayerAttack,
    MonsterAttack,
    Retreat,

    Warp,
    Sinkhole,
    Gas,

    Dead,
    Exit,
    Quit,
}

pub struct Game {
    dungeon: Dungeon,
    player: Player,

    state: GameState,

    prev_dir: Direction,

    currently_fighting: Option<Monster>,
    bribe_possible: bool,
    bribe_treasure: Option<TreasureType>,
    retreating: bool,

    spell_possible: bool,

    vendors_angry: bool,
    vendor_treasure_price: u32,
    vendor_treasure: Option<TreasureType>,

    turn: u32,
    last_recipe_turn: u32,

    lethargic: bool,
}

impl Game {
    pub fn new(xsize: u32, ysize: u32, zsize: u32) -> Game {
        let dungeon = Dungeon::new(xsize, ysize, zsize);

        let mut player = Player::new();
        player.set_position(dungeon.entrance_x(), 0, 0);

        Game {
            dungeon,
            player,
            state: GameState::Init,
            prev_dir: Direction::South,
            currently_fighting: None,
            bribe_possible: true,
            bribe_treasure: None,
            retreating: false,
            spell_possible: false,
            vendors_angry: false,
            vendor_treasure_price: 0,
            vendor_treasure: None,
            turn: 0,
            last_recipe_turn: 0,
            lethargic: false,
        }
    }

    /// Get a random monster type
    fn rand_monster_type() -> MonsterType {
        let monster_list = [
            MonsterType::Kobold,
            MonsterType::Orc,
            MonsterType::Wolf,
            MonsterType::Goblin,
            MonsterType::Ogre,
            MonsterType::Troll,
            MonsterType::Bear,
            MonsterType::Minotaur,
            MonsterType::Gargoyle,
            MonsterType::Chimera,
            MonsterType::Balrog,
            MonsterType::Dragon,
        ];

        let mut rng = thread_rng();

        monster_list[rng.gen_range(0, monster_list.len())]
    }

    /// Wrap an x coordinate
    pub fn wrap_x(&self, x: i32) -> u32 {
        if x < 0 {
            self.dungeon.xsize() - 1
        } else if x >= self.dungeon_xsize() as i32 {
            0
        } else {
            x as u32
        }
    }

    /// Wrap a y coordinate
    pub fn wrap_y(&self, y: i32) -> u32 {
        if y < 0 {
            self.dungeon_ysize() - 1
        } else if y >= self.dungeon_ysize() as i32 {
            0
        } else {
            y as u32
        }
    }

    /// Wrap a z coordinate
    pub fn wrap_z(&self, z: i32) -> u32 {
        if z < 0 {
            self.dungeon_zsize() - 1
        } else if z >= self.dungeon_zsize() as i32 {
            0
        } else {
            z as u32
        }
    }

    /// Choose a random direction
    fn rand_direction() -> Direction {
        match Game::d(1, 4) {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("SNH"),
        }
    }

    /// Mark a random room unexplored
    fn rand_mark_unexplored(&mut self) {
        let mut rng = thread_rng();

        let x = rng.gen_range(0, self.dungeon.xsize());
        let y = rng.gen_range(0, self.dungeon.ysize());
        let z = rng.gen_range(0, self.dungeon.zsize());

        self.dungeon.room_at_mut(x, y, z).set_discovered(false);
    }

    /// Mark the player's current room as empty
    fn make_current_room_empty(&mut self) {
        let room = self
            .dungeon
            .room_at_mut(*self.player.x(), *self.player.y(), *self.player.z());

        room.make_empty();
    }

    /// Return the room at the player position
    pub fn room_at_player(&self) -> &Room {
        self.dungeon
            .room_at(*self.player.x(), *self.player.y(), *self.player.z())
    }

    /// Discover the room at the player position
    pub fn discover_room_at_player(&mut self) {
        self.dungeon
            .discover(*self.player.x(), *self.player.y(), *self.player.z())
    }

    /// Handle Gold room effects
    fn room_effect_gold(&mut self) -> Event {
        let gold_amount = Game::d(1, 10);

        self.player.add_gp(gold_amount as i32);

        self.make_current_room_empty();

        Event::FoundGold(gold_amount)
    }

    /// Handle Flare room effects
    fn room_effect_flares(&mut self) -> Event {
        let flare_amount = Game::d(1, 5);

        self.player.change_flares(flare_amount as i32);

        self.make_current_room_empty();

        Event::FoundFlares(flare_amount)
    }

    /// Handle Sinkhole room effects
    fn room_effect_sinkhole(&mut self) -> Event {
        let p_z = *self.player.z() as i32;

        let new_z = self.wrap_z(p_z + 1);

        self.player.set_z(new_z);

        self.discover_room_at_player();

        Event::Sinkhole
    }

    /// Handle Warp room effects
    fn room_effect_warp(&mut self, orb_of_zot: bool) -> Event {
        if orb_of_zot {
            let prev_dir = self.prev_dir;
            self.move_dir(prev_dir);
        } else {
            let mut rng = thread_rng();

            self.player.set_x(rng.gen_range(0, *self.dungeon.xsize()));
            self.player.set_y(rng.gen_range(0, *self.dungeon.ysize()));
            self.player.set_z(rng.gen_range(0, *self.dungeon.zsize()));
        }

        self.discover_room_at_player();

        Event::Warp
    }

    /// Handle Treasure room effects
    fn room_effect_treasure(&mut self, treasure: Treasure) -> Event {
        self.make_current_room_empty();

        self.player.treasure_add(treasure.treasure_type);

        Event::Treasure(treasure)
    }

    // Handle Monster room effects
    fn room_effect_monster(&mut self, monster: &Monster) -> Event {
        // If Vendors are not angry, head into vendor trade state instead of combat
        if monster.monster_type() == MonsterType::Vendor && !self.vendors_angry {
            self.state = GameState::Vendor;
            return Event::Vendor;
        }

        self.currently_fighting = Some(monster.clone());

        // Monster gets first attack if player blind or lethargic
        if self.lethargic || self.player.is_blind() {
            self.state = GameState::MonsterAttack;
        } else {
            self.state = GameState::PlayerAttack;
            self.bribe_possible = true;
            self.spell_possible = true;
        }

        self.retreating = false;

        Event::Combat(monster.monster_type())
    }

    /// True if the player can bribe
    pub fn bribe_possible(&self) -> bool {
        self.bribe_possible
    }

    /// True if the player can cast a spell
    pub fn spell_possible(&self) -> bool {
        self.spell_possible && *self.player.stat(&Stat::Intelligence) > 14
    }

    /// Handle player attacking monster
    pub fn attack(&mut self) -> Result<CombatEvent, Error> {
        self.attack_with(AttackType::Melee)
    }

    /// Attack a creature with a given attack type (melee, various magic)
    fn attack_with(&mut self, attack_type: AttackType) -> Result<CombatEvent, Error> {
        if self.state != GameState::PlayerAttack {
            return Err(Error::WrongState);
        }

        let hit;
        let mut damage = 0;

        match attack_type {
            AttackType::Melee => {
                if self.player.weapon().weapon_type() == WeaponType::None {
                    self.state = GameState::MonsterAttack;
                    return Ok(CombatEvent::NoWeapon);
                }

                if *self.player.book_stuck() {
                    self.state = GameState::MonsterAttack;
                    return Ok(CombatEvent::BookHands);
                }

                hit = *self.player.stat(&Stat::Dexterity)
                    >= (Game::d(1, 20) + (self.player.is_blind() as u32) * 3);

                if hit {
                    damage = self.player.weapon().damage();
                }
            }
            AttackType::Fireball => {
                // -1 ST and IQ
                if self.player.change_stat(Stat::Strength, -1) == 0
                    || self.player.change_stat(Stat::Intelligence, -1) == 0
                {
                    self.state = GameState::Dead;
                    return Ok(CombatEvent::Died);
                }

                hit = true;
                damage = Game::d(2, 7);
            }
            AttackType::Deathspell => {
                if *self.player.stat(&Stat::Intelligence) < 15 + Game::d(1, 4) {
                    self.state = GameState::Dead;
                    return Ok(CombatEvent::Died);
                }

                hit = true;
                damage = 99999;
            }
        }

        if hit {
            let mut result = HitResult {
                damage,
                broke_weapon: false,
                defeated: false,
                treasure: 0,
                got_runestaff: false,
                killed_vendor: false,
                got_lamp: false,
            };

            let mut next_state = GameState::MonsterAttack;

            if let Some(ref mut monster) = self.currently_fighting {
                if attack_type == AttackType::Melee
                    && monster.can_break_weapon()
                    && Game::d(1, 8) == 1
                {
                    result.broke_weapon = true;
                    self.player.set_weapon(Weapon::new(WeaponType::None));
                }

                result.defeated = monster.take_damage(result.damage);

                if result.defeated {
                    next_state = GameState::Move;

                    // Take vendor's wares
                    if monster.monster_type() == MonsterType::Vendor {
                        result.killed_vendor = true;

                        self.player
                            .change_stat(Stat::Strength, Game::d(1, 6) as i32);
                        self.player
                            .change_stat(Stat::Intelligence, Game::d(1, 6) as i32);
                        self.player
                            .change_stat(Stat::Dexterity, Game::d(1, 6) as i32);

                        self.player.set_armor_by_type(ArmorType::Plate);
                        self.player.set_weapon_by_type(WeaponType::Sword);

                        if !self.player.has_lamp() {
                            self.player.set_lamp(true);
                            result.got_lamp = true;
                        }
                    } else {
                        // Non-vendor creature
                        if monster.has_runestaff() {
                            self.player.give_runestaff(true);
                            result.got_runestaff = true;
                        }

                        result.treasure = Game::d(1, 1000);
                    }
                }
            } else {
                panic!("not fighting a monster");
            }

            if result.defeated {
                self.make_current_room_empty();
                self.currently_fighting = None;
            }

            self.player.add_gp(result.treasure as i32);

            self.state = next_state;
            return Ok(CombatEvent::Hit(result));
        }

        self.state = GameState::MonsterAttack;
        Ok(CombatEvent::Miss)
    }

    /// Helper function to get the next state after a monster attack
    fn state_after_monster_attack(&mut self) {
        if self.retreating {
            self.state = GameState::Retreat;
        } else {
            self.state = GameState::PlayerAttack;
        }
    }

    /// Handle a monster attack
    pub fn be_attacked(&mut self) -> Result<CombatEvent, Error> {
        if self.state != GameState::MonsterAttack {
            return Err(Error::WrongState);
        }

        self.bribe_possible = false;
        self.spell_possible = false;

        let mut web_broke = false;

        // Check for web breaking / stuck
        if let Some(ref mut monster) = self.currently_fighting {
            if *monster.webbed() > 0 {
                if monster.weaken_web() {
                    web_broke = true;
                } else {
                    self.state = GameState::PlayerAttack;
                    return Ok(CombatEvent::MonsterWebbed);
                }
            }
        } else {
            panic!("being attacked, but not by any monster");
        }

        let hit = *self.player.stat(&Stat::Dexterity)
            < (Game::d(3, 7) + (self.player.is_blind() as u32) * 3);

        let mut combat_event = None;
        let mut defeated = false;

        // Handle player hit
        if hit {
            if let Some(ref mut monster) = self.currently_fighting {
                let damage = monster.damage();
                let armor_value = self.player.armor().armor_value();

                let st_damage = std::cmp::max(damage as isize - armor_value as isize, 0) as u32;
                defeated = self.player.damage_st(st_damage);

                let armor_damage = std::cmp::min(damage, armor_value);
                let armor_destroyed = self.player.damage_armor(armor_damage);

                combat_event = Some(CombatEvent::MonsterHit(
                    st_damage,
                    defeated,
                    armor_destroyed,
                    web_broke,
                ));
            } else {
                panic!("being attacked, but not by any monster");
            }
        }

        // Set next state
        if hit {
            if defeated {
                self.state = GameState::Dead;
            } else {
                self.state_after_monster_attack();
            }

            if let Some(c_event) = combat_event {
                return Ok(c_event);
            }
        }

        self.state_after_monster_attack();

        Ok(CombatEvent::MonsterMiss)
    }

    /// Handle retreat
    ///
    /// This is split out from retreat_dir because the monster gets another
    /// attack in the middle of it.
    pub fn retreat(&mut self) -> Result<(), Error> {
        if self.state != GameState::PlayerAttack {
            return Err(Error::WrongState);
        }

        self.state = GameState::MonsterAttack;
        self.retreating = true;

        Ok(())
    }

    /// Handle bribe
    pub fn bribe_accept(&mut self) -> Result<(), Error> {
        if self.state != GameState::PlayerAttack {
            return Err(Error::WrongState);
        }

        if !self.bribe_possible() {
            return Err(Error::BribeNotPossible);
        }

        if let Some(t_type) = self.bribe_treasure {
            if self.player.remove_treasure(t_type) {
                // Player had the treasure
                self.state = GameState::Move;

                // Check if we're bribing a vendor
                let roomtype = &self
                    .dungeon
                    .room_at(*self.player.x(), *self.player.y(), *self.player.z())
                    .roomtype;

                if let RoomType::Monster(m) = roomtype {
                    if m.monster_type() == MonsterType::Vendor {
                        // If we are, make them unangry
                        self.vendors_angry = false;
                    }
                }
            } else {
                panic!("we really thought player had a treasure");
            }

            self.bribe_treasure = None;
        } else {
            // No current bribeable treasure
            return Err(Error::BribeMustProposition);
        }

        Ok(())
    }

    /// Player declines bribe offer
    pub fn bribe_decline(&mut self) -> Result<(), Error> {
        if self.state != GameState::PlayerAttack {
            return Err(Error::WrongState);
        }

        if !self.bribe_possible() {
            return Err(Error::BribeNotPossible);
        }

        self.state = GameState::MonsterAttack;

        Ok(())
    }

    /// Get the bribe treasure
    pub fn bribe_proposition(&mut self) -> Result<Option<TreasureType>, Error> {
        if self.state != GameState::PlayerAttack {
            return Err(Error::WrongState);
        }

        if !self.bribe_possible() {
            return Err(Error::BribeNotPossible);
        }

        let treasures = self.player.get_treasures();

        let count = treasures.len();

        if count == 0 {
            // If you try to bribe with no treasures, the monsters attack
            self.state = GameState::MonsterAttack;
            return Ok(None);
        }

        let mut rng = thread_rng();

        let i = rng.gen_range(0, count);

        let t_type = treasures[i];

        self.bribe_treasure = Some(t_type);

        Ok(self.bribe_treasure)
    }

    /// After the monster's final attack
    pub fn retreat_dir(&mut self, dir: Direction) {
        self.state = GameState::Move;

        self.move_dir(dir);
    }

    /// Fireball spell
    pub fn spell_fireball(&mut self) -> Result<CombatEvent, Error> {
        self.attack_with(AttackType::Fireball)
    }

    /// Deathspell spell
    pub fn spell_deathspell(&mut self) -> Result<CombatEvent, Error> {
        self.attack_with(AttackType::Deathspell)
    }

    /// Web spell
    pub fn spell_web(&mut self) -> Result<CombatEvent, Error> {
        if self.state != GameState::PlayerAttack {
            return Err(Error::WrongState);
        }

        if self.player.change_stat(Stat::Strength, -1) == 0 {
            self.state = GameState::Dead;
            return Ok(CombatEvent::Died);
        }

        self.state = GameState::MonsterAttack;

        let result = HitResult {
            damage: 0,
            broke_weapon: false,
            defeated: false,
            treasure: 0,
            got_runestaff: false,
            killed_vendor: false,
            got_lamp: false,
        };

        if let Some(ref mut monster) = self.currently_fighting {
            monster.set_webbed(Game::d(1, 6) + 1);
        } else {
            panic!("not fighting a monster");
        }

        Ok(CombatEvent::Hit(result))
    }

    /// Check for a room event
    pub fn room_effect(&mut self) -> Event {
        let roomtype;

        {
            let room = self
                .dungeon
                .room_at(*self.player.x(), *self.player.y(), *self.player.z());
            roomtype = room.roomtype.clone();
        }

        match roomtype {
            RoomType::Gold => self.room_effect_gold(),
            RoomType::Flares => self.room_effect_flares(),
            RoomType::Sinkhole => self.room_effect_sinkhole(),
            RoomType::Warp(orb_of_zot) => self.room_effect_warp(orb_of_zot),
            RoomType::Treasure(t) => self.room_effect_treasure(t),
            RoomType::Monster(m) => self.room_effect_monster(&m),
            _ => Event::None,
        }
    }

    /// True if the player can teleport
    pub fn can_teleport(&self) -> bool {
        self.player.has_runestaff()
    }

    /// Teleport the player
    ///
    /// Returns true if the player found the Orb of Zot
    pub fn teleport(&mut self, x: u32, y: u32, z: u32) -> Result<bool, Error> {
        let mut found_orb_of_zot = false;

        if !self.can_teleport() {
            return Err(Error::CantGo);
        }

        if x > 7 || y > 7 || z > 7 {
            return Err(Error::OutOfBounds);
        }

        {
            let p = &mut self.player;

            p.set_position(x, y, z);

            let room = self.dungeon.room_at(*p.x(), *p.y(), *p.z());

            if let RoomType::Warp(true) = room.roomtype {
                found_orb_of_zot = true;
                p.give_runestaff(false);
                p.give_orb_of_zot(true);
            }
        }

        if found_orb_of_zot {
            self.make_current_room_empty();
        }

        Ok(found_orb_of_zot)
    }

    /// Handle going up/down stairs
    pub fn move_stairs(&mut self, dir: Stairs) -> Result<(), Error> {
        let p = &mut self.player;

        let room = self.dungeon.room_at(*p.x(), *p.y(), *p.z());

        match dir {
            Stairs::Up => {
                if room.roomtype != RoomType::StairsUp {
                    return Err(Error::CantGo);
                }
                p.up();
            }
            Stairs::Down => {
                if room.roomtype != RoomType::StairsDown {
                    return Err(Error::CantGo);
                }
                p.down();
            }
        }

        self.discover_room_at_player();

        Ok(())
    }

    /// Handle a move command
    pub fn move_dir(&mut self, dir: Direction) {
        self.prev_dir = dir;

        let roomtype = self.room_at_player().roomtype.clone();

        // Handle exit special case
        if roomtype == RoomType::Entrance && dir == Direction::North {
            self.state = GameState::Exit;
            return;
        }

        let (p_x, p_y) = (*self.player.x() as i32, *self.player.y() as i32);

        match dir {
            Direction::North => {
                let new_y = self.wrap_y(p_y - 1);
                self.player.set_y(new_y);
            }
            Direction::South => {
                let new_y = self.wrap_y(p_y + 1);
                self.player.set_y(new_y);
            }
            Direction::West => {
                let new_x = self.wrap_x(p_x - 1);
                self.player.set_x(new_x);
            }
            Direction::East => {
                let new_x = self.wrap_x(p_x + 1);
                self.player.set_x(new_x);
            }
        }

        self.discover_room_at_player();
    }

    /// Accept selling a treasure
    pub fn vendor_treasure_accept(&mut self) -> Result<(), Error> {
        if self.vendor_treasure == None {
            return Err(Error::VendorMustOfferTreasure);
        }

        let treasure_type = self.vendor_treasure.unwrap();

        if !self.player.remove_treasure(treasure_type) {
            panic!("player should have had this treasure");
        }

        self.player.add_gp(self.vendor_treasure_price as i32);

        self.vendor_treasure = None;

        Ok(())
    }

    /// Reject selling a treasure
    pub fn vendor_treasure_reject(&mut self) -> Result<(), Error> {
        if self.vendor_treasure == None {
            return Err(Error::VendorMustOfferTreasure);
        }

        self.vendor_treasure = None;

        Ok(())
    }

    /// Check if you can afford stats
    pub fn vendor_can_afford_stat(&self) -> bool {
        self.player_gp() >= 1000
    }

    /// Buy stats from a vendor
    pub fn vendor_buy_stat(&mut self, stat: Stat) -> Result<u32, Error> {
        self.player.spend(1000)?;

        let addition = Game::d(1, 6);

        Ok(self.player.change_stat(stat, addition as i32))
    }

    /// True if the player can buy a lamp from a vendor
    pub fn vendor_can_afford_lamp(&self) -> bool {
        *self.player.gp() >= 1000
    }

    /// Buy a lamp from a vendor
    pub fn vendor_buy_lamp(&mut self) -> Result<(), Error> {
        self.player.spend(1000)?;

        self.player.set_lamp(true);

        Ok(())
    }

    /// Begin negotiations to sell a treasure to a vendor
    pub fn vendor_treasure_offer(&mut self, treasure_type: TreasureType) -> Result<u32, Error> {
        if self.state != GameState::Vendor {
            return Err(Error::WrongState);
        }

        let max_value = Treasure::treasure_max_value(treasure_type);
        self.vendor_treasure_price = Game::d(1, max_value);
        self.vendor_treasure = Some(treasure_type);

        Ok(self.vendor_treasure_price)
    }

    /// Attack a vendor
    pub fn vendor_attack(&mut self) {
        self.vendors_angry = true;
        self.state = GameState::VendorAttack;
    }

    /// Complete vendor interactions
    pub fn vendor_complete(&mut self) {
        self.state = GameState::Move;
    }

    /// Drink
    pub fn drink(&mut self) -> Result<DrinkEvent, Error> {
        let roomtype = self.room_at_player().room_type().clone();

        if roomtype != RoomType::Pool {
            return Err(Error::CantGo);
        }

        match Game::d(1, 8) {
            1 => {
                self.player
                    .change_stat(Stat::Strength, Game::d(1, 3) as i32);
                Ok(DrinkEvent::Stronger)
            }
            2 => {
                self.player
                    .change_stat(Stat::Strength, -(Game::d(1, 3) as i32));
                Ok(DrinkEvent::Weaker)
            }
            3 => {
                self.player
                    .change_stat(Stat::Intelligence, Game::d(1, 3) as i32);
                Ok(DrinkEvent::Smarter)
            }
            4 => {
                self.player
                    .change_stat(Stat::Intelligence, -(Game::d(1, 3) as i32));
                Ok(DrinkEvent::Dumber)
            }
            5 => {
                self.player
                    .change_stat(Stat::Dexterity, Game::d(1, 3) as i32);
                Ok(DrinkEvent::Nimbler)
            }
            6 => {
                self.player
                    .change_stat(Stat::Dexterity, -(Game::d(1, 3) as i32));
                Ok(DrinkEvent::Clumsier)
            }
            7 => {
                let races = [Race::Dwarf, Race::Elf, Race::Hobbit, Race::Human];

                let n = Game::d(1, 3) - 1;
                let mut i = 0;

                for _ in 0..n {
                    if races[i] == *self.player.race() {
                        i += 1;
                    }
                    i += 1;
                }

                self.player.set_race(races[i]);

                Ok(DrinkEvent::ChangeRace)
            }
            8 => {
                if *self.player.gender() == Gender::Male {
                    self.player.set_gender(Gender::Female);
                } else {
                    self.player.set_gender(Gender::Male);
                }

                Ok(DrinkEvent::ChangeGender)
            }
            _ => panic!("should not happen"),
        }
    }

    /// Shine the lamp
    pub fn shine_lamp(&mut self, dir: Direction) -> Result<(u32, u32, u32, RoomType), Error> {
        if !self.player.has_lamp() {
            return Err(Error::CantGo);
        }

        let (x, y);

        match dir {
            Direction::North => {
                x = *self.player.x();
                y = self.wrap_y(*self.player.y() as i32 - 1);
            }
            Direction::South => {
                x = *self.player.x();
                y = self.wrap_y(*self.player.y() as i32 + 1);
            }
            Direction::West => {
                x = self.wrap_x(*self.player.x() as i32 - 1);
                y = *self.player.y();
            }
            Direction::East => {
                x = self.wrap_x(*self.player.x() as i32 + 1);
                y = *self.player.y();
            }
        }

        let z = *self.player.z();

        let room = self.dungeon.room_at_mut(x, y, z);

        room.set_discovered(true);

        Ok((x, y, z, room.room_type().clone()))
    }

    /// Fire a flare from the player location
    pub fn flare(&mut self) -> Result<(), Error> {
        if self.player.flares() == 0 {
            return Err(Error::CantGo);
        }

        if self.player.is_blind() {
            return Err(Error::Blind);
        }

        self.player.change_flares(-1);

        let xm1 = *self.player.x() as i32 - 1;
        let ym1 = *self.player.y() as i32 - 1;

        let z = *self.player.z();

        for y in ym1..(ym1 + 3) {
            let yw = self.wrap_y(y);

            for x in xm1..(xm1 + 3) {
                let xw = self.wrap_x(x);

                self.dungeon.room_at_mut(xw, yw, z).set_discovered(true);
            }
        }

        Ok(())
    }

    /// Gaze into an Orb
    pub fn gaze(&mut self) -> Result<OrbEvent, Error> {
        {
            let room_type = self.room_at_player().room_type();

            if *room_type != RoomType::CrystalOrb {
                return Err(Error::CantGo);
            }
        }

        if self.player.is_blind() {
            return Err(Error::Blind);
        }

        let mut rng = thread_rng();

        match Game::d(1, 6) {
            1 => {
                self.player
                    .change_stat(Stat::Strength, -(Game::d(1, 2) as i32));
                self.make_current_room_empty();
                Ok(OrbEvent::BloodyHeap)
            }

            2 => Ok(OrbEvent::Polymorph(Game::rand_monster_type())),

            3 => Ok(OrbEvent::GazeBack(Game::rand_monster_type())),

            4 => {
                let x = rng.gen_range(0, self.dungeon.xsize());
                let y = rng.gen_range(0, self.dungeon.ysize());
                let z = rng.gen_range(0, self.dungeon.zsize());

                let room_type = self.dungeon.room_at(x, y, z).room_type().clone();

                self.dungeon.room_at_mut(x, y, z).set_discovered(true);

                Ok(OrbEvent::Item(room_type, x, y, z))
            }

            5 => {
                let (x, y, z);

                if Game::d(1, 8) <= 3 {
                    // Actual location
                    let loc = self.dungeon.orb_of_zot_location();
                    x = loc.0;
                    y = loc.1;
                    z = loc.2;
                } else {
                    // Fake location
                    x = rng.gen_range(0, self.dungeon.xsize());
                    y = rng.gen_range(0, self.dungeon.ysize());
                    z = rng.gen_range(0, self.dungeon.zsize());
                }

                Ok(OrbEvent::OrbOfZot(x, y, z))
            }

            6 => Ok(OrbEvent::SoapOpera),

            _ => panic!("SNH"),
        }
    }

    /// Open a book
    pub fn open_book(&mut self) -> Result<BookEvent, Error> {
        {
            let room_type = self.room_at_player().room_type();

            if *room_type != RoomType::Book {
                return Err(Error::CantGo);
            }
        }

        self.make_current_room_empty();

        match Game::d(1, 6) {
            1 => {
                self.player.set_blind(true);
                Ok(BookEvent::Blind)
            }
            2 => Ok(BookEvent::Poetry),
            3 => Ok(BookEvent::PlayMonster(Game::rand_monster_type())),
            4 => {
                self.player.set_stat(Stat::Dexterity, 18);
                Ok(BookEvent::Dexterity)
            }
            5 => {
                self.player.set_stat(Stat::Strength, 18);
                Ok(BookEvent::Strength)
            }
            6 => {
                self.player.set_book_stuck(true);
                Ok(BookEvent::Sticky)
            }
            _ => panic!("SNH"),
        }
    }

    /// Open a chest
    pub fn open_chest(&mut self) -> Result<ChestEvent, Error> {
        {
            let room_type = self.room_at_player().room_type();

            if *room_type != RoomType::Chest {
                return Err(Error::CantGo);
            }
        }

        // In the original game, gas would not destroy the chest.
        // We mod that here to destroy the chest in all cases.
        self.make_current_room_empty();

        match Game::d(1, 4) {
            1 => {
                if self.player.damage_st(Game::d(1, 6)) {
                    self.state = GameState::Dead;
                }
                Ok(ChestEvent::Explode)
            }
            2 => {
                self.add_turn(20);
                self.move_dir(Game::rand_direction());
                Ok(ChestEvent::Gas)
            }
            3...4 => {
                let gold = Game::d(1, 1000);
                self.player.add_gp(gold as i32);
                Ok(ChestEvent::Treasure(gold))
            }
            _ => panic!("SNR"),
        }
    }

    /// Cure blindness
    ///
    /// True if blindness was cured
    pub fn cure_blindness(&mut self) -> bool {
        if self.player.is_blind() && self.player.has_treasure(TreasureType::OpalEye) {
            self.player.set_blind(false);
            true
        } else {
            false
        }
    }

    /// Cure book stuck to hands
    ///
    /// True if the book was dissolved
    pub fn cure_book(&mut self) -> bool {
        if *self.player.book_stuck() && self.player.has_treasure(TreasureType::BlueFlame) {
            self.player.set_book_stuck(false);
            true
        } else {
            false
        }
    }

    /// Handle curses
    pub fn curse_effects(&mut self) {
        if self.player.has_curse(CurseType::Lethargy) {
            if !self.player.has_treasure(TreasureType::RubyRed) {
                self.lethargic = true;
                self.turn += 1; // additional turn count per turn
            } else {
                self.lethargic = false;
            }
        }

        if self.player.has_curse(CurseType::Forgetfulness)
            && !self.player.has_treasure(TreasureType::GreenGem)
        {
            self.rand_mark_unexplored();
        }

        if self.player.has_curse(CurseType::TheLeech)
            && !self.player.has_treasure(TreasureType::PalePearl)
        {
            self.player.add_gp(-(Game::d(1, 5) as i32));
        }
    }

    /// Check for catching a curse
    pub fn curse_check(&mut self) -> bool {
        let curse = *self.room_at_player().curse();

        self.player.add_curse(curse);

        curse != CurseType::None
    }

    /// Choose a random message
    pub fn rand_message(&self) -> RandomMessage {
        if Game::d(1, 5) != 1 {
            return RandomMessage::None;
        }

        let mut msgs = vec![
            RandomMessage::HearSound,
            RandomMessage::Sneeze,
            RandomMessage::StepFrog,
            RandomMessage::MonsterFrying,
            RandomMessage::Watched,
            RandomMessage::Playing,
        ];

        // In the original game, "YOU SEE A BAT" was replaced by "YOU STEPPED ON
        // A FROG" if the player was blind. Instead, here we just don't show the
        // "SEE" messages if the player is blind.

        if !self.player.is_blind() {
            msgs.push(RandomMessage::SeeBat);
        }

        let mut rng = thread_rng();

        let i = rng.gen_range(0, msgs.len());

        msgs[i]
    }

    /// Tell the caller if it's time for a random recipe
    pub fn rand_recipe(&mut self) -> bool {
        if self.last_recipe_turn == 0 || self.turn - self.last_recipe_turn > 60 {
            self.last_recipe_turn = self.turn;
            true
        } else {
            false
        }
    }

    /// Quit the game
    pub fn quit(&mut self) {
        self.state = GameState::Quit;
    }

    /// Roll a die (1d6, 2d7, etc.)
    pub fn d(count: u32, sides: u32) -> u32 {
        let mut total = 0;

        let mut rng = thread_rng();

        for _ in 0..count {
            total += rng.gen_range(0, sides) + 1;
        }

        total
    }

    /// Return game state
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Accessors for player position
    pub fn player_x(&self) -> u32 {
        *self.player.x()
    }

    /// Accessors for player position
    pub fn player_y(&self) -> u32 {
        *self.player.y()
    }

    /// Accessors for player position
    pub fn player_z(&self) -> u32 {
        *self.player.z()
    }

    /// Accessor for player race
    pub fn player_race(&self) -> &Race {
        self.player.race()
    }

    /// Accessor for player gold pieces
    pub fn player_gp(&self) -> u32 {
        *self.player.gp()
    }

    /// Accessor for player additional points
    pub fn player_additional_points(&self) -> u32 {
        *self.player.additional_points()
    }

    /// Accessor for player stats
    pub fn player_stat(&self, stat: Stat) -> u32 {
        *self.player.stat(&stat)
    }

    /// Accessor for player armor type
    pub fn player_armor_type(&self) -> ArmorType {
        self.player.armor().armor_type()
    }

    /// Accessor for player weapon type
    pub fn player_weapon_type(&self) -> WeaponType {
        self.player.weapon().weapon_type()
    }

    /// Accessor for player lamp
    pub fn player_has_lamp(&self) -> bool {
        self.player.has_lamp()
    }

    /// Accessor for player flares
    pub fn player_flares(&self) -> u32 {
        self.player.flares()
    }

    /// Init the player
    pub fn player_init(&mut self, race: Race) {
        self.player.init(race);
    }

    /// Set player's gender
    pub fn player_set_gender(&mut self, gender: Gender) {
        self.player.set_gender(gender);
    }

    /// Allocate player stat points
    pub fn player_allocate_points(&mut self, stat: Stat, points: u32) -> Result<u32, Error> {
        self.player.allocate_points(stat, points)
    }

    /// Give the player some armor
    pub fn player_purchase_armor(&mut self, a: ArmorType, is_vendor: bool) -> Result<(), Error> {
        self.player.purchase_armor(a, is_vendor)
    }

    /// Give the player a weapon
    pub fn player_purchase_weapon(&mut self, w: WeaponType, is_vendor: bool) -> Result<(), Error> {
        self.player.purchase_weapon(w, is_vendor)
    }

    /// True if the player can afford a lamp
    pub fn player_can_purchase_lamp(&self) -> bool {
        self.player.can_purchase_lamp()
    }

    /// Purchase a lamp
    pub fn player_purchase_lamp(&mut self, lamp: bool) -> Result<(), Error> {
        self.player.purchase_lamp(lamp)
    }

    /// Return the max number of flares a player can afford
    pub fn player_max_flares(&self) -> u32 {
        self.player.max_flares()
    }

    /// Purchase flares
    pub fn player_purchase_flares(&mut self, flares: u32) -> Result<(), Error> {
        self.player.purchase_flares(flares)
    }

    /// Return true if the player is blind
    pub fn player_is_blind(&self) -> bool {
        self.player.is_blind()
    }

    /// True if the player has the Orb of Zot
    pub fn player_has_orb_of_zot(&self) -> bool {
        self.player.has_orb_of_zot()
    }

    /// Return a list of players treasures
    pub fn player_get_treasures(&self) -> &Vec<TreasureType> {
        self.player.get_treasures()
    }

    /// True if the player has the Runestaff
    pub fn player_has_runestaff(&self) -> bool {
        self.player.has_runestaff()
    }

    /// Return x dimension
    pub fn dungeon_xsize(&self) -> u32 {
        *self.dungeon.xsize()
    }

    /// Return y dimension
    pub fn dungeon_ysize(&self) -> u32 {
        *self.dungeon.ysize()
    }

    /// Return z dimension
    pub fn dungeon_zsize(&self) -> u32 {
        *self.dungeon.zsize()
    }

    /// Return a reference to the room at a location
    pub fn dungeon_room_at(&self, x: u32, y: u32, z: u32) -> &Room {
        self.dungeon.room_at(x, y, z)
    }

    /// Return a mutable reference to the room at a location
    pub fn dungeon_room_at_mut(&mut self, x: u32, y: u32, z: u32) -> &Room {
        self.dungeon.room_at_mut(x, y, z)
    }

    /// Get character gender
    pub fn player_gender(&self) -> &Gender {
        self.player.gender()
    }

    /// Return number of turns
    pub fn turn(&self) -> &u32 {
        &self.turn
    }

    /// Add to turns
    pub fn add_turn(&mut self, amount: u32) {
        self.turn += amount;
    }
}
