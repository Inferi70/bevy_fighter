# bevy_fighter

this is an attempt at making a networked (p2p), 2d fighter game using rollback netcode.

What is rollback?
It synchronizes the game state using only user input and relies on determinism for the simulation to
stay in sync. Uses the popular GGRS crate to do so.
When it isn't in sync rollback will goto a previous snapshot, and rerun with the new correct inputs it receives.
All within a single game frame.

At the moment I only send the inputs, cause the demo doesn't have good enough determinism for my experience,
to keep everything in sync, so i dont do rollback on the characters atm so when they are desynced they are forever desynced.

<<<<<<< HEAD
## Youtube video showing it off

https://youtu.be/x55EGR7V6lo
=======
For this project I aim to have seperate controllable characters,
that have some for of animations in place, aswell as having hitboxes.

Currently the Hitboxes aren't implemented, But we did get p2p networking to work.

## Controls

WASD, are used for movement.
JKL:UIOP are used for moves.

U is the only move currently implemented and its for the Punch button.
>>>>>>> 6a4506c (Redone animation system, redone input handling (old function was movement new is input_handling), updated readme)

## Running it

Id recommend doing "single-player" atm cause cause running multiplayer requires instlling the server.
```shell
cargo run -- --players 1
```

For Multiplayer one player needs to install and run the server (Not both, just one).
Install and run matchbox_server

```shell
cargo install matchbox_server
matchbox_server
```

Then run two instances of the "game", Could be seperate computers but easiest with one for testing:

```shell
cargo run
```

## Issues

Will always end up desyncing fast if I enable the player as a rollback object, cause of the determinism of the physics engine.
No HP system or hitbox system on attacks.

## Relevant links

This was built upon the github johanhelsing built.
link here: https://github.com/johanhelsing/bevy_gaff.github

Alot of the code has been removed or completly redone, mostly used as a base for the multiplayer p2p stuff.

## Licensing

Some of the stuff used in this project has licensing, These files can be found inside the files.
The licensing for the background img is inside the assests folder next to the img.
the licensing from johanhelsings project can be found at the main folder root.

## What i've added

so after taking the base template from johanhelsing, i've removed alot of functionality i dont need.
such as mouse inputs, wasm support, etc.

The major things ive added has been loading sprites, loading sprite animations and playing of animations on button inputs, redone input system (allowing for two players using same keys), 
Made the custom sprites (took along time making aka all sprite other then background was made by me), methods to stop the animations, 
handling changing directions (left to right, ect)
