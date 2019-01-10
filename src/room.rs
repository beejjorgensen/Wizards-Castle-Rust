use crate::treasure::Treasure;
use crate::monster::Monster;
use crate::curse::CurseType;

#[derive(Debug,PartialEq,Clone)]
pub enum RoomType {
    Empty,
    Entrance,
    StairsDown,
    StairsUp,
    Gold,
    Pool,
    Chest,
    Flares,
    Warp(bool),
    Sinkhole,
    CrystalOrb,
    Book,
    Monster(Monster),
    Treasure(Treasure),
}

#[derive(Debug)]
pub struct Room {
    pub roomtype: RoomType,
    pub discovered: bool,
    pub curse: CurseType,
}

impl Default for Room {
    fn default() -> Room {
        Room {
            roomtype: RoomType::Empty,
            discovered: false,
            curse: CurseType::None,
        }
    }
}

impl Room {
    /// Mark the room as empty
    pub fn make_empty(&mut self) {
        self.roomtype = RoomType::Empty;
    }

    /// Get the room type
    pub fn room_type(&self) -> &RoomType {
        &self.roomtype
    }

    /// Mark room as discovered
    pub fn set_discovered(&mut self, discovered: bool) {
        self.discovered = discovered;
    }

    /// Sets curse in a room
    pub fn set_curse(&mut self, curse_type: CurseType) {
        self.curse = curse_type;
    }

    /// Returns the room's curse status
    pub fn curse(&self) -> &CurseType {
        &self.curse
    }
}

