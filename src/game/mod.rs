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
        }
    }
    
    // Check for a room event
    pub fn room_effect(&mut self) -> Event {
        let p = &mut self.player;

        let zsize = self.dungeon.zsize;

        let room = self.dungeon.room_at(p.x, p.y, p.z);

        if room.roomtype == RoomType::Gold {
            let gold_amount = Game::d(1,10);

            p.gp += gold_amount;

            room.make_empty();

            return Event::FoundGold(gold_amount);
        }

        if room.roomtype == RoomType::Flares {
            let flare_amount = Game::d(1,5);

            p.flares += flare_amount;

            room.make_empty();

            return Event::FoundFlares(flare_amount);
        }

        if room.roomtype == RoomType::Sinkhole {
            p.z = (p.z + 1) % zsize;

            return Event::Sinkhole;
        }
        
        Event::None
    }

    /// Handle a move command
    pub fn move_dir(&mut self, dir:Direction) {
        let p = &mut self.player;

        let xsize = self.dungeon.xsize;
        let ysize = self.dungeon.ysize;

        let room = self.dungeon.room_at(p.x, p.y, p.z);

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