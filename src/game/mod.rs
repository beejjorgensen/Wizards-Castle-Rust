use dungeon::Dungeon;
use player::Player;

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
}