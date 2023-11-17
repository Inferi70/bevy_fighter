use bevy::prelude::*;
use clap::Parser;
use serde::Deserialize;
use std::ffi::OsString;

#[derive(Parser, Debug, Clone, Deserialize, Resource)]
#[serde(default)]
#[clap(
    name = "box_fighter",
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(long, default_value = "ws://127.0.0.1:3536")]
    pub matchbox: String,

    #[clap(long)]
    pub room: Option<String>,

    #[clap(long, short, default_value = "2")]
    pub players: usize,
}

impl Default for Args {
    fn default() -> Self {
        let args = Vec::<OsString>::new();
        Args::parse_from(args)
    }
}

impl Args {
    pub fn get() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Args::parse()
        }
    }
}
