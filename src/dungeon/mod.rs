extern crate rand;

use self::rand::Rng;
use self::rand::thread_rng;
use self::rand::seq::SliceRandom;

use room::{Room, RoomType};
use monster::{Monster, MonsterType};
use curse::Curse;
use treasure::Treasure;

#[derive(Debug)]
pub struct Dungeon {
    pub levels: Vec<Vec<Room>>,
    pub xsize: u32,
    pub ysize: u32,
    pub zsize: u32,
}

impl Dungeon {

    pub fn new(xsize: u32, ysize: u32, zsize: u32) -> Dungeon {
        let mut levels: Vec<Vec<Room>> = Vec::new();

        let mut rng = thread_rng();

        let area = xsize * ysize;

        let stair_count = area / 32; // 2 in 8x8
        let item_count = area / 21; // 3 in 8x8
        let vendor_count = area / 21; // 3 in 8x8

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
            let monsters_to_place = [
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
                // Not counting Vendors
            ];

            let monster_count = monsters_to_place.len();

            let monster_with_runestaff = rng.gen_range(0, monster_count);

            for i in 0..monster_count {
                let has_runestaff = i == monster_with_runestaff && z == runestaff_level;

                this_level.push(Room{ roomtype: RoomType::Monster(Monster::new(monsters_to_place[i], has_runestaff)), ..Default::default() });
            }

            // Vendors
            for _ in 0..vendor_count {
                this_level.push(Room{ roomtype: RoomType::Monster(Monster::new(MonsterType::Vendor, false)), ..Default::default() });
            }

            levels.push(this_level);
        }

        // Add curse rooms
        for i in 0..::curse::CURSE_COUNT {
            let curse_level = rng.gen_range(0, zsize) as usize;

            let curse = Curse::get_curse_by_id(i);

            levels[curse_level].push(Room { curse, ..Default::default() })
        }

        for i in 0..::treasure::TREASURE_COUNT {
            let treasure_level = rng.gen_range(0, zsize) as usize;

            levels[treasure_level].push(Room { roomtype: RoomType::Treasure(Treasure::new(i)), ..Default::default() })
        }

        // Run through the levels, padding them with empty rooms, shuffling
        // them, and moving certain rooms to their proper positions.

        for z in 0..zsize as usize {
            // Fill the rest with empty
            while levels[z].len() < area as usize {
                levels[z].push(Room{ roomtype: RoomType::Empty, ..Default::default() });
            }

            // Shuffle the level
            levels[z].shuffle(&mut rng);

            // Fix up the entrance
            for y in 0..ysize as usize {
                for x in 0..xsize as usize {
                    let i = y * xsize as usize + x;

                    // Swap the entrance
                    if levels[z][i].roomtype == RoomType::Entrance {
                        let i2 = (0 * xsize + entrance_x) as usize;

                        levels[z].swap(i, i2);
                    }
                }
            }

            /*
            // cheater code to reveal Runestaff location
            if z == runestaff_level {
                for y in 0..ysize as usize {
                    for x in 0..xsize as usize {
                        let i = y as usize * xsize + x as usize;

                        if let RoomType::Monster(ref m) = levels[z][i].roomtype {
                            if m.has_runestaff() {
                                println!("\n>>> RUNESTAFF IS AT {},{},{} <<<\n", x+1, y+1, z+1);
                            }
                        }
                    }
                }
            }

            // cheater code to reveal Orb of Zot location
            if z == orb_of_zot_level {
                for y in 0..ysize as usize {
                    for x in 0..xsize as usize {
                        let i = y as usize * xsize + x as usize;

                        if let RoomType::Warp(oz) = levels[z][i].roomtype {
                            if oz {
                                println!("\n>>> ORB OF ZOT IS AT {},{},{} <<<\n", x+1, y+1, z+1);
                            }
                        }
                    }
                }
            }
            */

            // Fix up the stairs up
            if z > 0 {
                let mut downs = Vec::new();
                let mut ups = Vec::new();

                for i in 0..area as usize {
                    if levels[z-1][i].roomtype == RoomType::StairsDown {
                        downs.push(i);
                    }
                }

                for i in 0..area as usize {
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

    /// Get the entrance x position
    pub fn entrance_x(&self) -> u32 {
        return (self.xsize - 1) / 2;
    }

    /// Return a reference to the room at a location
    pub fn room_at(&self, x: u32, y: u32, z: u32) -> &Room { // TODO: Result
        let i = y * self.xsize + x;

        &self.levels[z as usize][i as usize]
    }

    /// Return a reference to the room at a location
    pub fn room_at_mut(&mut self, x: u32, y: u32, z: u32) -> &mut Room { // TODO: Result
        let i = y * self.xsize + x;

        &mut self.levels[z as usize][i as usize]
    }

    pub fn discover(&mut self, x: u32, y: u32, z: u32) {
        let i = y * self.xsize + x;

        self.levels[z as usize][i as usize].discovered = true;
    }
}
