extern crate rand;

use self::rand::Rng;
use self::rand::thread_rng;
use self::rand::seq::SliceRandom;

use room::{Room, RoomType};
use monster::Monster;
use curse::Curse;
use treasure::Treasure;

#[derive(Debug)]
pub struct  Dungeon {
    pub levels: Vec<Vec<Room>>,
    pub xsize: usize,
    pub ysize: usize,
    pub zsize: usize,
}

impl Dungeon {

    pub fn new(xsize: usize, ysize: usize, zsize:usize) -> Dungeon {
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
            let monsters_to_place = ::monster::MONSTER_COUNT - 1; // -1 to not count the Vendors

            let monster_with_runestaff = rng.gen_range(0, monsters_to_place);

            for i in 0..monsters_to_place {
                let has_runestaff = i == monster_with_runestaff && z == runestaff_level;

                this_level.push(Room{ roomtype: RoomType::Monster(Monster::new(i, has_runestaff)), ..Default::default() });
            }

            levels.push(this_level);
        }

        // Add curse rooms
        for i in 0..::curse::CURSE_COUNT {
            let curse_level = rng.gen_range(0, zsize);

            let curse = Curse::get_curse_by_id(i);

            levels[curse_level].push(Room { curse, ..Default::default() })
        }

        for i in 0..::treasure::TREASURE_COUNT {
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

    /// Get the entrance x position
    pub fn entrance_x(&self) -> usize {
        return (self.xsize - 1) / 2;
    }

    /// Return a reference to the room at a location
    pub fn room_at(&self, x: usize, y: usize, z: usize) -> &Room { // TODO: Result
        let i = y * self.xsize + x;

        &self.levels[z][i]
    }

}
