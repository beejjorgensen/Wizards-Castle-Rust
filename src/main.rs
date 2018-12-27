extern crate wizardscastle;
extern crate rand; 

use std::io::{stdin,stdout,Write};

use self::rand::Rng;
use self::rand::thread_rng;

use wizardscastle::game::{Game,Direction,Event};
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
    fn intro(&self) {
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
}

/// Main
fn main() {
    let game = Game::new(8, 8, 8);

    let mut ui = UI {
        game: game,
        turn_count: 0,
    };

    ui.intro();

    ui.race_gender_select();
    ui.allocate_points();
    ui.buy_armor();
    ui.buy_weapon();
    ui.buy_lamp();
    ui.buy_flares();

    println!("\nOK {}, YOU ENTER THE CASTLE AND BEGIN.\n", ui.race_str());

    let playing = true;
    ui.turn_count = 0;

    while playing {

        let alive = true;

        while alive {
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
                Event::None => (),
            }

            ui.turn_count += 1;

            if automove {
                continue;
            }

            // TODO curse effects

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
                        println!("\n** STUPID {} THAT WASN'T A VALID COMMAND\n", ui.race_str());
                        valid_command = false;
                    }
                }

            }
        }
    }
}
