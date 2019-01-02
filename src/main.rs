extern crate wizardscastle;
extern crate rand; 

use std::io::{stdin,stdout,Write};

use self::rand::Rng;
use self::rand::thread_rng;

use wizardscastle::game::{Game,Direction,Event,CombatEvent,GameState};
use wizardscastle::room::RoomType;
use wizardscastle::player::{Race, Gender, Stat};
use wizardscastle::armor::ArmorType;
use wizardscastle::weapon::WeaponType;
use wizardscastle::treasure::TreasureType;
use wizardscastle::monster::MonsterType;

struct UI {
    game: Game,
    turn_count: usize,
}

impl UI {
    /// Return a random monster name
    fn rand_monster_str() -> String {
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

        let mut rng = thread_rng();

        let i = rng.gen_range(0, monster.len());

        UI::monster_name(monster[i])
    }

    fn monster_name(m:MonsterType) -> String {
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

    fn weapon_name(w:WeaponType) -> String {
        match w {
            WeaponType::None => String::from("NO WEAPON"),
            WeaponType::Dagger => String::from("DAGGER"),
            WeaponType::Mace => String::from("MACE"),
            WeaponType::Sword => String::from("SWORD"),
        }
    }

    fn armor_name(a:ArmorType) -> String {
        match a {
            ArmorType::None => String::from("NO ARMOR"),
            ArmorType::Leather => String::from("LEATHER"),
            ArmorType::Chainmail => String::from("CHAINMAIL"),
            ArmorType::Plate => String::from("PLATE"),
        }
    }

    fn treasure_name(t:&TreasureType) -> String {
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

    fn room_name(r:&RoomType) -> String {
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
                format!("THE {}", UI::treasure_name(t.treasure_type()))
            }
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
    fn move_dir(&mut self, dir:Direction) {
        self.game.move_dir(dir)
    }

    /// Print a map
    fn map(&mut self, show_all: bool) {
        let z = self.game.player.z;

        for y in 0..self.game.dungeon.ysize {
            for x in 0..self.game.dungeon.xsize {

                if x >= 1 {
                    print!("   ");
                }

                let r = self.game.dungeon.room_at(x, y, z);

                let bracket = x == self.game.player.x && y == self.game.player.y;

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
        match self.game.player.race {
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

        self.game.player.init(race);

        let gender = loop {
            let gender_str = UI::get_input(Some("\nWHICH SEX TO YOU PREFER? "));

            match gender_str.get(..1) {
                Some("M") => break Gender::Male,
                Some("F") => break Gender::Female,
                _ => println!("** CUTE {}, REAL CUTE. TRY M OR F.", self.race_str()),
            }
        };

        self.game.player.set_gender(gender);
    }

    /// Allocate additional stat points
    fn allocate_points(&mut self) {
        println!("\nOK {}, YOU HAVE THESE STATISTICS:\n", self.race_str());

        println!("STRENGTH= {} INTELLIGENCE= {} DEXTERITY= {}\n",
            self.game.player.st, self.game.player.iq, self.game.player.dx);

        println!("AND {} OTHER POINTS TO ALLOCATE AS YOU WISH.\n", self.game.player.additional_points);

        let stats = vec!(Stat::Intelligence, Stat::Strength, Stat::Dexterity);
        let stat_names = vec!("INTELLIGENCE", "STRENGTH", "DEXTERITY");

        for i in 0..3 {
            let mut ok = false;

            while !ok {
                let s = UI::get_input(Some(&format!("HOW MANY POINTS DO YOU ADD TO {}? ", stat_names[i])));

                let points_to_add;
                
                match s.parse::<usize>() {
                    Ok(p) => points_to_add = p,
                    Err(_) => {
                        print!("\n** ");
                        continue;
                    },
                };

                if let Ok(_) = self.game.player.allocate_points(&stats[i], points_to_add) {
                    ok = true;
                } else {
                    print!("\n** ");
                    continue;
                }
            }

            if self.game.player.additional_points == 0 {
                return;
            }
        }
    }

    /// Buy armor
    fn buy_armor(&mut self) {
        println!("\nOK, {}, YOU HAVE {} GOLD PIECES (GP's)\n", self.race_str(), self.game.player.gp);

        println!("HERE IS A LIST OF ARMOR YOU CAN BUY (WITH COST IN <>)\n");

        println!("PLATE<30> CHAINMAIL<20> LEATHER<10> NOTHING<0>");

        let _ = loop {
            let armor_str = UI::get_input(Some("\nYOUR CHOICE? "));

            match armor_str.get(..1) {

                Some("P") => break self.game.player.purchase_armor(ArmorType::Plate, false),
                Some("C") => break self.game.player.purchase_armor(ArmorType::Chainmail, false),
                Some("L") => break self.game.player.purchase_armor(ArmorType::Leather, false),
                Some("N") => break self.game.player.purchase_armor(ArmorType::None, false),
                _ => {
                    let mon_str = UI::rand_monster_str();
                    let article = UI::get_article(&mon_str);

                    println!("\n** ARE YOU A {} OR {} {}? TYPE P,C,L OR N", self.race_str(), article, mon_str);
                },
            }
        };
    }

    /// Buy weapon
    fn buy_weapon(&mut self) {

        println!("\nOK, BOLD {}, YOU HAVE {} GP's LEFT\n", self.race_str(), self.game.player.gp);

        println!("HERE IS A LIST OF WEAPONS YOU CAN BUY (WITH COST IN <>)\n");

        println!("SWORD<30> MACE<20> DAGGER<10> NOTHING<0>");

        let _ = loop {
            let armor_str = UI::get_input(Some("\nYOUR CHOICE? "));

            match armor_str.get(..1) {

                Some("S") => break self.game.player.purchase_weapon(WeaponType::Sword, false),
                Some("M") => break self.game.player.purchase_weapon(WeaponType::Mace, false),
                Some("D") => break self.game.player.purchase_weapon(WeaponType::Dagger, false),
                Some("N") => break self.game.player.purchase_weapon(WeaponType::None, false),
                _ => println!("\n** IS YOUR IQ REALLY {}? TYPE S, M, D, OR N", self.game.player.iq),
            }
        };
    }

    /// Buy lamp
    fn buy_lamp(&mut self) {
        if !self.game.player.can_purchase_lamp() {
            return;
        }

        let _ = loop {
            let lamp_str = UI::get_input(Some("\nWANT TO BUY A LAMP FOR 20 GP's? "));

            match lamp_str.get(..1) {
                Some("Y") => break self.game.player.purchase_lamp(true),
                Some("N") => break self.game.player.purchase_lamp(false),
                _ => println!("\n** ANSWER YES OR NO"),
            }
        };
    }

    /// Buy flares
    fn buy_flares(&mut self) {
        let max_flares = self.game.player.max_flares();

        if max_flares == 0 {
            return;
        }

        println!("\nOK, {}, YOU HAVE {} GOLD PIECES LEFT\n", self.race_str(), self.game.player.gp);

        loop {
            let flare_str = UI::get_input(Some("FLARES COST 1 GP EACH, HOW MANY DO YOU WANT? "));

            let flare_count;
            
            match flare_str.parse::<usize>() {
                Ok(f) => flare_count = f,
                Err(_) => {
                    print!("** IF YOU DON'T WANT ANY JUST TYPE 0 (ZERO)\n\n");
                    continue;
                },
            };

            match self.game.player.purchase_flares(flare_count) {
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
        let p = &self.game.player;

        if p.is_blind() {
            return;
        }

        println!("YOU ARE AT ({},{}) LEVEL {}", p.x + 1, p.y + 1, p.z + 1);
    }

    /// Print player stats
    fn print_stats(&self) {
        let p = &self.game.player;

        println!("ST={} IQ={} DX={} FLARES={} GP's={}",
            p.stat(Stat::Strength),
            p.stat(Stat::Intelligence),
            p.stat(Stat::Dexterity),
            p.flares(),
            p.gp());

        let w_name = UI::weapon_name(p.weapon().weapon_type());
        let a_name = UI::armor_name(p.armor().armor_type());

        print!( "{} / {}", w_name, a_name);

        if p.has_lamp() {
            print!(" / A LAMP");
        }

        println!("\n");
    }

    /// Print the current room
    fn print_room(&mut self) {
        let p = &self.game.player;

        let room = self.game.dungeon.room_at(p.x, p.y, p.z);

        let room_str = UI::room_name(&room.roomtype);

        println!("HERE YOU FIND {}\n", room_str);
    }

    // Attack a monster
    fn combat_attack(&mut self, m_art:&str, m_name:&str) {
        // Need to do this before the attack since the weapon might
        // break during it
        let weapon_type = self.game.player.weapon.weapon_type();

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
                    println!("\n** ANSWER YES OR NO");
                }
            }
        }

        self.game.retreat_dir(dir);
    }

    /// Handle combat
    fn combat(&mut self, monster_type:MonsterType) -> bool {

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
                        self.game.player.st, self.game.player.dx);

                    let err_str = "\n** CHOOSE ONE OF THE OPTIONS LISTED.";

                    match UI::get_input(Some("YOUR CHOICE? ")).get(..1) {
                        Some("A") => self.combat_attack(&m_art, &m_name),
                        Some("R") => self.combat_retreat(),
                        Some("B") => {
                            if can_bribe {
                                // TODO
                            } else {
                                println!("{}", err_str)
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
                if self.game.player.st == 0 {
                    println!("STRENGTH");
                }
                else if self.game.player.iq == 0 {
                    println!("INTELLIGENCE");
                }
                else if self.game.player.dx == 0 {
                    println!("DEXTERITY");
                }

                println!("\nWHEN YOU DIED YOU HAD:\n");
            },

            GameState::Exit => {
                let win = self.game.player.has_orb_of_zot();

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
        for t in self.game.player.get_treasures() {
            println!("{}", UI::treasure_name(t));
        }

        // Show weapon
        println!("{}", UI::weapon_name(self.game.player.weapon().weapon_type()));

        // Show armor
        println!("{}", UI::armor_name(self.game.player.armor().armor_type()));

        // Show lamp
        if self.game.player.has_lamp() {
            println!("A LAMP");
        }

        // Show flares
        println!("{} FLARES", self.game.player.flares());

        // Show GPs
        println!("{} GP's", self.game.player.gp());

        // Show Runestaff
        if self.game.player.has_runestaff() {
            println!("THE RUNESTAFF");
        }

        // Show turns
        println!("\nAND IT TOOK YOU {} TURNS!\n", self.turn_count);
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

        while alive {
            ui.turn_count += 1;

            ui.game.dungeon.discover(ui.game.player.x, ui.game.player.y, ui.game.player.z);

            println!();

            ui.print_location();
            ui.print_stats();

            ui.print_room();
            
            let mut automove = false;

            match ui.game.room_effect() {
                Event::FoundGold(_) => {
                    println!("YOU HAVE {}", ui.game.player.gp);
                },
                Event::FoundFlares(_) => {
                    println!("YOU HAVE {}", ui.game.player.flares);
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
                    println!("[STUB: Vendor trade]");
                }
                Event::None => (),
            }

            // See if we were killed by something
            if ui.game.state() == GameState::Dead {
                alive = false;
                continue;
            }

            if automove {
                continue;
            }

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

                match command.get(..1) {
                    Some("M") => ui.map(true),
                    Some("N") => ui.move_dir(Direction::North),
                    Some("S") => ui.move_dir(Direction::South),
                    Some("W") => ui.move_dir(Direction::West),
                    Some("E") => ui.move_dir(Direction::East),
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
