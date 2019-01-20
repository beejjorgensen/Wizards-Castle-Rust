use std::io::{stdin, stdout, Write};

use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;

use wizardscastle::armor::{Armor, ArmorType};
use wizardscastle::error::Error;
use wizardscastle::game::{
    BookEvent, ChestEvent, DrinkEvent, GameState, HitResult, OrbEvent, RandomMessage,
};
use wizardscastle::game::{CombatEvent, Direction, Event, Game, Stairs};
use wizardscastle::monster::MonsterType;
use wizardscastle::player::{Gender, Race, Stat};
use wizardscastle::room::RoomType;
use wizardscastle::treasure::TreasureType;
use wizardscastle::weapon::{Weapon, WeaponType};

struct UI {
    game: Game,
    rng: ThreadRng,
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

    fn treasure_name(t: TreasureType) -> String {
        match t {
            TreasureType::RubyRed => String::from("THE RUBY RED"),
            TreasureType::NornStone => String::from("THE NORN STONE"),
            TreasureType::PalePearl => String::from("THE PALE PEARL"),
            TreasureType::OpalEye => String::from("THE OPAL EYE"),
            TreasureType::GreenGem => String::from("THE GREEN GEM"),
            TreasureType::BlueFlame => String::from("THE BLUE FLAME"),
            TreasureType::Palantir => String::from("THE PALANTIR"),
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
            RoomType::Treasure(t) => UI::treasure_name(*t.treasure_type()).to_string(),
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

        false
    }

    fn get_article(s: &str) -> String {
        if UI::starts_with_vowel(s) {
            return String::from("AN");
        }

        String::from("A")
    }

    /// Move a direction
    fn move_dir(&mut self, dir: Direction) {
        self.game.move_dir(dir);

        // This is often redundant, but there's a case where we retreat from
        // monsters and the discover room gets overlooked
        self.game.discover_room_at_player();
    }

    /// Take some stairs
    fn move_stairs(&mut self, stairs: Stairs) -> bool {
        if self.game.move_stairs(stairs).is_err() {
            println!(
                "** OH {}, NO STAIRS GOING {} IN HERE",
                self.race_str(),
                UI::stair_name(stairs)
            );
            return false;
        }

        true
    }

    // Input a coordinate, 1-8
    fn input_coord(prompt: &str) -> u32 {
        let mut coord = 0;
        let mut got_num = false;

        while !got_num {
            let str = UI::get_input(Some(prompt));

            if let Ok(v) = str.parse::<u32>() {
                if v >= 1 && v <= 8 {
                    got_num = true;
                    coord = v;
                }
            }

            if !got_num {
                println!("\n** TRY A NUMBER FROM 1 TO 8\n");
            }
        }

        coord
    }

    /// Teleport
    fn teleport(&mut self) -> bool {
        if !self.game.can_teleport() {
            println!("** YOU CAN'T TELEPORT WITHOUT THE RUNESTAFF!");
            return false;
        }

        let x = UI::input_coord("X-COORD (1 = FAR WEST  8 = FAR EAST )? ");
        let y = UI::input_coord("Y-COORD (1 = FAR NORTH 8 = FAR SOUTH)? ");
        let z = UI::input_coord("Z-COORD (1 = TOP       8 = BOTTOM   )? ");

        match self.game.teleport(x - 1, y - 1, z - 1) {
            // back to 0-based
            Ok(found_orb_of_zot) => {
                if found_orb_of_zot {
                    println!("\nGREAT UNMITIGATED ZOT!\n");
                    println!("YOU JUST FOUND THE ORB OF ZOT!\n");
                    println!("THE RUNESTAFF IS GONE\n");
                }
            }
            Err(err) => panic!("{:#?}", err),
        }

        true
    }

    /// Drink
    fn drink(&mut self) {
        let s;

        match self.game.drink() {
            Ok(DrinkEvent::Stronger) => {
                s = String::from("FEEL STRONGER");
            }
            Ok(DrinkEvent::Weaker) => {
                s = String::from("FEEL WEAKER");
            }
            Ok(DrinkEvent::Smarter) => {
                s = String::from("FEEL SMARTER");
            }
            Ok(DrinkEvent::Dumber) => {
                s = String::from("FEEL DUMBER");
            }
            Ok(DrinkEvent::Nimbler) => {
                s = String::from("FEEL NIMBLER");
            }
            Ok(DrinkEvent::Clumsier) => {
                s = String::from("FEEL CLUMSIER");
            }
            Ok(DrinkEvent::ChangeRace) => {
                s = format!("TURN INTO A {}", self.race_str());
            }
            Ok(DrinkEvent::ChangeGender) => {
                s = format!(
                    "TURN INTO A {} {}",
                    UI::gender_name(*self.game.player_gender()),
                    self.race_str()
                );
            }
            Err(Error::CantGo) => {
                println!("** IF YOU WANT A DRINK, FIND A POOL");
                return;
            }
            Err(err) => panic!("{:#?}", err),
        }

        println!("YOU TAKE A DRINK AND {}", s);
    }

    /// Get the printable character for a room
    fn room_char(room_type: &RoomType) -> char {
        match room_type {
            RoomType::Empty => '.',
            RoomType::Entrance => 'E',
            RoomType::StairsDown => 'D',
            RoomType::StairsUp => 'U',
            RoomType::Gold => 'G',
            RoomType::Pool => 'P',
            RoomType::Chest => 'C',
            RoomType::Flares => 'F',
            RoomType::Warp(_) => 'W',
            RoomType::Sinkhole => 'S',
            RoomType::CrystalOrb => 'O',
            RoomType::Book => 'B',
            RoomType::Monster(ref m) => {
                if m.monster_type() == MonsterType::Vendor {
                    'V'
                } else {
                    'M'
                }
            }
            RoomType::Treasure(_) => 'T',
        }
    }

    /// Print a map
    fn map(&mut self, show_all: bool) {
        if self.game.player_is_blind() {
            println!("** YOU CAN'T SEE ANYTHING, DUMB {}", self.race_str());
            return;
        }

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
                    print!("{}", UI::room_char(&r.roomtype));
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

        println!(
            "{:^64}",
            "MANY CYCLES AGO, IN THE KINGDOM OF N'DIC, THE GNOMIC"
        );
        println!(
            "{:^64}",
            "WIZARD ZOT FORGED HIS GREAT *ORB OF POWER*. HE SOON"
        );
        println!(
            "{:^64}",
            "VANISHED, LEAVING BEHIND HIS VAST SUBTERRANEAN CASTLE"
        );
        println!(
            "{:^64}",
            "FILLED WITH ESURIENT MONSTERS, FABULOUS TREASURES, AND"
        );
        println!(
            "{:^64}",
            "THE INCREDIBLE *ORB OF ZOT*. FROM THAT TIME HENCE, MANY"
        );
        println!(
            "{:^64}",
            "A BOLD YOUTH HAS VENTURED INTO THE WIZARD'S CASTLE. AS"
        );
        println!(
            "{:^64}",
            "OF NOW, *NONE* HAS EVER EMERGED VICTORIOUSLY! BEWARE!!"
        );

        println!("\n{:*^64}\n", "");
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

        println!(
            "STRENGTH= {} INTELLIGENCE= {} DEXTERITY= {}\n",
            self.game.player_stat(Stat::Strength),
            self.game.player_stat(Stat::Intelligence),
            self.game.player_stat(Stat::Dexterity)
        );

        println!(
            "AND {} OTHER POINTS TO ALLOCATE AS YOU WISH.\n",
            self.game.player_additional_points()
        );

        let stats = vec![Stat::Intelligence, Stat::Strength, Stat::Dexterity];
        let stat_names = vec!["INTELLIGENCE", "STRENGTH", "DEXTERITY"];

        for i in 0..3 {
            let mut ok = false;

            while !ok {
                let s = UI::get_input(Some(&format!(
                    "HOW MANY POINTS DO YOU ADD TO {}? ",
                    stat_names[i]
                )));

                let points_to_add;

                match s.parse::<u32>() {
                    Ok(p) => points_to_add = p,
                    Err(_) => {
                        print!("\n** ");
                        continue;
                    }
                };

                if self
                    .game
                    .player_allocate_points(stats[i], points_to_add)
                    .is_ok()
                {
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
        println!(
            "\nOK, {}, YOU HAVE {} GOLD PIECES (GP's)\n",
            self.race_str(),
            self.game.player_gp()
        );

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

                    println!(
                        "\n** ARE YOU A {} OR {} {}? TYPE P,C,L OR N",
                        self.race_str(),
                        article,
                        mon_str
                    );
                }
            }
        };
    }

    /// Buy weapon
    fn buy_weapon(&mut self) {
        println!(
            "\nOK, BOLD {}, YOU HAVE {} GP's LEFT\n",
            self.race_str(),
            self.game.player_gp()
        );

        println!("HERE IS A LIST OF WEAPONS YOU CAN BUY (WITH COST IN <>)\n");

        println!("SWORD<30> MACE<20> DAGGER<10> NOTHING<0>");

        let _ = loop {
            let armor_str = UI::get_input(Some("\nYOUR CHOICE? "));

            match armor_str.get(..1) {
                Some("S") => break self.game.player_purchase_weapon(WeaponType::Sword, false),
                Some("M") => break self.game.player_purchase_weapon(WeaponType::Mace, false),
                Some("D") => break self.game.player_purchase_weapon(WeaponType::Dagger, false),
                Some("N") => break self.game.player_purchase_weapon(WeaponType::None, false),
                _ => println!(
                    "\n** IS YOUR IQ REALLY {}? TYPE S, M, D, OR N",
                    self.game.player_stat(Stat::Intelligence)
                ),
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

        println!(
            "\nOK, {}, YOU HAVE {} GOLD PIECES LEFT\n",
            self.race_str(),
            self.game.player_gp()
        );

        loop {
            let flare_str = UI::get_input(Some("FLARES COST 1 GP EACH, HOW MANY DO YOU WANT? "));

            let flare_count;

            match flare_str.parse::<u32>() {
                Ok(f) => flare_count = f,
                Err(_) => {
                    print!("** IF YOU DON'T WANT ANY JUST TYPE 0 (ZERO)\n\n");
                    continue;
                }
            };

            match self.game.player_purchase_flares(flare_count) {
                Ok(_) => break,
                Err(_) => {
                    print!("** YOU CAN ONLY AFFORD {}\n\n", max_flares);
                    continue;
                }
            }
        }
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

        println!(
            "YOU ARE AT ({},{}) LEVEL {}",
            self.game.player_x() + 1,
            self.game.player_y() + 1,
            self.game.player_z() + 1
        );
    }

    /// Print player stats
    fn print_stats(&self) {
        println!(
            "ST={} IQ={} DX={} FLARES={} GP's={}",
            self.game.player_stat(Stat::Strength),
            self.game.player_stat(Stat::Intelligence),
            self.game.player_stat(Stat::Dexterity),
            self.game.player_flares(),
            self.game.player_gp()
        );

        let w_name = UI::weapon_name(self.game.player_weapon_type());
        let a_name = UI::armor_name(self.game.player_armor_type());

        print!("{} / {}", w_name, a_name);

        if self.game.player_has_lamp() {
            print!(" / A LAMP");
        }

        println!("\n");
    }

    /// Print the current room
    fn print_room(&mut self) {
        let room = self.game.room_at_player();

        let room_str = UI::room_name(&room.roomtype);

        println!("HERE YOU FIND {}", room_str);
    }

    /// Print messaging when monster defeated by melee or magic
    fn monster_defeated_message(&mut self, result: HitResult, m_art: &str, m_name: &str) {
        if result.defeated {
            println!("\n{} {} LIES DEAD AT YOUR FEET", m_art, m_name);

            if self.game.rand_recipe() {
                let suffix = [
                    "WICH", " STEW", " SOUP", " BURGER", " ROAST", " MUNCHY", " TACO", " PIE",
                ];

                let i = self.rng.gen_range(0, suffix.len());

                println!("\nYOU SPEND AN HOUR EATING {}{}", m_name, suffix[i]);
            }

            if result.killed_vendor {
                println!("\nYOU GET ALL HIS WARES\n");
                println!("PLATE ARMOR");
                println!("A SWORD");
                println!("A STRENGTH POTION");
                println!("AN INTELLIGENCE POTION");
                println!("A DEXTERITY POTION");

                if result.got_lamp {
                    println!("A LAMP");
                }
            } else {
                if result.got_runestaff {
                    println!("\nGREAT ZOT! YOU'VE FOUND THE RUNESTAFF");
                }

                println!("\nYOU NOW GET HIS HOARD OF {} GP's", result.treasure);
            }
        }
    }

    // Attack a monster
    fn combat_attack(&mut self, m_art: &str, m_name: &str) {
        // Need to do this before the attack since the weapon might
        // break during it
        let weapon_type = self.game.player_weapon_type();

        match self.game.attack() {
            Ok(CombatEvent::NoWeapon) => {
                println!("\n** POUNDING ON {} {} WON'T HURT IT", m_art, m_name);
            }

            Ok(CombatEvent::BookHands) => {
                println!("\n** YOU CAN'T BEAT IT TO DEATH WITH A BOOK");
            }

            Ok(CombatEvent::Hit(result)) => {
                println!("\n  YOU HIT THE LOUSY {}", m_name);

                if result.broke_weapon {
                    println!("\nOH NO! YOUR {} BROKE", UI::weapon_name(weapon_type));
                }

                self.monster_defeated_message(result, m_art, m_name);
            }

            Ok(CombatEvent::Miss) => {
                println!("\n  DRAT! MISSED");
            }

            Ok(any) => panic!("unexpected combat event {:#?}", any),

            Err(err) => panic!("error in combat {:#?}", err),
        }
    }

    /// Be attacked by a monster
    fn combat_be_attacked(&mut self, m_name: &str) {
        match self.game.be_attacked() {
            Ok(CombatEvent::MonsterWebbed) => {
                println!("\nTHE {} IS STUCK AND CAN'T ATTACK", m_name);
            }

            Ok(CombatEvent::MonsterHit(_damage, _defeated, armor_destroyed, web_broke)) => {
                if web_broke {
                    println!("\nTHE WEB JUST BROKE!");
                }

                println!("\nTHE {} ATTACKS", m_name);

                println!("\n  OUCH! HE HIT YOU");

                if armor_destroyed {
                    println!("\nYOUR ARMOR IS DESTROYED - GOOD LUCK\n");
                }
            }

            Ok(CombatEvent::MonsterMiss) => {
                println!("\nTHE {} ATTACKS", m_name);

                println!("\n  HAH! HE MISSED YOU");
            }

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
            Ok(Some(t_type)) => loop {
                let tname = UI::treasure_name(t_type);

                let yn = UI::get_input(Some(&format!(
                    "\nI WANT {}, WILL YOU GIVE IT TO ME? ",
                    tname
                )));

                match yn.get(..1) {
                    Some("Y") => {
                        match self.game.bribe_accept() {
                            Ok(_) => {
                                bribed = true;
                            }
                            Err(err) => {
                                panic!("agree to bribe: {:#?}", err);
                            }
                        };
                        break;
                    }
                    Some("N") => {
                        match self.game.bribe_decline() {
                            Ok(_) => {
                                bribed = false;
                            }
                            Err(err) => {
                                panic!("disagree to bribe: {:#?}", err);
                            }
                        };
                        break;
                    }
                    _ => println!("\n** ANSWER YES OR NO"),
                }
            },
            Ok(None) => {
                println!("\n'ALL I WANT IS YOUR LIFE!'");
            }
            Err(err) => {
                panic!("bribe proposition: {:#?}", err);
            }
        };

        bribed
    }

    /// Handle combat spells
    fn combat_spell(&mut self, m_art: &str, m_name: &str) {
        match UI::get_input(Some("\nWHICH SPELL (WEB, FIREBALL, OR DEATHSPELL)? ")).get(..1) {
            Some("W") => match self.game.spell_web() {
                Ok(CombatEvent::Hit(_)) => (),
                Ok(CombatEvent::Died) => (),
                Ok(any) => panic!("Unexpected: {:#?}", any),
                Err(err) => panic!("{:#?}", err),
            },
            Some("F") => match self.game.spell_fireball() {
                Ok(CombatEvent::Hit(hr)) => {
                    println!("\n  IT DOES {} POINTS OF DAMAGE.\n", hr.damage);
                    self.monster_defeated_message(hr, m_art, m_name);
                }
                Ok(CombatEvent::Died) => (),
                Ok(any) => panic!("Unexpected: {:#?}", any),
                Err(err) => panic!("{:#?}", err),
            },
            Some("D") => {
                print!("\nDEATH - - - ");
                match self.game.spell_deathspell() {
                    Ok(CombatEvent::Hit(hr)) => {
                        println!("HIS\n");
                        self.monster_defeated_message(hr, m_art, m_name);
                    }
                    Ok(CombatEvent::Died) => {
                        println!("YOURS\n");
                    }
                    Ok(any) => panic!("Unexpected: {:#?}", any),
                    Err(err) => panic!("{:#?}", err),
                }
            }
            _ => println!("\n** TRY ONE OF THE OPTIONS GIVEN"),
        }
    }

    /// Handle combat
    fn combat(&mut self, monster_type: MonsterType) -> bool {
        let m_name = UI::monster_name(monster_type);
        let m_art = UI::get_article(&m_name);

        let mut in_combat = true;
        let mut retreated = false;

        while in_combat {
            match self.game.state() {
                GameState::PlayerAttack => {
                    println!("\nYOU'RE FACING {} {}!", m_art, m_name);

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

                    println!(
                        "\nYOUR STRENGTH IS {} AND DEXTERITY IS {}.\n",
                        self.game.player_stat(Stat::Strength),
                        self.game.player_stat(Stat::Dexterity)
                    );

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
                                self.combat_spell(&m_art, &m_name);
                            } else {
                                println!("\n** YOU CAN'T CAST A SPELL NOW");
                            }
                        }
                        _ => println!("\n** CHOOSE ONE OF THE OPTIONS LISTED."),
                    }
                }

                GameState::MonsterAttack => {
                    self.combat_be_attacked(&m_name);
                }

                GameState::Retreat => {
                    self.combat_retreat_dir();
                    retreated = true;
                }

                GameState::Move => {
                    in_combat = false;
                }

                GameState::Dead => {
                    in_combat = false;
                }

                any => panic!("unknown state during combat {:#?}", any),
            }
        } // while in_combat

        retreated
    }

