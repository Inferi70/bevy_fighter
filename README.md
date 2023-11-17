# bevy_fighter

this is an attempt at making a networked (p2p), 2d fighter game using rollback netcode.

It synchronizes only user input and relies on determinism for the simulation to
stay in sync. Uses the popular GGRS crate to do so.

Basicly for the final I want controlable characters, that have attack animations, hopefully hitboxes on the attacks

## Youtube video showing it off

https://youtu.be/x55EGR7V6lo

## Running it

Id recommend doing "single-player" atm cause network stuff doesnt work with current characters.

```shell
cargo run -- --players 1
```

For Multiplayer one player needs to install and run the server.
Install and run matchbox_server

```shell
cargo install matchbox_server
matchbox_server
```

...and run two instances of the "game":

```shell
cargo run
```

## Issues

Will always end up desyncing fast if i enable the player as a rollback object atm.

## Relevant links

This was built upon the github johanhelsing built.
link here: https://github.com/johanhelsing/bevy_gaff.github

Alot of the code has been removed or completly redone, mostly used as a base for the multiplayer.

## What i've added

so after taking the base template from johanhelsing, i've removed alot of functionality i dont need.
such as mouse inputs, wasm support, etc.

The major things ive added has been loading sprites, loading sprite animations, redone input system, 
Made the custom sprites (took along time making), methods to do the animations on clicks, 
handling changing directions (left to right, ect)
