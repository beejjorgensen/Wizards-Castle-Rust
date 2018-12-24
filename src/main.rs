extern crate rand;

use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

const MONSTER_COUNT: usize = 13;

#[derive(Debug,PartialEq)]
enum MonsterType {
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

#[derive(Debug,PartialEq)]
struct Monster {
    monster_type: MonsterType,
    name: String,
    hp: usize,
    damage: usize,
    break_weapon: bool,
    has_runestaff: bool,
}

impl Monster {
    fn new(monster_num: usize, has_runestaff: bool) -> Monster {
        let name = [
            "kobold",
            "orc",
            "wolf",
            "goblin",
            "ogre",
            "troll",
            "bear",
            "minotaur",
            "gargoyle",
            "chimera",
            "balrog",
            "dragon",
            "vendor",
        ];

        let monster_type = match monster_num {
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
            _ => MonsterType::Vendor, // This should never happen
        };

        let m1 = monster_num + 1; // Change to 1-based

        let hp = m1 + 2;
        let damage = 1 + m1 / 2;

        let break_weapon = monster_type == MonsterType::Gargoyle || monster_type == MonsterType::Dragon;

        Monster{
            monster_type,
            name: String::from(name[monster_num]),
            hp,
            damage,
            break_weapon,
            has_runestaff
        }
    }
}

const TREASURE_COUNT:usize = 8;

#[derive(Debug,PartialEq)]
enum TreasureType {
    RubyRed,
    NornStone,
    PalePearl,
    OpalEye,
    GreenGem,
    BlueFlame,
    Palintir,
    Silmaril,
}

#[derive(Debug,PartialEq)]
struct Treasure {
    treasure_type: TreasureType,
    name: String,
    max_value: usize,
}

impl Treasure {
    fn new(treasure_num: usize) -> Treasure {
        let name = [
            "ruby red",
            "norn stone",
            "pale pearl",
            "opal eye",
            "green gem",
            "blue flame",
            "palintir",
            "simaril",
        ];

        let max_value = (treasure_num + 1) * 1500;

        Treasure {
            treasure_type: Dungeon::get_treasure_by_id(treasure_num),
            name: String::from(name[treasure_num]),
            max_value,
        }
    }
}


const CURSE_COUNT:usize = 3;

#[derive(Debug)]
enum CurseType {
    None,
    Forgetfulness,
    TheLeech,
    Lethargy,
}

#[derive(Debug,PartialEq)]
enum RoomType {
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
struct Room {
    roomtype: RoomType,
    discovered: bool,
    curse: CurseType,
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

#[derive(Debug)]
struct Dungeon {
    levels: Vec<Vec<Room>>,
    xsize: usize,
    ysize: usize,
    zsize: usize,
}

impl Dungeon {

    fn new(xsize: usize, ysize: usize, zsize:usize) -> Dungeon {
        let mut levels: Vec<Vec<Room>> = Vec::new();

        let mut rng = thread_rng();

        let area = xsize * ysize;

        let stair_count = area / 32; // 2 in 8x8
        let item_count = area / 21; // 3 in 8x8

        let entrance_x = (xsize - 1) / 2;

        let orb_of_zot_level = rng.gen_range(0, zsize);
        let runestaff_level = rng.gen_range(0, zsize);

        // Add all necessary elements to the level
        for z in 0..zsize {
            let mut this_level = Vec::new();

            // Entrance
            if z == 0 {
                this_level.push(Room{ roomtype: RoomType::Entrance, discovered: true, ..Default::default() });
            }

            // Stairs down
            if z < zsize - 1 {
                for _ in 0..stair_count {
                    this_level.push(Room{ roomtype: RoomType::StairsDown, ..Default::default() });
                }
            }

            // Stairs up
            if z > 0 {
                for _ in 0..stair_count {
                    this_level.push(Room{ roomtype: RoomType::StairsUp, ..Default::default() });
                }
            }

            // Items
            for i in 0..item_count {
                let orb_of_zot_warp = i == 0 && z == orb_of_zot_level;

                this_level.push(Room{ roomtype: RoomType::Gold, ..Default::default() });
                this_level.push(Room{ roomtype: RoomType::Pool, ..Default::default() });
                this_level.push(Room{ roomtype: RoomType::Chest, ..Default::default() });
                this_level.push(Room{ roomtype: RoomType::Flares, ..Default::default() });
                this_level.push(Room{ roomtype: RoomType::Warp(orb_of_zot_warp), ..Default::default() });
                this_level.push(Room{ roomtype: RoomType::Sinkhole, ..Default::default() });
                this_level.push(Room{ roomtype: RoomType::CrystalOrb, ..Default::default() });
                this_level.push(Room{ roomtype: RoomType::Book, ..Default::default() });
            }

            // Monsters
            let monsters_to_place = MONSTER_COUNT - 1; // -1 to not count the Vendors

            let monster_with_runestaff = rng.gen_range(0, monsters_to_place);

            for i in 0..monsters_to_place {
                let has_runestaff = i == monster_with_runestaff && z == runestaff_level;

                this_level.push(Room{ roomtype: RoomType::Monster(Monster::new(i, has_runestaff)), ..Default::default() });
            }

            levels.push(this_level);
        }

        // Add curse rooms
        for i in 0..CURSE_COUNT {
            let curse_level = rng.gen_range(0, zsize);

            let curse = match i {
                0 => CurseType::Forgetfulness,
                1 => CurseType::TheLeech,
                2 => CurseType::Lethargy,
                _ => CurseType::None,
            };

            levels[curse_level].push(Room { curse, ..Default::default() })
        }

        for i in 0..TREASURE_COUNT {
            let treasure_level = rng.gen_range(0, zsize);

            levels[treasure_level].push(Room { roomtype: RoomType::Treasure(Treasure::new(i)), ..Default::default() })
        }

        // Run through the levels, padding them with empty rooms, shuffling
        // them, and moving certain rooms to their proper positions.

        for z in 0..zsize {
            // Fill the rest with empty
            while levels[z].len() < area {
                levels[z].push(Room{ roomtype: RoomType::Empty, ..Default::default() });
            }

            // Shuffle the level
            levels[z].shuffle(&mut rng);

            // Fix up the entrance
            for y in 0..ysize {
                for x in 0..xsize {
                    let i = y * xsize + x;

                    // Swap the entrance
                    if levels[z][i].roomtype == RoomType::Entrance {
                        let i2 = 0 * xsize + entrance_x;

                        levels[z].swap(i, i2);
                    }
                }
            }

            // Fix up the stairs up
            if z > 0 {
                let mut downs = Vec::new();
                let mut ups = Vec::new();

                for i in 0..xsize * ysize {
                    if levels[z-1][i].roomtype == RoomType::StairsDown {
                        downs.push(i);
                    }
                }

                for i in 0..xsize * ysize {
                    if levels[z][i].roomtype == RoomType::StairsUp {
                        ups.push(i);
                    }
                }

                while ups.len() > 0 {
                    let up_i = ups.pop().unwrap();
                    let down_i = downs.pop().unwrap();

                    levels[z].swap(up_i, down_i);
                }
            }
        }

        Dungeon{levels, xsize, ysize, zsize}
    }