    /// Print out the game over summary
    pub fn game_summary(&self) {
        match self.game.state() {
            GameState::Dead => {
                println!("\n{:*^64}\n", "");

                println!("A NOBLE EFFORT, OH FORMERLY LIVING {}\n", self.race_str());

                print!("YOU DIED FROM A LACK OF ");
                if self.game.player_stat(Stat::Strength) == 0 {
                    println!("STRENGTH");
                } else if self.game.player_stat(Stat::Intelligence) == 0 {
                    println!("INTELLIGENCE");
                } else if self.game.player_stat(Stat::Dexterity) == 0 {
                    println!("DEXTERITY");
                }

                println!("\nWHEN YOU DIED YOU HAD:\n");
            }

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
            }

            GameState::Quit => {
                println!();
                println!("A LESS THAN AWE-INSPIRING DEFEAT.\n");
                println!("WHEN YOU LEFT THE CASTLE YOU HAD:\n");

                println!("YOUR MISERABLE LIFE");
            }

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
        println!("\nAND IT TOOK YOU {} TURNS!\n", *self.game.turn());
    }

    /// Ask the user if they want to play again
    fn play_again(&self) -> bool {
        loop {
            let play_again = UI::get_input(Some("\nPLAY AGAIN? "));

            match play_again.get(..1) {
                Some("Y") => {
                    println!("\nSOME {}S NEVER LEARN\n\n", self.race_str());
                    break true;
                }
                Some("N") => {
                    println!("\nMAYBE DUMB {} NOT SO DUMB AFTER ALL\n", self.race_str());
                    break false;
                }
                _ => {
                    println!("\n** ANSWER YES OR NO");
                }
            }
        }
    }

