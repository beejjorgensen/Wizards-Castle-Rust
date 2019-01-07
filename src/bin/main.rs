extern crate wizardscastle;
extern crate rand; 

use std::io::{stdin,stdout,Write};

use self::rand::Rng;
use self::rand::rngs::ThreadRng;
use self::rand::thread_rng;

use wizardscastle::game::{Game, Direction, Stairs, Event, CombatEvent, DrinkEvent, GameState};
use wizardscastle::room::RoomType;
use wizardscastle::player::{Race, Gender, Stat};
use wizardscastle::armor::{Armor, ArmorType};
use wizardscastle::weapon::{Weapon, WeaponType};
use wizardscastle::treasure::TreasureType;
use wizardscastle::monster::MonsterType;
use wizardscastle::error::Error;

struct UI {
    game: Game,
    rng: ThreadRng,
    turn_count: u32,
}

impl UI {
    /// Return a random monster name
    fn rand_monster_str(&mut self) -> String {
        let monster = [
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
        ];

        let i = self.rng.gen_range(0, monster.len());

        UI::monster_name(monster[i])
    }

    fn monster_name(m: MonsterType) -> String {
        match m {
            MonsterType::Kobold => String::from("KOBOLD"),
            MonsterType::Orc => String::from("ORC"),
            MonsterType::Wolf => String::from("WOLF"),
            MonsterType::Goblin => String::from("GOBLIN"),
            MonsterType::Ogre => String::from("OGRE"),
            MonsterType::Troll => String::from("TROLL"),
            MonsterType::Bear => String::from("BEAR"),
            MonsterType::Minotaur => String::from("MINOTAUR"),
            MonsterType::Gargoyle => String::from("GARGOYLE"),
            MonsterType::Chimera => String::from("CHIMERA"),
            MonsterType::Balrog => String::from("BALROG"),
            MonsterType::Dragon => String::from("DRAGON"),
            MonsterType::Vendor => String::from("VENDOR"),
        }
    }

    fn stat_name(s: Stat) -> String {
        match s {
            Stat::Strength => String::from("STRENGTH"),
            Stat::Intelligence => String::from("INTELLIGENCE"),
            Stat::Dexterity => String::from("DEXTERITY"),
        }
    }

    fn weapon_name(w: WeaponType) -> String {
        match w {
            WeaponType::None => String::from("NO WEAPON"),
            WeaponType::Dagger => String::from("DAGGER"),
            WeaponType::Mace => String::from("MACE"),
            WeaponType::Sword => String::from("SWORD"),
        }
    }

    fn armor_name(a: ArmorType) -> String {
        match a {
            ArmorType::None => String::from("NO ARMOR"),
            ArmorType::Leather => String::from("LEATHER"),
            ArmorType::Chainmail => String::from("CHAINMAIL"),
            ArmorType::Plate => String::from("PLATE"),
        }
    }

    fn stair_name(s: Stairs) -> String {
        match s {
            Stairs::Up => String::from("UP"),
            Stairs::Down => String::from("DOWN"),
        }
    }

    fn treasure_name(t: &TreasureType) -> String {
        match t {
            TreasureType::RubyRed => String::from("THE RUBY RED"),
            TreasureType::NornStone => String::from("THE NORN STONE"),
            TreasureType::PalePearl => String::from("THE PALE PEARL"),
            TreasureType::OpalEye => String::from("THE OPAL EYE"),
            TreasureType::GreenGem => String::from("THE GREEN GEM"),
            TreasureType::BlueFlame => String::from("THE BLUE FLAME"),
            TreasureType::Palintir => String::from("THE PALINTIR"),
            TreasureType::Silmaril => String::from("THE SILMARIL"),
        }
    }

    fn room_name(r: &RoomType) -> String {
        match r {
            RoomType::Empty => String::from("AN EMPTY ROOM"),
            RoomType::Entrance => String::from("THE ENTRANCE"),
            RoomType::StairsDown => String::from("STAIRS GOING DOWN"),
            RoomType::StairsUp => String::from("STAIRS GOING UP"),
            RoomType::Gold => String::from("GOLD PIECES"),
            RoomType::Pool => String::from("A POOL"),
            RoomType::Chest => String::from("A CHEST"),
            RoomType::Flares => String::from("FLARES"),
            RoomType::Warp(_) => String::from("A WARP"),
            RoomType::Sinkhole => String::from("A SINKHOLE"),
            RoomType::CrystalOrb => String::from("A CRYSTAL ORB"),
            RoomType::Book => String::from("A BOOK"),
            RoomType::Monster(m) => {
                    let mon_str = UI::monster_name(m.monster_type());
                    format!("{} {}", UI::get_article(&mon_str), mon_str)
                }
            RoomType::Treasure(t) => {
                format!("{}", UI::treasure_name(t.treasure_type()))
            }
        }
    }

