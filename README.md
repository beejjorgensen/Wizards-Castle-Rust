# Wizard's Castle

I'm coding up the classic game [Wizard's
Castle](https://github.com/beejjorgensen/Wizards-Castle-Info) as a way to learn
Rust.

Since this is a learning project, it's going to be weird, unstructured,
non-idiomatic, and subject to frequent refactoring.

Please don't hold it against me.

## How to Play

Clone the repo and `cargo run`.

### Commands

| Command |                      |
|:-------:|----------------------|
|   `N`   | North                |
|   `S`   | South                |
|   `W`   | West                 |
|   `E`   | East                 |
|   `U`   | Up                   |
|   `D`   | Down                 |
|   `T`   | Teleport             |
|   `M`   | Map                  |
|   `L`   | Lamp                 |
|   `F`   | Flare                |
|   `G`   | Gaze into an orb     |
|   `DR`  | Drink from a pool    |
|   `O`   | Open a chest or book |
|   `H`   | Help                 |
|   `Q`   | Quit                 |

## Goals

* Learn Rust

* Have a Wizard's Castle library that other front-ends can use to play the game
  * Build an ncurses front end
  * Build to WASM with a web front end

## TODO

* Have `room_at` and `room_at_mut` return undiscovered status
* Make fewer fields `pub` and provide accessors
  * Notably in `Room`
* Move player buying things code out of player into game?

<beej@beej.us>
