// Needed for some reason in tests where very large jsons are hardcoded
#![recursion_limit = "256"]

pub mod agent;
pub mod args;
pub mod game;
pub mod ws_client;
