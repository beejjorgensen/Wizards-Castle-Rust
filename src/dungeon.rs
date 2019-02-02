extern crate rand;

use self::rand::seq::SliceRandom;
use self::rand::thread_rng;
use self::rand::Rng;

use crate::curse::Curse;
use crate::monster::{Monster, MonsterType};
use crate::room::{Room, RoomType};
use crate::treasure::Treasure;

#[derive(Debug)]
pub struct Dungeon {
    levels: Vec<Vec<Room>>,
    xsize: u32,
    ysize: u32,
    zsize: u32,
    orb_of_zot: (u32, u32, u32),
    runestaff: (u32, u32, u32),
}

impl Dungeon {
    pub fn new(xsize: u32, ysize: u32, zsize: u32) -> Dungeon {
        let mut levels: Vec<Vec<Room>> = Vec::new();

        let area = xsize * ysize;

        let mut rng = thread_rng();

        let orb_of_zot_level = rng.gen_range(0, zsize);

        // Add all necessary elements to the level
        for z in 0..zsize {
            let mut this_level = Vec::new();

            Dungeon::place_ent_stairs(&mut this_level, z, zsize, area);
            Dungeon::place_items(&mut this_level, orb_of_zot_level, z, area);
            Dungeon::place_monsters_vendors(&mut this_level, z, zsize, area);

            levels.push(this_level);
        }

        // Curses and treasures
        Dungeon::place_curse_treasure(&mut levels, zsize);

        // Run through the levels, padding them with empty rooms, shuffling
        // them, and moving certain rooms to their proper positions.

        for l in &mut levels {
            // Fill the rest with empty
            while l.len() < area as usize {
                l.push(Room {
                    roomtype: RoomType::Empty,
                    ..Default::default()
                });
            }

            // Shuffle the level
            l.shuffle(&mut rng);
        }

        // Fix up the stairs and entrance
        Dungeon::place_fixup(&mut levels, xsize, ysize, zsize, area);

        let mut orb_of_zot = (0, 0, 0);
        let mut runestaff = (0, 0, 0);

        // Find the orb of zot
        //for z in 0..zsize as usize {
        // Clippy, you crazy for wanting this line instead:
        for (z, l) in levels.iter().enumerate().take(zsize as usize) {
            // Find Orb of Zot (for gazing into orbs)
            if z as u32 == orb_of_zot_level {
                for y in 0..ysize {
                    for x in 0..xsize {
                        let i = (y * xsize + x) as usize;

                        if let RoomType::Warp(oz) = l[i].roomtype {
                            if oz {
                                orb_of_zot = (x, y, z as u32);
                            }
                        }
                    }
                }
            }

            // Find runestaff
            for y in 0..ysize as usize {
                for x in 0..xsize as usize {
                    let i = y * xsize as usize + x;

                    if let RoomType::Monster(ref m) = levels[z][i].roomtype {
                        if m.has_runestaff() {
                            runestaff = (x as u32, y as u32, z as u32);
                        }
                    }
                }
            }
        }

        Dungeon {
            levels,
            xsize,
            ysize,
            zsize,
            orb_of_zot,
            runestaff,
        }
    }

    /// Place the entryway and the stairs
    fn place_ent_stairs(this_level: &mut Vec<Room>, z: u32, zsize: u32, area: u32) {
        let stair_count = area / 32; // 2 in 8x8

        // Entrance
        if z == 0 {
            this_level.push(Room {
                roomtype: RoomType::Entrance,
                discovered: true,
                ..Default::default()
            });
        }

        // Stairs down
        if z < zsize - 1 {
            for _ in 0..stair_count {
                this_level.push(Room {
                    roomtype: RoomType::StairsDown,
                    ..Default::default()
                });
            }
        }

        // Stairs up
        if z > 0 {
            for _ in 0..stair_count {
                this_level.push(Room {
                    roomtype: RoomType::StairsUp,
                    ..Default::default()
                });
            }
        }
    }

    /// Place the items in the dungeon
    fn place_items(this_level: &mut Vec<Room>, orb_of_zot_level: u32, z: u32, area: u32) {
        let item_count = area / 21; // 3 in 8x8

        // Items
        for i in 0..item_count {
            let orb_of_zot_warp = i == 0 && z == orb_of_zot_level;

            this_level.push(Room {
                roomtype: RoomType::Gold,
                ..Default::default()
            });
            this_level.push(Room {
                roomtype: RoomType::Pool,
                ..Default::default()
            });
            this_level.push(Room {
                roomtype: RoomType::Chest,
                ..Default::default()
            });
            this_level.push(Room {
                roomtype: RoomType::Flares,
                ..Default::default()
            });
            this_level.push(Room {
                roomtype: RoomType::Warp(orb_of_zot_warp),
                ..Default::default()
            });
            this_level.push(Room {
                roomtype: RoomType::Sinkhole,
                ..Default::default()
            });
            this_level.push(Room {
                roomtype: RoomType::CrystalOrb,
                ..Default::default()
            });
            this_level.push(Room {
                roomtype: RoomType::Book,
                ..Default::default()
            });
        }
    }

