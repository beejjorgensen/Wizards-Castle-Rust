extern crate rand;

use dungeon::Dungeon;
use player::Player;
use room::RoomType;

use self::rand::Rng;
use self::rand::thread_rng;

#[derive(Debug,Clone,Copy)]
pub enum Event {
    None,
    FoundGold(usize),
    FoundFlares(usize),
    Sinkhole,
    Warp,
}

#[derive(Debug,Clone,Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

pub enum GameState {
    CharGenChooseClass,
    CharGenStats,
    CharGenOutfit,

    Move,

    Vendor,

    CombatPlayer,
    CombatMonster,

    Warp,
    Sinkhole,
    Gas,
}

pub struct Game {
    pub dungeon: Dungeon,
    pub player: Player,
    state: GameState,
    prev_dir: Direction,
}

impl Game {
    pub fn new(xsize: usize, ysize: usize, zsize: usize) -> Game {

        let dungeon = Dungeon::new(xsize, ysize, zsize);

        let mut player = Player::new();
        player.set_position(dungeon.entrance_x(), 0, 0);

        Game {
            dungeon,
            player,
            state: GameState::CharGenChooseClass,
            prev_dir: Direction::South,
        }
    }
    
    fn room_effect_gold(&mut self) -> Event {
        let gold_amount = Game::d(1,10);

        self.player.gp += gold_amount;

        let room = self.dungeon.room_at_mut(self.player.x, self.player.y, self.player.z);

        room.make_empty();

        return Event::FoundGold(gold_amount);
    }

    fn room_effect_flares(&mut self) -> Event {
        let flare_amount = Game::d(1,5);

        self.player.flares += flare_amount;

        let room = self.dungeon.room_at_mut(self.player.x, self.player.y, self.player.z);

        room.make_empty();

        return Event::FoundFlares(flare_amount);
    }

    fn room_effect_sinkhole(&mut self) -> Event {
        self.player.z = (self.player.z + 1) % self.dungeon.zsize;

        return Event::Sinkhole;
    }

    fn room_effect_warp(&mut self, orb_of_zot:bool) -> Event {
        if orb_of_zot {
            let prev_dir = self.prev_dir;
            self.move_dir(prev_dir);
        } else {
            let mut rng = thread_rng();

            self.player.x = rng.gen_range(0, self.dungeon.xsize);
            self.player.y = rng.gen_range(0, self.dungeon.ysize);
            self.player.z = rng.gen_range(0, self.dungeon.zsize);
        }

        return Event::Warp;
    }

    // Check for a room event
    pub fn room_effect(&mut self) -> Event {

        let roomtype;

        {
            let room = self.dungeon.room_at(self.player.x, self.player.y, self.player.z);
            roomtype = room.roomtype;
        }

        match roomtype {
            RoomType::Gold => self.room_effect_gold(),
            RoomType::Flares => self.room_effect_flares(),
            RoomType::Sinkhole => self.room_effect_sinkhole(),
            RoomType::Warp(orb_of_zot) => self.room_effect_warp(orb_of_zot),
            _ => Event::None,
        }

/*
        let action;
        let mut orb_of_zot = false;

        {
            let room = self.dungeon.room_at(self.player.x, self.player.y, self.player.z);

            action = match room.roomtype {
                RoomType::Gold => 1,
                RoomType::Flares => 2,
                RoomType::Sinkhole => 3,
                RoomType::Warp(oz) => {
                    orb_of_zot = oz;
                    4
                }
                _ => 99,
            };
        }

        match action {
            1 => self.room_effect_gold(),
            2 => self.room_effect_flares(),
            3 => self.room_effect_sinkhole(),
            4 => self.room_effect_warp(orb_of_zot),
            _ => Event::None,
        }
        */
    }

    /// Handle a move command
    pub fn move_dir(&mut self, dir:Direction) {
        let p = &mut self.player;

        let xsize = self.dungeon.xsize;
        let ysize = self.dungeon.ysize;

        let room = self.dungeon.room_at(p.x, p.y, p.z);

        self.prev_dir = dir;

        if room.roomtype == RoomType::Entrance {
            // TODO handle game exit
        }

        match dir {
            Direction::North => {
                if p.y == 0 {
                    p.y = ysize - 1;
                } else {
                    p.y -= 1;
                }
            }
            Direction::South => p.y = (p.y + 1) % ysize,
            Direction::West =>  {
                if p.x == 0 {
                    p.x = xsize - 1;
                } else {
                    p.x -= 1;
                }
            }
            Direction::East => p.x = (p.x + 1) % xsize,
        }
    }

    /// Roll a die (1d6, 2d7, etc.)
    pub fn d(count:usize, sides:usize) -> usize {
        let mut total = 0;

        let mut rng = thread_rng();

        for _ in 0..count {
            total += rng.gen_range(0, sides) + 1;
        }

        total
    }
}