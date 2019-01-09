# Wizard's Castle

I'm coding up the classic game [Wizard's
Castle](https://github.com/beejjorgensen/Wizards-Castle-Info) as a way to learn
Rust.

Since this is a learning project, it's going to be weird, unstructured,
non-idiomatic, and subject to frequent refactoring.

Please don't hold it against me.

## Goals

* Learn Rust

* Have a Wizard's Castle library that other front-ends can use to play the game
  * Build an ncurses front end
  * Build to WASM with a web front end

* Have a default binary that plays the game in the classic text format

## TODO

* Random eating messages
* Random messages between turns
* Give monster initiative if blind
* Check all other places blindness should have an effect
* Spells
  * Check for monster stuck in web
  * Check for web breaking

### Refactoring TODO

* Have `room_at` and `room_at_mut` return undiscovered status
* Make fewer fields `pub` and provide accessors
  * Notably in `Room`
* Break up the `main.rs` and `game.rs` code into smaller pieces
* Move player buying things code out of player into game?

<beej@beej.us>