    /// Place monsters and vendors in the dungeon
    fn place_monsters_vendors(this_level: &mut Vec<Room>, z: u32, zsize: u32, area: u32) {
        let vendor_count = area / 21; // 3 in 8x8
        let monster_count = area / 5; // 12 in 8x8

        let mut rng = thread_rng();

        let runestaff_level = rng.gen_range(0, zsize);

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

        let num_monsters = monsters_to_place.len();

        let monster_with_runestaff = rng.gen_range(0, monster_count) as usize;

        for i in 0..monster_count as usize {
            let has_runestaff = i == monster_with_runestaff && z == runestaff_level;

            let m_num = i % num_monsters;

            this_level.push(Room {
                roomtype: RoomType::Monster(Monster::new(monsters_to_place[m_num], has_runestaff)),
                ..Default::default()
            });
        }

        // Vendors
        for _ in 0..vendor_count {
            this_level.push(Room {
                roomtype: RoomType::Monster(Monster::new(MonsterType::Vendor, false)),
                ..Default::default()
            });
        }
    }

    /// Place curses and treasures
    fn place_curse_treasure(levels: &mut Vec<Vec<Room>>, zsize: u32) {
        let mut rng = thread_rng();

        // Add curse rooms
        for i in 0..crate::curse::CURSE_COUNT {
            let curse_level = rng.gen_range(0, zsize) as usize;

            let curse = Curse::get_curse_by_id(i);

            levels[curse_level].push(Room {
                curse,
                ..Default::default()
            })
        }

        // Add treasures
        for i in 0..crate::treasure::TREASURE_COUNT {
            let treasure_level = rng.gen_range(0, zsize) as usize;

            levels[treasure_level].push(Room {
                roomtype: RoomType::Treasure(Treasure::new(i)),
                ..Default::default()
            })
        }
    }

    /// Fix stairs and entrance on levels
    fn place_fixup(levels: &mut Vec<Vec<Room>>, xsize: u32, ysize: u32, zsize: u32, area: u32) {
        let entrance_x = (xsize - 1) / 2;

        for z in 0..zsize as usize {
            // Fix up the entrance
            for y in 0..ysize as usize {
                for x in 0..xsize as usize {
                    let i = y * xsize as usize + x;

                    // Swap the entrance
                    if levels[z][i].roomtype == RoomType::Entrance {
                        let i2 = (/*0 * xsize + */entrance_x) as usize;

                        levels[z].swap(i, i2);
                    }
                }
            }

            // Fix up the stairs up
            if z > 0 {
                let mut downs = Vec::new();
                let mut ups = Vec::new();

                for i in 0..area as usize {
                    if levels[z - 1][i].roomtype == RoomType::StairsDown {
                        downs.push(i);
                    }
                }

                for i in 0..area as usize {
                    if levels[z][i].roomtype == RoomType::StairsUp {
                        ups.push(i);
                    }
                }

                while !ups.is_empty() {
                    let up_i = ups.pop().unwrap();
                    let down_i = downs.pop().unwrap();

                    levels[z].swap(up_i, down_i);
                }
            }
        }
    }

    /// Get the entrance x position
    pub fn entrance_x(&self) -> u32 {
        (self.xsize - 1) / 2
    }

    /// Return a reference to the room at a location
    pub fn room_at(&self, x: u32, y: u32, z: u32) -> &Room {
        // TODO: Result
        // TODO modify this so that it returns unknown for undiscovered rooms
        let i = y * self.xsize + x;

        &self.levels[z as usize][i as usize]
    }

    /// Return a reference to the room at a location
    pub fn room_at_mut(&mut self, x: u32, y: u32, z: u32) -> &mut Room {
        // TODO: Result
        let i = y * self.xsize + x;

        &mut self.levels[z as usize][i as usize]
    }

    /// Discover a room
    pub fn discover(&mut self, x: u32, y: u32, z: u32) {
        let i = y * self.xsize + x;

        self.levels[z as usize][i as usize].discovered = true;
    }

    /// Return x dimension
    pub fn xsize(&self) -> &u32 {
        &self.xsize
    }

    /// Return y dimension
    pub fn ysize(&self) -> &u32 {
        &self.ysize
    }

    /// Return z dimension
    pub fn zsize(&self) -> &u32 {
        &self.zsize
    }

    /// Return Orb of Zot location
    pub fn orb_of_zot_location(&self) -> (u32, u32, u32) {
        self.orb_of_zot
    }

    /// Return Runestaff location
    pub fn runestaff_location(&self) -> (u32, u32, u32) {
        self.runestaff
    }
}
