use dungeon::Dungeon;
use player::Player;
use room::RoomType;

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

    pub fn move_dir(&mut self, dir:Direction) {
        let p = &mut self.player;

        let room = self.dungeon.room_at(p.x, p.y, p.z);

        if room.roomtype == RoomType::Entrance {
            // TODO handle game exit
        }

        match dir {
            Direction::North => {
                if p.y == 0 {
                    p.y = self.dungeon.ysize - 1;
                } else {
                    p.y -= 1;
                }
            }
            Direction::South => p.y = (p.y + 1) % self.dungeon.ysize,
            Direction::West =>  {
                if p.x == 0 {
                    p.x = self.dungeon.xsize - 1;
                } else {
                    p.x -= 1;
                }
            }
            Direction::East => p.x = (p.x + 1) % self.dungeon.xsize,
        }
    }
}