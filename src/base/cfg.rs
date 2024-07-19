use clap::Parser;
use serenity::all::{GatewayIntents, UserId};
use std::time::Duration;

use crate::{Country, Impact};

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Config {
    #[arg(short, long)]
    token: String,

    #[arg(short, long, default_value_t, value_enum)]
    impact: Impact,

    #[arg(short, long, default_value_t = 2, value_parser = |s: &str| s.parse::<u8>().map(i64::from))]
    notice: i64,

    #[arg(required = true, num_args = 1..)]
    users: Vec<UserId>,

    #[arg(required = true, num_args = 1.., value_enum, last = true)]
    countries: Vec<Country>,
}

impl Config {
    pub const DELAY: Duration = Duration::from_secs(600);
    pub const INTENTS: GatewayIntents = GatewayIntents::GUILD_MEMBERS;
    pub const ENDPOINT: &'static str = "https://nfs.faireconomy.media/ff_calendar_thisweek.json";

    pub fn new() -> Self {
        Self::parse()
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub const fn impact(&self) -> Impact {
        self.impact
    }

    pub const fn notice(&self) -> i64 {
        self.notice
    }

    pub fn users(&self) -> &[UserId] {
        self.users.as_slice()
    }

    pub fn countries(&self) -> &[Country] {
        self.countries.as_slice()
    }
}
