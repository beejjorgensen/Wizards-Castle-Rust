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
    //Monster(Monster)
}

//#[derive(Debug)]
//struct Monster {
//    monster_type: usize
//}

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

        for z in 0..zsize {
            let mut v = Vec::new();
            let mut remaining = area;

            if z == 0 {
                v.push(Room{ roomtype: RoomType::Entrance, discovered: false});
                remaining -= 1;
            }

            if z < zsize - 1 {
                for _ in 0..stair_count {
                    v.push(Room{ roomtype: RoomType::StairsDown, discovered: false});
                    remaining -= 1;
                }
            }

            if z > 0 {
                for _ in 0..stair_count {
                    v.push(Room{ roomtype: RoomType::StairsUp, discovered: false});
                    remaining -= 1;
                }
            }

            for _ in 0..item_count {
                v.push(Room{ roomtype: RoomType::Gold, discovered: false});
                v.push(Room{ roomtype: RoomType::Pool, discovered: false});
                remaining -= 2;
            }

            for _ in 0..remaining {
                v.push(Room{ roomtype: RoomType::Empty, discovered: false});
            }

            v.shuffle(&mut rng);

            // Fix up the levels
            for y in 0..ysize {
                for x in 0..xsize {
                    let i = y * xsize + x;

                    let entrance_x: usize;
                    let entrance_y: usize;

                    // Find the entrance
                    if v[i].roomtype == RoomType::Entrance {
                    }
                }
            }


            levels.push(v);
        }

        Dungeon{levels, xsize, ysize, zsize}
    }

    fn room_at(&self, x: usize, y: usize, z: usize) -> &Room { // TODO: Result
        let i = y * self.xsize + x;

        &self.levels[z][i]
    }

    #[allow(dead_code)]
    fn map_dump(&self, player: Player) {
        self.map_base(player, true)
    }

    #[allow(dead_code)]
    fn map(&self, player: Player) {
        self.map_base(player, false)
    }

    fn map_base(&self, player: Player, show_all: bool) {
        let z = player.z;

        for y in 0..self.ysize {
            for x in 0..self.xsize {

                if x >= 1 {
                    print!("   ");
                }

                let r = self.room_at(x, y, z);

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
                        //RoomType::Monster(_) => print!("M"),
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

fn main() {
    let dungeon = Dungeon::new(8, 8, 8);
    let player = Player::new(3, 0, 0);

    dungeon.map_dump(player);
    //dungeon.map(player);
}
