extern crate wizardscastle;

use std::io::{stdin,stdout,Write};

use wizardscastle::game::Game;
use wizardscastle::dungeon::Dungeon;
use wizardscastle::room::RoomType;
use wizardscastle::player::{Race, Gender, Player, Stat};

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
    println!("{:*^64}\n", "");

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

    game.player.set_race(race);

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

        println!();

        if game.player.additional_points == 0 {
            return;
        }
    }
}

/// Main
fn main() {
    let mut game = Game::new(8, 8, 8);

    intro();

    race_gender_select(&mut game);

    allocate_points(&mut game);

    map(&game.dungeon, &game.player, true);
}