    /// Sell treasures to a vendor
    fn vendor_trade_treasures(&mut self) {
        let treasures = self.game.player_get_treasures().clone();

        if treasures.is_empty() {
            return;
        }

        println!();

        for t in treasures {
            let price = match self.game.vendor_treasure_offer(t) {
                Ok(p) => p,
                Err(err) => panic!("vendor treasure offer: {:#?}", err),
            };

            loop {
                let yn = UI::get_input(Some(&format!(
                    "DO YOU WANT TO SELL {} FOR {} GP's? ",
                    UI::treasure_name(t),
                    price
                )));

                match yn.get(..1) {
                    Some("Y") => {
                        match self.game.vendor_treasure_accept() {
                            Ok(_) => (),
                            Err(err) => panic!("vendor treasure accept: {:#?}", err),
                        }
                        break;
                    }
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

        println!(
            "\nOK, {}, YOU HAVE {} GOLD PIECES AND {}",
            self.race_str(),
            self.game.player_gp(),
            UI::armor_name(self.game.player_armor_type())
        );

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
                Some("P") => match self.game.player_purchase_armor(ArmorType::Plate, true) {
                    Ok(_) => break,
                    Err(Error::NotEnoughGP) => println!("\n** YOU CAN'T AFFORD PLATE"),
                    _ => (),
                },
                Some("C") => match self.game.player_purchase_armor(ArmorType::Chainmail, true) {
                    Ok(_) => break,
                    Err(Error::NotEnoughGP) => println!("\n** YOU HAVEN'T GOT THAT MUCH CASH"),
                    _ => (),
                },
                Some("L") => {
                    // If we get to this point we already had enough to buy leather
                    let _ = self.game.player_purchase_armor(ArmorType::Leather, true);
                    break;
                }
                Some("N") => break,
                _ => {
                    println!("\n** DON'T BE SILLY. CHOOSE A SELECTION");
                }
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

        println!(
            "\nYOU HAVE {} GP's LEFT WITH {} IN HAND",
            self.game.player_gp(),
            UI::weapon_name(self.game.player_weapon_type())
        );

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
                Some("S") => match self.game.player_purchase_weapon(WeaponType::Sword, true) {
                    Ok(_) => break,
                    Err(Error::NotEnoughGP) => {
                        println!("\n** DUNGEON EXPRESS CARD - YOU LEFT HOME WITHOUT IT!")
                    }
                    _ => (),
                },
                Some("M") => match self.game.player_purchase_weapon(WeaponType::Mace, true) {
                    Ok(_) => break,
                    Err(Error::NotEnoughGP) => println!("\n** SORRY SIR, I DON'T GIVE CREDIT"),
                    _ => (),
                },
                Some("D") => {
                    // If we get to this &point we already had enough to buy a dagger
                    let _ = self.game.player_purchase_weapon(WeaponType::Dagger, true);
                    break;
                }
                Some("N") => break,
                _ => {
                    println!("\n** TRY CHOOSING A SELECTION");
                }
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
                let play_again = UI::get_input(Some(&format!(
                    "\nWANT TO BUY A POTION OF {} FOR 1000 GP's? ",
                    stat_name
                )));

                match play_again.get(..1) {
                    Some("Y") => {
                        match self.game.vendor_buy_stat(*s) {
                            Ok(new_value) => {
                                println!("\nYOUR {} IS NOW {}", stat_name, new_value);
                            }
                            Err(err) => panic!("{:#?}", err),
                        }
                        break;
                    }
                    Some("N") => {
                        i += 1;
                        break;
                    }
                    _ => {
                        println!("\n** ANSWER YES OR NO");
                    }
                }
            }
        }
    }

    /// Buy a lamp from the vendor
    fn vendor_buy_lamp(&mut self) {
        if self.game.player_has_lamp() || !self.game.vendor_can_afford_lamp() {
            return;
        }

        loop {
            let lamp = UI::get_input(Some("\nWANT A LAMP FOR OR 1000 GP's? "));

            match lamp.get(..1) {
                Some("Y") => {
                    match self.game.vendor_buy_lamp() {
                        Ok(()) => println!("\nIT'S GUARANTEED TO OUTLIVE YOU!"),
                        Err(err) => panic!("{:#?}", err),
                    }
                    break;
                }
                Some("N") => {
                    break;
                }
                _ => {
                    println!("\n** ANSWER YES OR NO");
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
        self.vendor_buy_lamp();
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
                }
                Some("A") => {
                    println!("\nYOU'LL BE SORRY YOU DID THAT");
                    self.game.vendor_attack();
                    fighting_vendor = true;
                    break;
                }
                Some("I") => {
                    self.game.vendor_complete();
                    break;
                }
                _ => println!("\n** NICE SHOT, {}.", self.race_str()),
            }
        }

        fighting_vendor
    }

    /// Shine the lamp into another room
    pub fn lamp(&mut self) -> bool {
        if self.game.player_is_blind() {
            println!("** YOU CAN'T SEE ANYTHING, DUMB {}", self.race_str());
            return false;
        }

        if !self.game.player_has_lamp() {
            println!("** YOU DON'T HAVE A LAMP");
            return false;
        }

        let dir;

        let dir_str = UI::get_input(Some("WHERE DO YOU WANT TO SHINE THE LAMP (N,S,E, OR W)? "));

        match dir_str.get(..1) {
            Some("N") => dir = Direction::North,
            Some("S") => dir = Direction::South,
            Some("W") => dir = Direction::West,
            Some("E") => dir = Direction::East,
            _ => {
                println!("\n** TURKEY! THAT'S NOT A DIRECTION");
                return false;
            }
        }

        let (x, y, z, room_type);

        match self.game.shine_lamp(dir) {
            Ok((tx, ty, tz, troom_type)) => {
                x = tx;
                y = ty;
                z = tz;
                room_type = troom_type;
            }
            Err(err) => panic!(err),
        }

        println!(
            "\nTHE LAMP SHINES INTO ({},{}) LEVEL {}\n",
            x + 1,
            y + 1,
            z + 1
        );

        let room_str = UI::room_name(&room_type);

        println!("THERE YOU'LL FIND {}", room_str);

        true
    }

    /// Set off a flare
    pub fn flare(&mut self) -> bool {
        if self.game.player_is_blind() {
            println!("** YOU CAN'T SEE ANYTHING, DUMB {}", self.race_str());
            return false;
        }

        if self.game.player_flares() == 0 {
            println!("** HEY BRIGHT ONE, YOU'RE OUT OF FLARES");
            return false;
        }

        if let Err(err) = self.game.flare() {
            panic!(err);
        }

        let xm1 = self.game.player_x() as i32 - 1;
        let ym1 = self.game.player_y() as i32 - 1;

        let z = self.game.player_z();

        for y in ym1..(ym1 + 3) {
            let yw = self.game.wrap_y(y);

            for x in xm1..(xm1 + 3) {
                let xw = self.game.wrap_x(x);

                let room_type = self.game.dungeon_room_at(xw, yw, z).room_type();

                if x == xm1 {
                    print!(" ");
                }

                print!("{}", UI::room_char(&room_type));

                if x == xm1 || x == xm1 + 1 {
                    print!("     ");
                }
            }

            println!("\n");
        }

        true
    }

    /// Gaze into an Orb
    pub fn gaze(&mut self) -> bool {
        let mut success = false;

        match self.game.gaze() {
            Ok(event) => {
                print!("YOU SEE ");

                match event {
                    OrbEvent::BloodyHeap => println!("YOURSELF IN A BLOODY HEAP"),
                    OrbEvent::Polymorph(m) => {
                        let mon_str = UI::monster_name(m);
                        println!(
                            "YOURSELF DRINKING FROM A POOL AND BECOMING {} {}",
                            UI::get_article(&mon_str),
                            mon_str
                        );
                    }
                    OrbEvent::GazeBack(m) => {
                        let mon_str = UI::monster_name(m);
                        println!(
                            "{} {} GAZING BACK AT YOU",
                            UI::get_article(&mon_str),
                            mon_str
                        );
                    }
                    OrbEvent::Item(room_type, x, y, z) => {
                        println!(
                            "{} AT ({},{}) LEVEL {}",
                            UI::room_name(&room_type),
                            x + 1,
                            y + 1,
                            z + 1
                        );
                    }
                    OrbEvent::OrbOfZot(x, y, z) => {
                        println!("THE ORB OF ZOT AT ({},{}) LEVEL {}", x + 1, y + 1, z + 1);
                    }
                    OrbEvent::SoapOpera => {
                        println!("A SOAP OPERA RERUN");
                    }
                }

                println!();

                success = true;
            }
            Err(Error::Blind) => println!("** YOU CAN'T SEE ANYTHING, DUMB {}", self.race_str()),
            Err(Error::CantGo) => println!("** NO ORB - NO GAZE"),
            _ => panic!("SNH"),
        }

        success
    }

    /// Open a chest
    fn open_chest(&mut self) {
        match self.game.open_chest() {
            Ok(event) => match event {
                ChestEvent::Explode => println!("KABOOM! IT EXPLODES"),
                ChestEvent::Gas => println!("GAS! YOU STAGGER FROM THE ROOM"),
                ChestEvent::Treasure(amount) => println!("YOU FIND {} GOLD PIECES", amount),
            },

            Err(err) => panic!(err),
        }

        println!();
    }

    /// Open a book
    fn open_book(&mut self) {
        match self.game.open_book() {
            Ok(event) => match event {
                BookEvent::Blind => {
                    println!("FLASH! OH NO! YOU ARE NOW A BLIND {}", self.race_str())
                }
                BookEvent::Poetry => println!("IT'S ANOTHER VOLUME OF ZOT'S POETRY! - YEECH!"),
                BookEvent::PlayMonster(m) => {
                    println!("IT'S AN OLD COPY OF PLAY{}", UI::monster_name(m))
                }
                BookEvent::Dexterity => println!("IT'S A MANUAL OF DEXTERITY!"),
                BookEvent::Strength => println!("IT'S A MANUAL OF STRENGTH!"),
                BookEvent::Sticky => {
                    println!("THE BOOK STICKS TO YOUR HANDS -\n\nNOW YOU CAN'T DRAW YOUR WEAPON!")
                }
            },
            Err(err) => panic!(err),
        }

        println!();
    }

    /// Open a book or chest
    pub fn open(&mut self) -> bool {
        let room_type = self.game.room_at_player().room_type().clone();

        match room_type {
            RoomType::Chest => self.open_chest(),
            RoomType::Book => self.open_book(),
            _ => {
                println!("** THE ONLY THING YOU OPENED WAS YOUR BIG MOUTH");
                return false;
            }
        }

        true
    }

    /// Display a random message
    fn rand_message(&mut self) {
        match self.game.rand_message() {
            RandomMessage::SeeBat => println!("\nYOU SEE A BAT FLY BY"),
            RandomMessage::HearSound => {
                let sounds = ["A SCREAM", "FOOTSTEPS", "A WUMPUS", "THUNDER"];

                let i = self.rng.gen_range(0, sounds.len());

                println!("\nYOU HEAR {}", sounds[i]);
            }
            RandomMessage::Sneeze => println!("\nYOU SNEEZED"),
            RandomMessage::StepFrog => println!("\nYOU STEPPED ON A FROG"),
            RandomMessage::MonsterFrying => {
                println!("\nYOU SMELL {} FRYING", self.rand_monster_str())
            }
            RandomMessage::Watched => println!("\nYOU FEEL LIKE YOU'RE BEING WATCHED"),
            RandomMessage::Playing => println!("\nYOU ARE PLAYING WIZARD'S CASTLE"),
            RandomMessage::None => (),
        }
    }

    /// Equip player phase
    pub fn equip(&mut self) {
        self.race_gender_select();
        self.allocate_points();
        self.buy_armor();
        self.buy_weapon();
        self.buy_lamp();
        self.buy_flares();
    }

    /// Things to do at the start of the turn
    pub fn at_turn_start(&mut self) {
        self.game.add_turn(1);

        self.game.discover_room_at_player();

        self.game.curse_effects();

        self.game.curse_check();

        self.rand_message();

        // Cure blindness
        if self.game.cure_blindness() {
            println!("\nTHE OPAL EYE CURES YOUR BLINDNESS");
        }

        // Cure book stuck to hands
        if self.game.cure_book() {
            println!("\nTHE BLUE FLAME DISSOLVES THE BOOK");
        }
    }

    /// Quit the game
    pub fn quit(&mut self) -> bool {
        loop {
            match UI::get_input(Some("DO YOU REALLY WANT TO QUIT? ")).get(..1) {
                Some("Y") => {
                    self.game.quit();
                    break true;
                }
                Some("N") => {
                    println!("\n** THEN DON'T SAY YOU DO\n");
                    break false;
                }
                _ => println!("\n** ANSWER YES OR NO\n"),
            }
        }
    }

    /// Give help
    ///
    /// This wasn't in the original game
    pub fn help(&self) {
        println!("YOU CAN USE THE FOLLOWING COMMANDS:\n");

        println!("(N)ORTH      (D)OWN        (G)AZE");
        println!("(S)OUTH      (T)ELEPORT    (DR)INK");
        println!("(W)EST       (M)AP         (O)PEN");
        println!("(E)AST       (L)AMP        (H)ELP");
        println!("(U)P         (F)LARE       (Q)UIT");
    }
}

/// Main
fn main() {
    let mut playing = true;

    UI::intro();

    while playing {
        let game = Game::new(8, 8, 8);

        let mut ui = UI {
            game,
            rng: thread_rng(),
        };

        ui.equip();

        println!("\n\nOK {}, YOU ENTER THE CASTLE AND BEGIN.", ui.race_str());

        let mut alive = true;
        let mut automove = false;
        let mut free_move = false;

        let mut quiet = false;
        let mut print_location = true;
        let mut print_stats = true;
        let mut resolve_room_effects = true;

        while alive {
            // See if we were killed by something
            if ui.game.state() == GameState::Dead {
                alive = false;
                continue;
            }

            if !free_move {
                ui.at_turn_start();
            } else {
                free_move = false;
            }

            if automove {
                println!("\n");
                automove = false;
            } else {
                let mut valid_command = false;

                while !valid_command {
                    valid_command = true;

                    let command = UI::get_input(Some("\n\nYOUR MOVE? "));

                    println!();

                    if let Some("DR") = command.get(..2) {
                        ui.drink();
                        quiet = true;
                        break;
                    }

                    match command.get(..1) {
                        Some("M") => {
                            ui.map(false);
                            print_stats = false;
                            resolve_room_effects = false;
                        }
                        Some("N") => ui.move_dir(Direction::North),
                        Some("S") => ui.move_dir(Direction::South),
                        Some("W") => ui.move_dir(Direction::West),
                        Some("E") => ui.move_dir(Direction::East),
                        Some("U") => {
                            if !ui.move_stairs(Stairs::Up) {
                                quiet = true;
                            }
                        }
                        Some("D") => {
                            if !ui.move_stairs(Stairs::Down) {
                                quiet = true;
                            }
                        }
                        Some("T") => {
                            if !ui.teleport() {
                                quiet = true;
                            }
                        }
                        Some("L") => {
                            ui.lamp();
                            quiet = true;
                        }
                        Some("F") => {
                            if !ui.flare() {
                                print_location = false;
                            }
                            print_stats = false;
                            resolve_room_effects = false;
                        }
                        Some("G") => {
                            if !ui.gaze() {
                                quiet = true;
                            }
                        }
                        Some("O") => {
                            if !ui.open() {
                                quiet = true;
                            }
                        }
                        Some("Q") => {
                            if ui.quit() {
                                quiet = true;
                                alive = false;
                            }
                        }
                        Some("H") | Some("?") => {
                            ui.help();
                            free_move = true;
                            quiet = true;
                        }
                        _ => {
                            println!("** STUPID {} THAT WASN'T A VALID COMMAND", ui.race_str());
                            valid_command = false;
                        }
                    }
                }
            } // if !automove

            // See if the player walked out
            if ui.game.state() == GameState::Exit {
                alive = false;
                continue;
            }

            // See if we were killed by something (exploding chest)
            if ui.game.state() == GameState::Dead {
                alive = false;
                continue;
            }

            if quiet {
                print_location = false;
                print_stats = false;
                resolve_room_effects = false;
            }

            quiet = false;

            if print_location {
                ui.print_location();
            }

            print_location = true;

            if print_stats {
                ui.print_stats();
                ui.print_room();
            }

            print_stats = true;

            if resolve_room_effects {
                match ui.game.room_effect() {
                    Event::FoundGold(_) => {
                        println!("\nYOU HAVE {}", ui.game.player_gp());
                    }
                    Event::FoundFlares(_) => {
                        println!("\nYOU HAVE {}", ui.game.player_flares());
                    }
                    Event::Sinkhole => {
                        automove = true;
                    }
                    Event::Warp => {
                        automove = true;
                    }
                    Event::Combat(monster_type) => {
                        let retreated = ui.combat(monster_type);

                        automove = retreated;
                    }
                    Event::Treasure(_) => {
                        println!("\nIT'S NOW YOURS\n");
                    }
                    Event::Vendor => {
                        ui.vendor();
                    }
                    Event::None => (),
                }
            } // if resolve_room_effects

            resolve_room_effects = true;

            // If we're chosen to fight the vendor, let's do that
            if ui.game.state() == GameState::VendorAttack {
                automove = true;
                print_location = false;
                print_stats = false;
            }
        } // while alive

        ui.game_summary();

        if !ui.play_again() {
            playing = false;
        }
    } // while playing
}