    /// Return a treasure for a given ID
    fn get_treasure_by_id(id: usize) -> TreasureType {
        match id {
            0 => TreasureType::RubyRed,
            1 => TreasureType::NornStone,
            2 => TreasureType::PalePearl,
            3 => TreasureType::OpalEye,
            4 => TreasureType::GreenGem,
            5 => TreasureType::BlueFlame,
            6 => TreasureType::Palintir,
            7 => TreasureType::Silmaril,
            _ => panic!("get_treasure_by_id: unknown ID")
        }
    }

    /// Get the entrance x position
    fn entrance_x(&self) -> usize {
        return (self.xsize - 1) / 2;
    }

    /// Return a reference to the room at a location
    fn room_at(&self, x: usize, y: usize, z: usize) -> &Room { // TODO: Result
        let i = y * self.xsize + x;

        &self.levels[z][i]
    }

}

struct Player {
    x: usize,
    y: usize,
    z: usize,
    //blind: bool,
}

impl Player {
    fn new(x: usize, y: usize, z: usize) -> Player {

        Player {
            x,
            y,
            z,
            //blind: false
        }
    }
}

/// Print a map
fn map(dungeon: &Dungeon, player: &Player, show_all: bool) {
    let z = player.z;

    for y in 0..dungeon.ysize {
        for x in 0..dungeon.xsize {

            if x >= 1 {
                print!("   ");
            }

            let r = dungeon.room_at(x, y, z);

            let bracket = x == player.x && y == player.y;

            if bracket {
                print!("<");
            } else {
                print!(" ");
            }

            if r.discovered || show_all {
                match r.roomtype {
                    RoomType::Empty => print!("."),
                    RoomType::Entrance => print!("E"),
                    RoomType::StairsDown => print!("D"),
                    RoomType::StairsUp => print!("U"),
                    RoomType::Gold => print!("G"),
                    RoomType::Pool => print!("P"),
                    RoomType::Chest => print!("C"),
                    RoomType::Flares => print!("F"),
                    RoomType::Warp(_) => print!("W"),
                    RoomType::Sinkhole => print!("S"),
                    RoomType::CrystalOrb => print!("O"),
                    RoomType::Book => print!("B"),
                    RoomType::Monster(_) => print!("M"),
                    RoomType::Treasure(_) => print!("T"),
                }
            } else {
                print!("?");
            }

            if bracket {
                print!(">");
            } else {
                print!(" ");
            }
        }

        println!("\n");
    }
}

/// Main
fn main() {
    let dungeon = Dungeon::new(8, 8, 8);
    let mut player = Player::new(dungeon.entrance_x(), 0, 0);

    map(&dungeon, &player, true);

    player.z = 1;
    println!("");

    map(&dungeon, &player, true);
}