    fn gender_name(g: Gender) -> String {
        match g {
            Gender::Female => String::from("FEMALE"),
            Gender::Male => String::from("MALE"),
        }
    }

    fn starts_with_vowel(s: &str) -> bool {
        if let Some(c) = String::from(s).to_uppercase().chars().next() {
            return c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U';
        }

        return false;
    }

    fn get_article(s: &str) -> String {
        if UI::starts_with_vowel(s) {
            return String::from("AN");
        }

        String::from("A")
    }

    /// Move a direction
    fn move_dir(&mut self, dir: Direction) {
        self.game.move_dir(dir)
    }

    /// Take some stairs
    fn move_stairs(&mut self, stairs: Stairs) {
        match self.game.move_stairs(stairs) {
            Err(_) => println!("** OH {}, NO STAIRS GOING {} IN HERE", self.race_str(), UI::stair_name(stairs)),
            Ok(_) => (),
        };
    }

    // Input a coordinate, 1-8
    fn input_coord(prompt:&str) -> u32 {
        let mut coord = 0;
        let mut got_num = false;

        while !got_num {
            let str = UI::get_input(Some(prompt));

            match str.parse::<u32>() {
                Ok(v) => {
                    if v >= 1 && v <= 8 {
                        got_num = true;
                        coord = v;
                    }
                },
                Err(_) => (),
            }

            if !got_num {
                println!("\n** TRY A NUMBER FROM 1 TO 8\n");
            }
        }

        coord

    }

    /// Teleport
    fn teleport(&mut self) {

        if !self.game.can_teleport() {
            println!("\n** YOU CAN'T TELEPORT WITHOUT THE RUNESTAFF!");
            return;
        }

        let x = UI::input_coord("X-COORD (1 = FAR WEST  8 = FAR EAST )? ");
        let y = UI::input_coord("Y-COORD (1 = FAR NORTH 8 = FAR SOUTH)? ");
        let z = UI::input_coord("Z-COORD (1 = TOP       8 = BOTTOM   )? ");

        match self.game.teleport(x - 1, y - 1, z - 1) { // back to 0-based
            Ok(found_orb_of_zot) => {
                if found_orb_of_zot {
                    println!("\nGREAT UNMITIGATED ZOT!\n");
                    println!("YOU JUST FOUND THE ORB OF ZOT!\n");
                    println!("THE RUNESTAFF IS GONE");
                }
            },
            Err(err) => panic!("{:#?}", err),
        }
    }

    /// Drink
    fn drink(&mut self) {
        let s;

        match self.game.drink() {
            Ok(DrinkEvent::Stronger) => {
                s = String::from("FEEL STRONGER");
            },
            Ok(DrinkEvent::Weaker) => {
                s = String::from("FEEL WEAKER");
            },
            Ok(DrinkEvent::Smarter) => {
                s = String::from("FEEL SMARTER");
            },
            Ok(DrinkEvent::Dumber) => {
                s = String::from("FEEL DUMBER");
            },
            Ok(DrinkEvent::Nimbler) => {
                s = String::from("FEEL NIMBLER");
            },
            Ok(DrinkEvent::Clumsier) => {
                s = String::from("FEEL CLUMSIER");
            },
            Ok(DrinkEvent::ChangeRace) => {
                s = format!("TURN INTO A {}", self.race_str());
            },
            Ok(DrinkEvent::ChangeGender) => {
                s = format!("TURN INTO A {} {}",
                    UI::gender_name(*self.game.player_gender()), self.race_str());
            },
            Err(Error::CantGo) => {
                println!("** IF YOU WANT A DRINK, FIND A POOL");
                return;
            },
            Err(err) => panic!("{:#?}", err),
        }

        print!("YOU TAKE A DRINK AND {}\n", s);
    }

