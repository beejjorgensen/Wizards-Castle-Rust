extern crate wizardscastle;

use std::io::{stdin,stdout,Write};

use wizardscastle::game::Game;
use wizardscastle::dungeon::Dungeon;
use wizardscastle::room::RoomType;
use wizardscastle::player::{Race, Gender, Player};

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

/*
1585 PRINT "WHICH SEX TO YOU PREFER";

1595 IF O$="M" THEN SX=1 : GOTO 1615
1600 IF O$="F" GOTO 1615
1605 PRINT "** CUTE ";R$(RC);", REAL CUTE. TRY M OR F."
*/

}

/// Main
fn main() {
    let mut game = Game::new(8, 8, 8);

    intro();

    race_gender_select(&mut game);

    map(&game.dungeon, &game.player, true);
}
