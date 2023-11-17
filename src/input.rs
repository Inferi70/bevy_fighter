use bevy::core::{Pod, Zeroable};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ggrs::{LocalInputs, LocalPlayers};

use crate::GgrsConfig;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable, Debug, Default, Reflect)]
pub struct InferiInput {
    pub buttons: u16
}

// Direction inputs
pub const INPUT_UP: u16 = 1 << 0;
pub const INPUT_UP_JUST_PRESSED: u16 = 1 << 1;
pub const INPUT_DOWN: u16 = 1 << 2;
pub const INPUT_DOWN_JUST_PRESSED: u16 = 1 << 3;
pub const INPUT_LEFT: u16 = 1 << 4;
pub const INPUT_LEFT_JUST_PRESSED: u16 = 1 << 5;
pub const INPUT_RIGHT: u16 = 1 << 6;
pub const INPUT_RIGHT_JUST_PRESSED: u16 = 1 << 7;

// Attack inputs
pub const INPUT_PUNCH: u16 = 1 << 8;
pub const INPUT_PUNCH_JUST_PRESSED: u16 = 1 << 9;
pub const INPUT_KICK: u16 = 1 << 10;
pub const INPUT_KICK_JUST_PRESSED: u16 = 1 << 11;
pub const INPUT_SLASH: u16 = 1 << 12;
pub const INPUT_SLASH_JUST_PRESSED: u16 = 1 << 13;
pub const INPUT_HEAVY: u16 = 1 << 14;
pub const INPUT_HEAVY_JUST_PRESSED: u16 = 1 << 15;

pub fn input(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    let mut input: u16 = 0;

    // Direction Inputs
    if keyboard.pressed(KeyCode::W) {
        if keyboard.just_pressed(KeyCode::W) {
            input |= INPUT_UP_JUST_PRESSED;
        }
        input |= INPUT_UP;
    }
    if keyboard.pressed(KeyCode::A) {
        if keyboard.just_pressed(KeyCode::A) {
            input |= INPUT_LEFT_JUST_PRESSED;
        }
        input |= INPUT_LEFT;
    }
    if keyboard.pressed(KeyCode::S) {
        if keyboard.just_pressed(KeyCode::S) {
            input |= INPUT_DOWN_JUST_PRESSED;
        }
        input |= INPUT_DOWN;
    }
    if keyboard.pressed(KeyCode::D) {
        if keyboard.just_pressed(KeyCode::D) {
            input |= INPUT_RIGHT_JUST_PRESSED;
        }
        input |= INPUT_RIGHT;
    }

    // Attack inputs
    if keyboard.pressed(KeyCode::U) {
        if keyboard.just_pressed(KeyCode::U) {
            input |= INPUT_PUNCH_JUST_PRESSED;
        }
        input |= INPUT_PUNCH;
    }
    if keyboard.pressed(KeyCode::I) {
        if keyboard.just_pressed(KeyCode::I) {
            input |= INPUT_KICK_JUST_PRESSED;
        }
        input |= INPUT_KICK;
    }
    if keyboard.pressed(KeyCode::O) {
        if keyboard.just_pressed(KeyCode::O) {
            input |= INPUT_SLASH_JUST_PRESSED;
        }
        input |= INPUT_SLASH;
    }
    if keyboard.pressed(KeyCode::P) {
        if keyboard.just_pressed(KeyCode::P) {
            input |= INPUT_HEAVY_JUST_PRESSED;
        }
        input |= INPUT_HEAVY;
    }

    let inferi_input = InferiInput {
        buttons: input,
        ..default()
    };

    for handle in &local_players.0 {
        local_inputs.insert(*handle, inferi_input);
    }

    commands.insert_resource(LocalInputs::<GgrsConfig>(local_inputs));
}
