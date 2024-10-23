// Needed for some reason in tests where very large jsons are hardcoded
#![recursion_limit = "256"]

pub mod args;
pub mod bot;
pub mod bot_trait;
pub mod ws_client;
