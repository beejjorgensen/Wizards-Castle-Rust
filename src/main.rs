extern crate wizardscastle;
extern crate rand; 

use std::io::{stdin,stdout,Write};

use self::rand::Rng;
use self::rand::thread_rng;

use wizardscastle::game::Game;
use wizardscastle::dungeon::Dungeon;
use wizardscastle::room::RoomType;
use wizardscastle::player::{Race, Gender, Player, Stat};
use wizardscastle::armor::ArmorType;
use wizardscastle::weapon::WeaponType;

/// Return a random monster name
fn rand_monster_str() -> String {
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
    ];

    let mut rng = thread_rng();

    let i = rng.gen_range(0, name.len());

    String::from(name[i]).to_uppercase()
}

fn starts_with_vowel(s: &str) -> bool {
    if let Some(c) = String::from(s).to_uppercase().chars().next() {
        return c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U';
    }

    return false;
}

fn get_article(s: &str) -> String {
    if starts_with_vowel(s) {
        return String::from("AN");
    }

    String::from("A")
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

fn race_str(player:&Player) -> &str {
    match player.race {
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
fn race_gender_select(game: &mut Game) {
    let race = loop {

        println!("ALL RIGHT, BOLD ONE.");
        println!("YOU MAY BE AN ELF, DWARF, MAN, OR HOBBIT.\n");

        let race_str = get_input(Some("YOUR CHOICE? "));

        match race_str.get(..1) {
            Some("H") => break Race::Hobbit,
            Some("E") => break Race::Elf,
            Some("M") => break Race::Human,
            Some("D") => break Race::Dwarf,
            _ => println!("** THAT WAS INCORRECT. PLEASE TYPE E, D, M, OR H.\n"),
        }
    };

    game.player.init(race);

    let gender = loop {
        let gender_str = get_input(Some("\nWHICH SEX TO YOU PREFER? "));

        match gender_str.get(..1) {
            Some("M") => break Gender::Male,
            Some("F") => break Gender::Female,
            _ => println!("** CUTE {}, REAL CUTE. TRY M OR F.", race_str(&game.player)),
        }
    };

    game.player.set_gender(gender);
}

fn allocate_points(game: &mut Game) {
    println!("\nOK {}, YOU HAVE THESE STATISTICS:\n", race_str(&game.player));

    println!("STRENGTH= {} INTELLIGENCE= {} DEXTERITY= {}\n",
        game.player.st, game.player.iq, game.player.dx);

    println!("AND {} OTHER POINTS TO ALLOCATE AS YOU WISH.\n", game.player.additional_points);

    let stats = vec!(Stat::Intelligence, Stat::Strength, Stat::Dexterity);
    let stat_names = vec!("INTELLIGENCE", "STRENGTH", "DEXTERITY");

    for i in 0..3 {
        let mut ok = false;

        while !ok {
            let s = get_input(Some(&format!("HOW MANY POINTS DO YOU ADD TO {}? ", stat_names[i])));

            let points_to_add;
            
            match s.parse::<usize>() {
                Ok(p) => points_to_add = p,
                Err(_) => {
                    print!("\n** ");
                    continue;
                },
            };

            if let Ok(_) = game.player.allocate_points(&stats[i], points_to_add) {
                ok = true;
            } else {
                print!("\n** ");
                continue;
            }
        }

        if game.player.additional_points == 0 {
            return;
        }
    }
}

/// Buy armor
fn buy_armor(game: &mut Game) {
    println!("\nOK, {}, YOU HAVE {} GOLD PIECES (GP's)\n", race_str(&game.player), game.player.gp);

    println!("HERE IS A LIST OF ARMOR YOU CAN BUY (WITH COST IN <>)\n");

    println!("PLATE<30> CHAINMAIL<20> LEATHER<10> NOTHING<0>");

    let _ = loop {
        let armor_str = get_input(Some("\nYOUR CHOICE? "));

        match armor_str.get(..1) {

            Some("P") => break game.player.purchase_armor(ArmorType::Plate, false),
            Some("C") => break game.player.purchase_armor(ArmorType::Chainmail, false),
            Some("L") => break game.player.purchase_armor(ArmorType::Leather, false),
            Some("N") => break game.player.purchase_armor(ArmorType::None, false),
            _ => {
                let mon_str = rand_monster_str();
                let article = get_article(&mon_str);

                println!("\n** ARE YOU A {} OR {} {}? TYPE P,C,L OR N", race_str(&game.player), article, mon_str);
            },
        }
    };
}

fn buy_weapon(game: &mut Game) {

    println!("\nOK, BOLD {}, YOU HAVE {} GP's LEFT\n", race_str(&game.player), game.player.gp);

    println!("HERE IS A LIST OF WEAPONS YOU CAN BUY (WITH COST IN <>)\n");

    println!("SWORD<30> MACE<20> DAGGER<10> NOTHING<0>");

    let _ = loop {
        let armor_str = get_input(Some("\nYOUR CHOICE? "));

        match armor_str.get(..1) {

            Some("S") => break game.player.purchase_weapon(WeaponType::Sword, false),
            Some("M") => break game.player.purchase_weapon(WeaponType::Mace, false),
            Some("D") => break game.player.purchase_weapon(WeaponType::Dagger, false),
            Some("N") => break game.player.purchase_weapon(WeaponType::None, false),
            _ => println!("\n** IS YOUR IQ REALLY {}? TYPE S, M, D, OR N", game.player.iq),
        }
    };
}

fn buy_lamp(game: &mut Game) {
    if !game.player.can_purchase_lamp() {
        return;
    }

    let _ = loop {
        let lamp_str = get_input(Some("\nWANT TO BUY A LAMP FOR 20 GP's? "));

        match lamp_str.get(..1) {
            Some("Y") => break game.player.purchase_lamp(true),
            Some("N") => break game.player.purchase_lamp(false),
            _ => println!("\n** ANSWER YES OR NO"),
        }
    };
}

fn buy_flares(game: &mut Game) {
    let max_flares = game.player.max_flares();

    if max_flares == 0 {
        return;
    }

    println!("\nOK, {}, YOU HAVE {} GOLD PIECES LEFT\n", race_str(&game.player), game.player.gp);

    loop {
        let flare_str = get_input(Some("FLARES COST 1 GP EACH, HOW MANY DO YOU WANT? "));

        let flare_count;
        
        match flare_str.parse::<usize>() {
            Ok(f) => flare_count = f,
            Err(_) => {
                print!("** IF YOU DON'T WANT ANY JUST TYPE 0 (ZERO)\n\n");
                continue;
            },
        };

        match game.player.purchase_flares(flare_count) {
            Ok(_) => break,
            Err(_) => {
                print!("** YOU CAN ONLY AFFORD {}\n\n", max_flares);
                continue;
            }
        }
    };
}

/// Main
fn main() {
    let mut game = Game::new(8, 8, 8);

    intro();

    race_gender_select(&mut game);
    allocate_points(&mut game);
    buy_armor(&mut game);
    buy_weapon(&mut game);
    buy_lamp(&mut game);
    buy_flares(&mut game);

    let playing = true;

    while playing {
        map(&game.dungeon, &game.player, true);
    }
}
