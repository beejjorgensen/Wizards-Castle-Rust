extern crate rand;

use dungeon::Dungeon;
use player::{Player, Stat, Race};
use room::{Room, RoomType};
use treasure::{Treasure, TreasureType};
use monster::{Monster, MonsterType};
use weapon::{Weapon, WeaponType};
use error::Error;

use self::rand::Rng;
use self::rand::thread_rng;

#[derive(Debug,Clone)]
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

#[derive(Debug,Clone,Copy)]
pub enum CombatEvent {
    NoWeapon,
    //BookHands,
    Miss,
    Hit(u32, bool, bool, u32, bool),
    MonsterMiss,
    MonsterHit(u32, bool, bool),
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Stairs {
    Up,
    Down,
}

#[derive(Debug,Clone,Copy,PartialEq)]
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
}

pub struct Game {
    pub dungeon: Dungeon,
    pub player: Player,
    state: GameState,
    prev_dir: Direction,
    currently_fighting: Option<Monster>,
    bribe_possible: bool,
    bribe_treasure:Option<TreasureType>,
    retreating:bool,
    vendors_angry:bool,
    vendor_treasure_price:u32,
    vendor_treasure: Option<TreasureType>,
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
            vendors_angry: false,
            vendor_treasure_price: 0,
            vendor_treasure: None,
        }
    }

    /// Wrap an x coordinate
    fn wrap_x(&self, x: i32) -> u32 {
        if x < 0 {
            self.dungeon.xsize - 1
        } else if x >= self.dungeon.xsize as i32 {
            0
        } else {
            x as u32
        }
    }
    
    /// Wrap a y coordinate
    fn wrap_y(&self, y: i32) -> u32 {
        if y < 0 {
            self.dungeon.ysize - 1
        } else if y >= self.dungeon.ysize as i32 {
            0
        } else {
            y as u32
        }
    }
    
    /// Wrap a z coordinate
    fn wrap_z(&self, z: i32) -> u32 {
        if z < 0 {
            self.dungeon.zsize - 1
        } else if z >= self.dungeon.zsize as i32 {
            0
        } else {
            z as u32
        }
    }
    
    /// Mark the player's current room as empty
    fn make_current_room_empty(&mut self) {
        let room = self.dungeon.room_at_mut(*self.player.x(), *self.player.y(), *self.player.z());

        room.make_empty();
    }

    /// Return the room at the player position
    pub fn room_at_player(&self) -> &Room {
        self.dungeon.room_at(*self.player.x(), *self.player.y(), *self.player.z())
    }

    /// Discover the room at the player position
    pub fn discover_room_at_player(&mut self) {
        self.dungeon.discover(*self.player.x(), *self.player.y(), *self.player.z())
    }

    /// Handle Gold room effects
    fn room_effect_gold(&mut self) -> Event {
        let gold_amount = Game::d(1,10);

        self.player.add_gp(gold_amount);

        self.make_current_room_empty();

        return Event::FoundGold(gold_amount);
    }

    /// Handle Flare room effects
    fn room_effect_flares(&mut self) -> Event {
        let flare_amount = Game::d(1,5);

        self.player.flares += flare_amount;

        self.make_current_room_empty();

        return Event::FoundFlares(flare_amount);
    }

    /// Handle Sinkhole room effects
    fn room_effect_sinkhole(&mut self) -> Event {
        let p_z = *self.player.z() as i32;

        let new_z = self.wrap_z(p_z + 1);

        self.player.set_z(new_z);

        return Event::Sinkhole;
    }

    /// Handle Warp room effects
    fn room_effect_warp(&mut self, orb_of_zot:bool) -> Event {
        if orb_of_zot {
            let prev_dir = self.prev_dir;
            self.move_dir(prev_dir);
        } else {
            let mut rng = thread_rng();

            self.player.set_x(rng.gen_range(0, self.dungeon.xsize));
            self.player.set_y(rng.gen_range(0, self.dungeon.ysize));
            self.player.set_z(rng.gen_range(0, self.dungeon.zsize));
        }

        return Event::Warp;
    }

    /// Handle Treasure room effects
    fn room_effect_treasure(&mut self, treasure:Treasure) -> Event {
        self.make_current_room_empty();

        self.player.treasures.push(treasure.treasure_type);

        Event::Treasure(treasure)
    }

    // Handle Monster room effects
    fn room_effect_monster(&mut self, monster:Monster) -> Event {

        // If Vendors are not angry, head into vendor trade state instead of combat
        if monster.monster_type() == MonsterType::Vendor && !self.vendors_angry {
            self.state = GameState::Vendor;
            return Event::Vendor;
        }

        self.currently_fighting = Some(monster.clone());

        // TODO check for blind or lethargy

        self.state = GameState::PlayerAttack;

        self.bribe_possible = true;

        self.retreating = false;

        Event::Combat(monster.monster_type())
    }

    /// True if the player can bribe
    pub fn bribe_possible(&self) -> bool {
        self.bribe_possible
    }

    /// True if the player can cast a spell
    pub fn spell_possible(&self) -> bool {
        self.player.iq > 14
    }

    /// Handle player attacking monster
    pub fn attack(&mut self) -> Result<CombatEvent, Error> {
        if self.state != GameState::PlayerAttack {
            return Err(Error::WrongState);
        }

        if self.player.weapon.weapon_type() == WeaponType::None {
            self.state = GameState::MonsterAttack;
            return Ok(CombatEvent::NoWeapon);
        }

        let hit = self.player.dx >= (Game::d(1, 20) + (self.player.is_blind() as u32) * 3);

        if hit {
            let damage = self.player.weapon.damage();
            let mut broke_weapon = false;
            let mut next_state = GameState::MonsterAttack;
            let defeated;
            let mut got_runestaff = false;
            let treasure;

            if let Some(ref mut monster) = self.currently_fighting {
                if monster.can_break_weapon() && Game::d(1,8) == 1 {
                    broke_weapon = true;
                    self.player.weapon = Weapon::new(WeaponType::None);
                }

                defeated = monster.take_damage(damage);
                
                if defeated {
                    next_state = GameState::Move;

                    if monster.has_runestaff() {
                        self.player.give_runestaff(true);
                        got_runestaff = true;
                    }

                    // TODO if defeated vendor, get his wares
                }
            } else {
                panic!("not fighting a monster");
            }

            if defeated {
                self.make_current_room_empty();
                self.currently_fighting = None;

                treasure = Game::d(1,1000);
            } else {
                treasure = 0;
            }

            self.player.add_gp(treasure);

            self.state = next_state;
            return Ok(CombatEvent::Hit(damage, broke_weapon, defeated, treasure, got_runestaff));
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

        // TODO check for web breaking

        // TODO check for stuck in web

        let hit = self.player.dx < (Game::d(3,7) + (self.player.is_blind() as u32) * 3);

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

                combat_event = Some(CombatEvent::MonsterHit(st_damage, defeated, armor_destroyed));

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
                let roomtype = &self.dungeon.room_at(*self.player.x(), *self.player.y(), *self.player.z()).roomtype;

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

        let t_type = treasures.get(i).unwrap();

        self.bribe_treasure = Some(*t_type);

        Ok(self.bribe_treasure)
    }

    /// After the monster's final attack
    pub fn retreat_dir(&mut self, dir:Direction) {
        self.state = GameState::Move;

        self.move_dir(dir);
    }
    
    /// Check for a room event
    pub fn room_effect(&mut self) -> Event {

        let roomtype;

        {
            let room = self.dungeon.room_at(*self.player.x(), *self.player.y(), *self.player.z());
            roomtype = room.roomtype.clone();
        }

        match roomtype {
            RoomType::Gold => self.room_effect_gold(),
            RoomType::Flares => self.room_effect_flares(),
            RoomType::Sinkhole => self.room_effect_sinkhole(),
            RoomType::Warp(orb_of_zot) => self.room_effect_warp(orb_of_zot),
            RoomType::Treasure(t) => self.room_effect_treasure(t),
            RoomType::Monster(m) => self.room_effect_monster(m),
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
    pub fn teleport(&mut self, x:u32, y:u32, z:u32) -> Result<bool, Error> {
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
    pub fn move_stairs(&mut self, dir:Stairs) -> Result<(), Error> {
        let p = &mut self.player;

        let room = self.dungeon.room_at(*p.x(), *p.y(), *p.z());

        match dir {
            Stairs::Up => {
                if room.roomtype != RoomType::StairsUp {
                    return Err(Error::CantGo);
                }
                p.up();
            },
            Stairs::Down => {
                if room.roomtype != RoomType::StairsDown {
                    return Err(Error::CantGo);
                }
                p.down();
            },
        }

        Ok(())
    }

    /// Handle a move command
    pub fn move_dir(&mut self, dir:Direction) {
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
            },
            Direction::South => {
                let new_y = self.wrap_y(p_y + 1);
                self.player.set_y(new_y);
            },
            Direction::West => {
                let new_x = self.wrap_x(p_x - 1);
                self.player.set_x(new_x);
            },
            Direction::East => {
                let new_x = self.wrap_x(p_x + 1);
                self.player.set_x(new_x);
            },
        }
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

        self.player.add_gp(self.vendor_treasure_price);

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
    pub fn vendor_buy_stat(&mut self, stat:Stat) -> Result<u32, Error> {
        self.player.spend(1000)?;

        let addition = Game::d(1,6);

        Ok(self.player.add_stat(&stat, addition))
    }

    /// Begin negotiations to sell a treasure to a vendor
    pub fn vendor_treasure_offer(&mut self, treasure_type:TreasureType) -> Result<u32, Error> {
        if self.state != GameState::Vendor {
            return Err(Error::WrongState);
        }

        let max_value = Treasure::treasure_max_value(treasure_type);
        self.vendor_treasure_price = Game::d(1,max_value);
        self.vendor_treasure = Some(treasure_type);

        return Ok(self.vendor_treasure_price);
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

    /// Roll a die (1d6, 2d7, etc.)
    pub fn d(count:u32, sides:u32) -> u32 {
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
}