    /// Print a map
    fn map(&mut self, show_all: bool) {
        let z = self.game.player_z();

        for y in 0..self.game.dungeon_ysize() {
            for x in 0..self.game.dungeon_xsize() {

                if x >= 1 {
                    print!("   ");
                }

                let r = self.game.dungeon_room_at(x, y, z);

                let bracket = x == self.game.player_x() && y == self.game.player_y();

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
                        RoomType::Monster(ref m) => {
                            if m.monster_type() == MonsterType::Vendor {
                                print!("V");
                            } else {
                                print!("M");
                            }
                        },
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

    fn race_str(&self) -> &str {
        match self.game.player_race() {
            Race::Hobbit => "HOBBIT",
            Race::Elf => "ELF",
            Race::Human => "HUMAN",
            Race::Dwarf => "DWARF",
        }
    }

    /// Input a line of text
    fn get_input(prompt: Option<&str>) -> String {
        let mut s = String::new();

        if let Some(s) = prompt {
            print!("{}", s);
            stdout().flush().unwrap();
        }

        stdin().read_line(&mut s).expect("Input error");

        s.trim().to_string().to_uppercase()
    }

    /// Print intro text
    ///
    /// Note: the original version lacked this preamble--it only appears in the
    /// magazine article. It was, however, included in the MBASIC port.
    ///
    fn intro() {
        println!("\n{:*^64}\n", "");

        println!("{:^64}\n", "* * * THE WIZARD'S CASTLE * * *");

        println!("{:*^64}\n", "");

        println!("MANY CYCLES AGO, IN THE KINGDOM OF N'DIC, THE GNOMIC");
        println!("WIZARD ZOT FORGED HIS GREAT *ORB OF POWER*. HE SOON");
        println!("VANISHED, LEAVING BEHIND HIS VAST SUBTERRANEAN CASTLE");
        println!("FILLED WITH ESURIENT MONSTERS, FABULOUS TREASURES, AND");
        println!("THE INCREDIBLE *ORB OF ZOT*. FROM THAT TIME HENCE, MANY");
        println!("A BOLD YOUTH HAS VENTURED INTO THE WIZARD'S CASTLE. AS");
        println!("OF NOW, *NONE* HAS EVER EMERGED VICTORIOUSLY! BEWARE!!\n");
    }

    /// Select the player's race and sex
    fn race_gender_select(&mut self) {
        let race = loop {

            println!("ALL RIGHT, BOLD ONE.");
            println!("YOU MAY BE AN ELF, DWARF, MAN, OR HOBBIT.\n");

            let race_str = UI::get_input(Some("YOUR CHOICE? "));

            match race_str.get(..1) {
                Some("H") => break Race::Hobbit,
                Some("E") => break Race::Elf,
                Some("M") => break Race::Human,
                Some("D") => break Race::Dwarf,
                _ => println!("** THAT WAS INCORRECT. PLEASE TYPE E, D, M, OR H.\n"),
            }
        };

        self.game.player_init(race);

        let gender = loop {
            let gender_str = UI::get_input(Some("\nWHICH SEX TO YOU PREFER? "));

            match gender_str.get(..1) {
                Some("M") => break Gender::Male,
                Some("F") => break Gender::Female,
                _ => println!("** CUTE {}, REAL CUTE. TRY M OR F.", self.race_str()),
            }
        };

        self.game.player_set_gender(gender);
    }

    /// Allocate additional stat points
    fn allocate_points(&mut self) {
        println!("\nOK {}, YOU HAVE THESE STATISTICS:\n", self.race_str());

        println!("STRENGTH= {} INTELLIGENCE= {} DEXTERITY= {}\n",
            self.game.player_stat(Stat::Strength),
            self.game.player_stat(Stat::Intelligence),
            self.game.player_stat(Stat::Dexterity));

        println!("AND {} OTHER POINTS TO ALLOCATE AS YOU WISH.\n", self.game.player_additional_points());

        let stats = vec!(Stat::Intelligence, Stat::Strength, Stat::Dexterity);
        let stat_names = vec!("INTELLIGENCE", "STRENGTH", "DEXTERITY");

        for i in 0..3 {
            let mut ok = false;

            while !ok {
                let s = UI::get_input(Some(&format!("HOW MANY POINTS DO YOU ADD TO {}? ", stat_names[i])));

                let points_to_add;
                
                match s.parse::<u32>() {
                    Ok(p) => points_to_add = p,
                    Err(_) => {
                        print!("\n** ");
                        continue;
                    },
                };

                if let Ok(_) = self.game.player_allocate_points(&stats[i], points_to_add) {
                    ok = true;
                } else {
                    print!("\n** ");
                    continue;
                }
            }

            if self.game.player_additional_points() == 0 {
                return;
            }
        }
    }

    /// Buy armor
    fn buy_armor(&mut self) {
        println!("\nOK, {}, YOU HAVE {} GOLD PIECES (GP's)\n", self.race_str(), self.game.player_gp());

        println!("HERE IS A LIST OF ARMOR YOU CAN BUY (WITH COST IN <>)\n");

        println!("PLATE<30> CHAINMAIL<20> LEATHER<10> NOTHING<0>");

        let _ = loop {
            let armor_str = UI::get_input(Some("\nYOUR CHOICE? "));

            match armor_str.get(..1) {

                Some("P") => break self.game.player_purchase_armor(ArmorType::Plate, false),
                Some("C") => break self.game.player_purchase_armor(ArmorType::Chainmail, false),
                Some("L") => break self.game.player_purchase_armor(ArmorType::Leather, false),
                Some("N") => break self.game.player_purchase_armor(ArmorType::None, false),
                _ => {
                    let mon_str = self.rand_monster_str();
                    let article = UI::get_article(&mon_str);

                    println!("\n** ARE YOU A {} OR {} {}? TYPE P,C,L OR N", self.race_str(), article, mon_str);
                },
            }
        };
    }

    /// Buy weapon
    fn buy_weapon(&mut self) {

        println!("\nOK, BOLD {}, YOU HAVE {} GP's LEFT\n", self.race_str(), self.game.player_gp());

        println!("HERE IS A LIST OF WEAPONS YOU CAN BUY (WITH COST IN <>)\n");

        println!("SWORD<30> MACE<20> DAGGER<10> NOTHING<0>");

        let _ = loop {
            let armor_str = UI::get_input(Some("\nYOUR CHOICE? "));

            match armor_str.get(..1) {

                Some("S") => break self.game.player_purchase_weapon(WeaponType::Sword, false),
                Some("M") => break self.game.player_purchase_weapon(WeaponType::Mace, false),
                Some("D") => break self.game.player_purchase_weapon(WeaponType::Dagger, false),
                Some("N") => break self.game.player_purchase_weapon(WeaponType::None, false),
                _ => println!("\n** IS YOUR IQ REALLY {}? TYPE S, M, D, OR N",
                    self.game.player_stat(Stat::Intelligence)),
            }
        };
    }

    /// Buy lamp
    fn buy_lamp(&mut self) {
        if !self.game.player_can_purchase_lamp() {
            return;
        }

        let _ = loop {
            let lamp_str = UI::get_input(Some("\nWANT TO BUY A LAMP FOR 20 GP's? "));

            match lamp_str.get(..1) {
                Some("Y") => break self.game.player_purchase_lamp(true),
                Some("N") => break self.game.player_purchase_lamp(false),
                _ => println!("\n** ANSWER YES OR NO"),
            }
        };
    }

    /// Buy flares
    fn buy_flares(&mut self) {
        let max_flares = self.game.player_max_flares();

        if max_flares == 0 {
            return;
        }

        println!("\nOK, {}, YOU HAVE {} GOLD PIECES LEFT\n", self.race_str(), self.game.player_gp());

        loop {
            let flare_str = UI::get_input(Some("FLARES COST 1 GP EACH, HOW MANY DO YOU WANT? "));

            let flare_count;
            
            match flare_str.parse::<u32>() {
                Ok(f) => flare_count = f,
                Err(_) => {
                    print!("** IF YOU DON'T WANT ANY JUST TYPE 0 (ZERO)\n\n");
                    continue;
                },
            };

            match self.game.player_purchase_flares(flare_count) {
                Ok(_) => break,
                Err(_) => {
                    print!("** YOU CAN ONLY AFFORD {}\n\n", max_flares);
                    continue;
                }
            }
        };
    }

    /// Print the player's location
    ///
    /// Note: the original game had a horizontal Y axis and a vertical X axis.
    /// This version reverses that.
    ///
    fn print_location(&self) {
        if self.game.player_is_blind() {
            return;
        }

        println!("YOU ARE AT ({},{}) LEVEL {}", self.game.player_x() + 1,
            self.game.player_y() + 1, self.game.player_z() + 1);
    }

    /// Print player stats
    fn print_stats(&self) {
        println!("ST={} IQ={} DX={} FLARES={} GP's={}",
            self.game.player_stat(Stat::Strength),
            self.game.player_stat(Stat::Intelligence),
            self.game.player_stat(Stat::Dexterity),
            self.game.player_flares(),
            self.game.player_gp());

        let w_name = UI::weapon_name(self.game.player_weapon_type());
        let a_name = UI::armor_name(self.game.player_armor_type());

        print!( "{} / {}", w_name, a_name);

        if self.game.player_has_lamp() {
            print!(" / A LAMP");
        }

        println!("\n");
    }

    /// Print the current room
    fn print_room(&mut self) {
        let room = self.game.room_at_player();

        let room_str = UI::room_name(&room.roomtype);

        println!("HERE YOU FIND {}\n", room_str);
    }

    // Attack a monster
    fn combat_attack(&mut self, m_art:&str, m_name:&str) {
        // Need to do this before the attack since the weapon might
        // break during it
        let weapon_type = self.game.player_weapon_type();

        match self.game.attack() {
            Ok(CombatEvent::NoWeapon) =>  {
                println!("\n** POUNDING ON {} {} WON'T HURT IT", m_art, m_name);
            },

            Ok(CombatEvent::Hit(_, weapon_broke, defeated, treasure, got_runestaff)) =>  {
                println!("\n  YOU HIT THE LOUSY {}", m_name);

                if weapon_broke {
                    println!("\nOH NO! YOUR {} BROKE", UI::weapon_name(weapon_type));
                }

                if defeated {
                    println!("\n{} {} LIES DEAD AT YOUR FEET", m_art, m_name);

                    // TODO random eating message

                    if got_runestaff {
                        println!("\nGREAT ZOT! YOU'VE FOUND THE RUNESTAFF");
                    }

                    println!("\nYOU NOW GET HIS HOARD OF {} GP's", treasure);
                }
            },
            
            Ok(CombatEvent::Miss) => {
                println!("\n  DRAT! MISSED");
            },

            // TODO: check for book hands

            Ok(any) => panic!("unexpected combat event {:#?}", any),

            Err(err) => panic!("error in combat {:#?}", err),
        }
    }

    /// Be attacked by a monster
    fn combat_be_attacked(&mut self) {
        match self.game.be_attacked() {
            Ok(CombatEvent::MonsterHit(_damage, _defeated, armor_destroyed)) => {
                println!("\n  OUCH! HE HIT YOU");

                if armor_destroyed {
                    println!("\nYOUR ARMOR IS DESTROYED - GOOD LUCK\n");
                }
            },

            Ok(CombatEvent::MonsterMiss) => {
                println!("\n  HAH! HE MISSED YOU");
            },

            Ok(any) => panic!("unexpected event while being attacked {:#?}", any),

            Err(err) => panic!("error in combat being attacked {:#?}", err),
        }

    }

    /// Retreat
    fn combat_retreat(&mut self) {
        match self.game.retreat() {
            Ok(_) => (),
            Err(err) => panic!("error retreating {:#?}", err),
        };
    }

    /// Retreat a direction after last monster attack
    fn combat_retreat_dir(&mut self) {
        println!("\n\nYOU HAVE ESCAPED\n");

        let dir;

        loop {
            let dir_str = UI::get_input(Some("\nDO YOU GO NORTH, SOUTH, EAST, OR WEST? "));

            match dir_str.get(..1) {
                Some("N") => {
                    dir = Direction::North;
                    break;
                }
                Some("S") => {
                    dir = Direction::South;
                    break;
                }
                Some("W") => {
                    dir = Direction::West;
                    break;
                }
                Some("E") => {
                    dir = Direction::East;
                    break;
                }
                _ => {
                    println!("\n** DON'T PRESS YOUR LUCK {}", self.race_str());
                }
            }
        }

        self.game.retreat_dir(dir);
    }

    /// Handle Bribe
    fn combat_bribe(&mut self) -> bool {
        let mut bribed = false;

        match self.game.bribe_proposition() {
            Ok(Some(t_type)) => {
                loop {
                    let tname = UI::treasure_name(&t_type);

                    let yn = UI::get_input(Some(&format!("\nI WANT {}, WILL YOU GIVE IT TO ME? ", tname)));

                    match yn.get(..1) {
                        Some("Y") => {
                            match self.game.bribe_accept() {
                                Ok(_) => {
                                    bribed = true;
                                },
                                Err(err) => {
                                    panic!("agree to bribe: {:#?}", err);
                                }
                            };
                            break;
                        },
                        Some("N") => {
                            match self.game.bribe_decline() {
                                Ok(_) => {
                                    bribed = false;
                                },
                                Err(err) => {
                                    panic!("disagree to bribe: {:#?}", err);
                                }
                            };
                            break;
                        },
                        _ => println!("\n** ANSWER YES OR NO"),
                    }
                };
            },
            Ok(None) => {
                println!("\n'ALL I WANT IS YOUR LIFE!'");
            },
            Err(err) => {
                panic!("bribe proposition: {:#?}", err);
            }
        };

        bribed
    }

    /// Handle combat
    fn combat(&mut self, monster_type: MonsterType) -> bool {

        let m_name = UI::monster_name(monster_type);
        let m_art = UI::get_article(&m_name);

        println!("YOU'RE FACING {} {}!", m_art, m_name);

        let mut in_combat = true;
        let mut retreated = false;

        while in_combat {

            match self.game.state() {
                GameState::PlayerAttack => {
                    print!("\nYOU MAY ATTACK OR RETREAT");

                    let can_bribe = self.game.bribe_possible();
                    let can_cast_spell = self.game.spell_possible();

                    if can_bribe {
                        print!(", OR BRIBE");
                    }

                    if can_cast_spell {
                        print!(", OR CAST A SPELL");
                    }

                    println!(".\n");

                    println!("\nYOUR STRENGTH IS {} AND DEXTERITY IS {}.\n",
                        self.game.player_stat(Stat::Strength),
                        self.game.player_stat(Stat::Dexterity));

                    let err_str = "\n** CHOOSE ONE OF THE OPTIONS LISTED.";

                    match UI::get_input(Some("YOUR CHOICE? ")).get(..1) {
                        Some("A") => self.combat_attack(&m_art, &m_name),
                        Some("R") => self.combat_retreat(),
                        Some("B") => {
                            if can_bribe {
                                if self.combat_bribe() {
                                    println!("\nOK, JUST DON'T TELL ANYONE");
                                }
                            } else {
                                println!("{}", err_str);
                            }
                        }
                        Some("C") => {
                            if can_cast_spell {
                                // TODO
                            } else {
                                println!("{}", err_str)
                            }
                        }
                        _ => println!("{}", err_str),
                    }
                },

                GameState::MonsterAttack => {
                    println!("\nTHE {} ATTACKS", m_name);

                    self.combat_be_attacked();
                },

                GameState::Retreat => {
                    self.combat_retreat_dir();
                    retreated = true;
                },

                GameState::Move => {
                    in_combat = false;
                },

                GameState::Dead => {
                    in_combat = false;
                },

                any => panic!("unknown state during combat {:#?}", any),
            }

        } // while in_combat

        retreated
    }

    /// Print out the game over summary
    pub fn game_summary(&self) {
        match self.game.state() {
            GameState::Dead => {
                println!("\n\nA NOBLE EFFORT, OH FORMERLY LIVING {}\n", self.race_str());

                print!("YOU DIED FROM LACK OF ");
                if self.game.player_stat(Stat::Strength) == 0 {
                    println!("STRENGTH");
                }
                else if self.game.player_stat(Stat::Intelligence) == 0 {
                    println!("INTELLIGENCE");
                }
                else if self.game.player_stat(Stat::Dexterity) == 0 {
                    println!("DEXTERITY");
                }

                println!("\nWHEN YOU DIED YOU HAD:\n");
            },

            GameState::Exit => {
                let win = self.game.player_has_orb_of_zot();

                print!("YOU LEFT THE CASTLE WITH");

                if !win {
                    print!("OUT");
                }

                println!(" THE ORB OF ZOT\n\n");

                if win {
                    println!("A GLORIOUS VICTORY!\n");
                    println!("YOU ALSO GOT OUT WITH THE FOLLOWING:\n");
                } else {
                    println!("A LESS THAN AWE-INSPIRING DEFEAT.\n");
                    println!("WHEN YOU LEFT THE CASTLE YOU HAD:\n");
                }

                println!("YOUR MISERABLE LIFE");
            },

            any => panic!("unexpected game state at end {:#?}", any),
        }

        // List treasures
        for t in self.game.player_get_treasures() {
            println!("{}", UI::treasure_name(t));
        }

        // Show weapon
        println!("{}", UI::weapon_name(self.game.player_weapon_type()));

        // Show armor
        println!("{}", UI::armor_name(self.game.player_armor_type()));

        // Show lamp
        if self.game.player_has_lamp() {
            println!("A LAMP");
        }

        // Show flares
        println!("{} FLARES", self.game.player_flares());

        // Show GPs
        println!("{} GP's", self.game.player_gp());

        // Show Runestaff
        if self.game.player_has_runestaff() {
            println!("THE RUNESTAFF");
        }

        // Show turns
        println!("\nAND IT TOOK YOU {} TURNS!\n", self.turn_count);
    }
    
    /// Sell treasures to a vendor
    fn vendor_trade_treasures(&mut self) {
        let treasures = self.game.player_get_treasures().clone();

        if treasures.len() == 0 {
            return;
        }

        println!();

        for t in treasures {
            let price = match self.game.vendor_treasure_offer(t) {
                Ok(p) => p,
                Err(err) => panic!("vendor treasure offer: {:#?}", err),
            };

            loop {
                let yn = UI::get_input(Some(&format!("DO YOU WANT TO SELL {} FOR {} GP's? ", UI::treasure_name(&t), price)));

                match yn.get(..1) {
                    Some("Y") => {
                        match self.game.vendor_treasure_accept() {
                            Ok(_) => (),
                            Err(err) => panic!("vendor treasure accept: {:#?}", err),
                        }
                        break;
                    },
                    Some("N") => {
                        match self.game.vendor_treasure_reject() {
                            Ok(_) => (),
                            Err(err) => panic!("vendor treasure accept: {:#?}", err),
                        }
                        break;
                    }
                    _ => {
                        println!("\n** ANSWER YES OR NO");
                    }
                }
            }
        }
    }

    /// Trade armor
    fn vendor_trade_armor(&mut self) {
        let leather_cost = Armor::cost(ArmorType::Leather, true);

        if self.game.player_gp() < leather_cost {
            return;
        }

        let chainmail_cost = Armor::cost(ArmorType::Chainmail, true);
        let plate_cost = Armor::cost(ArmorType::Plate, true);

        println!("\nOK, {}, YOU HAVE {} GOLD PIECES AND {}",
            self.race_str(), self.game.player_gp(),
            UI::armor_name(self.game.player_armor_type()));

        println!("\nHERE IS A LIST OF ARMOR YOU CAN BUY");

        print!("\nNOTHING<0> LEATHER<{}>", leather_cost);

        if self.game.player_gp() >= chainmail_cost {
            print!(" CHAINMAIL<{}>", chainmail_cost)
        }

        if self.game.player_gp() >= plate_cost {
            print!(" PLATE<{}>", plate_cost)
        }

        println!();

        loop {
            let armor_str = UI::get_input(Some("\nYOUR CHOICE? "));

            match armor_str.get(..1) {

                Some("P") => {
                    match self.game.player_purchase_armor(ArmorType::Plate, true) {
                        Ok(_) => break,
                        Err(Error::NotEnoughGP) => println!("\n** YOU CAN'T AFFORD PLATE"),
                        _ => (),
                    }
                },
                Some("C") => {
                    match self.game.player_purchase_armor(ArmorType::Chainmail, true) {
                        Ok(_) => break,
                        Err(Error::NotEnoughGP) => println!("\n** YOU HAVEN'T GOT THAT MUCH CASH"),
                        _ => (),
                    }
                },
                Some("L") => {
                    // If we get to this point we already had enough to buy leather
                    let _ = self.game.player_purchase_armor(ArmorType::Leather, true);
                    break;
                },
                Some("N") => break,
                _ => {
                    println!("\n** DON'T BE SILLY. CHOOSE A SELECTION");
                },
            }
        }
    }

    /// Trade armor
    fn vendor_trade_weapons(&mut self) {
        let dagger_cost = Weapon::cost(WeaponType::Dagger, true);

        if self.game.player_gp() < dagger_cost {
            return;
        }

        let mace_cost = Weapon::cost(WeaponType::Mace, true);
        let sword_cost = Weapon::cost(WeaponType::Sword, true);

        println!("\nYOU HAVE {} GP's LEFT WITH {} IN HAND",
            self.game.player_gp(),
            UI::weapon_name(self.game.player_weapon_type()));

        println!("\nHERE IS A LIST OF ARMOR YOU CAN BUY");

        print!("\nNOTHING<0> DAGGER<{}>", dagger_cost);

        if self.game.player_gp() >= mace_cost {
            print!(" MACE<{}>", mace_cost)
        }

        if self.game.player_gp() >= sword_cost {
            print!(" SWORD<{}>", sword_cost)
        }

        println!();

        loop {
            let armor_str = UI::get_input(Some("\nYOUR CHOICE? "));

            match armor_str.get(..1) {

                Some("S") => {
                    match self.game.player_purchase_weapon(WeaponType::Sword, true) {
                        Ok(_) => break,
                        Err(Error::NotEnoughGP) => println!("\n** DUNGEON EXPRESS CARD - YOU LEFT HOME WITHOUT IT!"),
                        _ => (),
                    }
                },
                Some("M") => {
                    match self.game.player_purchase_weapon(WeaponType::Mace, true) {
                        Ok(_) => break,
                        Err(Error::NotEnoughGP) => println!("\n** SORRY SIR, I DON'T GIVE CREDIT"),
                        _ => (),
                    }
                },
                Some("D") => {
                    // If we get to this &point we already had enough to buy a dagger
                    let _ = self.game.player_purchase_weapon(WeaponType::Dagger, true);
                    break;
                },
                Some("N") => break,
                _ => {
                    println!("\n** TRY CHOOSING A SELECTION");
                },
            }
        }
    }

    /// Buy stats from a Vendor
    fn vendor_buy_stats(&mut self) {

        let stats = [Stat::Strength, Stat::Intelligence, Stat::Dexterity];

        let mut i = 0;

        while i < 3 {

            let s = &stats[i];

            if !self.game.vendor_can_afford_stat() {
                break;
            }

            let stat_name = UI::stat_name(*s);

            loop {
                let play_again = UI::get_input(Some(&format!("\nWANT TO BUY A POTION OF {} FOR 1000 GP's? ", stat_name)));

                match play_again.get(..1) {
                    Some("Y") => {
                        match self.game.vendor_buy_stat(*s) {
                            Ok(new_value) => {
                                println!("\nYOUR {} IS NOW {}", stat_name, new_value);
                            },
                            Err(err) => panic!("{:#?}", err),
                        }
                        break;
                    },
                    Some("N") => {
                        i += 1;
                        break;
                    },
                    _ => {
                        println!("\n** ANSWER YES OR NO");
                    },
                }
            }

        }

    }

    /// Trade with a Vendor
    fn vendor_trade(&mut self) {
        self.vendor_trade_treasures();

        if self.game.player_gp() < 1000 {
            println!("\n** YOU'RE TOO POOR TO TRADE");
            return;
        }

        self.vendor_trade_armor();
        self.vendor_trade_weapons();
        self.vendor_buy_stats();
    }

    /// Interact with a Vendor
    pub fn vendor(&mut self) -> bool {
        println!("YOU MAY TRADE WITH, ATTACK, OR IGNORE THE VENDOR");

        let mut fighting_vendor = false;

        loop {
            let choice = UI::get_input(Some("\nYOUR CHOICE? "));

            match choice.get(..1) {
                Some("T") => {
                    self.vendor_trade();
                    self.game.vendor_complete();
                    break;
                },
                Some("A") => {
                    println!("\nYOU'LL BE SORRY YOU DID THAT");
                    self.game.vendor_attack();
                    fighting_vendor = true;
                    break;
                },
                Some("I") => {
                    self.game.vendor_complete();
                    break;
                },
                _ => println!("\n** NICE SHOT, {}.", self.race_str()),
            }
        };

        fighting_vendor
    }

    /// Shine the lamp into another room
    pub fn lamp(&mut self) {

        let dir;

        let dir_str = UI::get_input(Some("WHERE DO YOU WANT TO SHINE THE LAMP (N,S,E, OR W)? "));

        match dir_str.get(..1) {
            Some("N") => dir = Direction::North,
            Some("S") => dir = Direction::South,
            Some("W") => dir = Direction::West,
            Some("E") => dir = Direction::East,
            _ => {
                println!("\n** TURKEY! THAT'S NOT A DIRECTION\n");
                return;
            },
        }

        let (x, y, z, room_type) = self.game.shine_lamp(dir);

        println!("\nTHE LAMP SHINES INTO ({},{}) LEVEL {}\n", x+1, y+1, z+1);

        let room_str = UI::room_name(&room_type);

        println!("THERE YOU'LL FIND {}\n", room_str);
    }

}

/// Main
fn main() {
    let mut playing = true;

    UI::intro();

    while playing {

        let game = Game::new(8, 8, 8);

        let mut ui = UI {
            game: game,
            rng: thread_rng(),
            turn_count: 0,
        };

        ui.race_gender_select();
        ui.allocate_points();
        ui.buy_armor();
        ui.buy_weapon();
        ui.buy_lamp();
        ui.buy_flares();

        println!("\nOK {}, YOU ENTER THE CASTLE AND BEGIN.\n", ui.race_str());

        let mut alive = true;

        let mut map_requested = false;

        while alive {
            ui.turn_count += 1;

            ui.game.discover_room_at_player();

            if map_requested {
                ui.print_location();
            } else {
                println!();

                if ui.game.state() != GameState::VendorAttack {
                    ui.print_location();
                    ui.print_stats();

                    ui.print_room();
                }
                
                let mut automove = false;

                match ui.game.room_effect() {
                    Event::FoundGold(_) => {
                        println!("YOU HAVE {}", ui.game.player_gp());
                    },
                    Event::FoundFlares(_) => {
                        println!("YOU HAVE {}", ui.game.player_flares());
                    },
                    Event::Sinkhole => {
                        automove = true;
                    },
                    Event::Warp => {
                        automove = true;
                    },
                    Event::Combat(monster_type) => {
                        let retreated = ui.combat(monster_type);

                        automove = retreated;
                    }
                    Event::Treasure(_) => {
                        println!("IT'S NOW YOURS\n");
                    }
                    Event::Vendor => {
                        ui.vendor();
                    }
                    Event::None => (),
                }

                // See if we were killed by something
                if ui.game.state() == GameState::Dead {
                    alive = false;
                    continue;
                }

                // If we're chosen to fight the vendor, let's do that
                if ui.game.state() == GameState::VendorAttack {
                    automove = true;
                }

                if automove {
                    continue;
                }
            }

            map_requested = false;

            // TODO curse effects (Does this happen before automove??)

            // TODO curse check

            // TODO random message

            // TODO cure blindness

            // TODO dissolve books

            let mut valid_command = false;

            while !valid_command {
                valid_command = true;

                let command = UI::get_input(Some("\nYOUR MOVE? "));

                println!();

                match command.get(..2) {
                    Some("DR") => {
                        ui.drink();
                        break;
                    }
                    _ => ()
                }

                match command.get(..1) {
                    Some("M") => {
                        ui.map(false);
                        map_requested = true;
                    },
                    Some("N") => ui.move_dir(Direction::North),
                    Some("S") => ui.move_dir(Direction::South),
                    Some("W") => ui.move_dir(Direction::West),
                    Some("E") => ui.move_dir(Direction::East),
                    Some("U") => ui.move_stairs(Stairs::Up),
                    Some("D") => ui.move_stairs(Stairs::Down),
                    Some("T") => ui.teleport(),
                    Some("L") => ui.lamp(),
                    _ => {
                        println!("** STUPID {} THAT WASN'T A VALID COMMAND\n", ui.race_str());
                        valid_command = false;
                    }
                }
            }

            // See if the player walked out
            if ui.game.state() == GameState::Exit {
                alive = false;
                continue;
            }

        } // while alive

        ui.game_summary();

        let mut valid_command = false;

        while !valid_command {
            let play_again = UI::get_input(Some("\nPLAY AGAIN? "));

            match play_again.get(..1) {
                Some("Y") => {
                    println!("\nSOME {}S NEVER LEARN\n\n", ui.race_str());
                    valid_command = true;
                },
                Some("N") => {
                    println!("\nMAYBE DUMB {} NOT SO DUMB AFTER ALL\n", ui.race_str());
                    playing = false;
                    valid_command = true;
                }
                _ => {
                    println!("\n** ANSWER YES OR NO");
                }
            }
        }

    } // while playing

}
