# Wizard's Castle

I'm coding up the classic game [Wizard's
Castle](https://github.com/beejjorgensen/Wizards-Castle-Info) as a way to learn
Rust.

Since this is a learning project, it's going to be weird, unstructured,
non-idiomatic, and subject to frequent refactoring.

Please don't hold it against me.

## Goals

* Learn Rust

* Have a Wizard's Castle library that other front-ends can use to play the game.
  * Build an ncurses front end.
  * Build to WASM with a web front end.

* Have a default binary that plays the game in the classic text format.

## TODO

* Make fewer fields `pub` and provide accessors
  * Notably in `Room`
* Random eating messages
* Curse effects
* Random messages between turns
* Have `room_at` and `room_at_mut` return undiscovered status
* Give monster initiative if blind or lethargic
* Check all other places blindness should have an effect
* Spells
  * Check for monster stuck in web
  * Checck for web breaking
* Break up the `main.rs` and `game.rs` code into smaller pieces

<beej@beej.us>