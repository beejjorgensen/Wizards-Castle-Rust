extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

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
    Warp,
    Sinkhole,
    CrystalOrb,
    Book,
    Monster(Monster)
}

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
}

impl Monster {
    fn new(monster_num: usize) -> Monster {
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

        Monster{monster_type, name: String::from(name[monster_num]), hp, damage, break_weapon}
    }
}

#[derive(Debug)]
struct Room {
    roomtype: RoomType,
    discovered: bool,
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
        let mut levels = Vec::new();

        let mut rng = thread_rng();

        let area = xsize * ysize;

        let stair_count = area / 32; // 2 in 8x8
        let item_count = area / 21; // 3 in 8x8

        let entrance_x = (xsize - 1) / 2;

        for z in 0..zsize {
            let mut v = Vec::new();
            let mut remaining = area;

            // Entrance

            if z == 0 {
                v.push(Room{ roomtype: RoomType::Entrance, discovered: true});
                remaining -= 1;
            }

            // Stairs down

            if z < zsize - 1 {
                for _ in 0..stair_count {
                    v.push(Room{ roomtype: RoomType::StairsDown, discovered: false});
                    remaining -= 1;
                }
            }

            // Stairs up

            if z > 0 {
                for _ in 0..stair_count {
                    v.push(Room{ roomtype: RoomType::StairsUp, discovered: false});
                    remaining -= 1;
                }
            }

            // Items

            for _ in 0..item_count {
                v.push(Room{ roomtype: RoomType::Gold, discovered: false});
                v.push(Room{ roomtype: RoomType::Pool, discovered: false});
                v.push(Room{ roomtype: RoomType::Chest, discovered: false});
                v.push(Room{ roomtype: RoomType::Flares, discovered: false});
                v.push(Room{ roomtype: RoomType::Warp, discovered: false});
                v.push(Room{ roomtype: RoomType::Sinkhole, discovered: false});
                v.push(Room{ roomtype: RoomType::CrystalOrb, discovered: false});
                v.push(Room{ roomtype: RoomType::Book, discovered: false});
                remaining -= 8;
            }

            // Monsters {
            for i in 0..(MONSTER_COUNT-1) { // -1 to not count the Vendors
                v.push(Room{ roomtype: RoomType::Monster(Monster::new(i)), discovered: false});
                remaining -= 1;
            }

            // Fill the rest with empty

            for _ in 0..remaining {
                v.push(Room{ roomtype: RoomType::Empty, discovered: false});
            }

            v.shuffle(&mut rng);

            // Fix up the levels
            for y in 0..ysize {
                for x in 0..xsize {
                    let i = y * xsize + x;

                    // Swap the entrance
                    if v[i].roomtype == RoomType::Entrance {
                        let i2 = 0 * xsize + entrance_x;

                        v.swap(i, i2);
                    }
                }
            }


            levels.push(v);
        }

        Dungeon{levels, xsize, ysize, zsize}
    }

    fn entrance_x(&self) -> usize {
        return (self.xsize - 1) / 2;
    }

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
                    RoomType::Warp => print!("W"),
                    RoomType::Sinkhole => print!("S"),
                    RoomType::CrystalOrb => print!("O"),
                    RoomType::Book => print!("B"),
                    RoomType::Monster(_) => print!("M"),
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

fn main() {
    let dungeon = Dungeon::new(8, 8, 8);
    let player = Player::new(dungeon.entrance_x(), 0, 0);

    map(&dungeon, &player, true);
}
