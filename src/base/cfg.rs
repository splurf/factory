use clap::Parser;
use serenity::all::{ChannelId, GatewayIntents};
use std::{str::FromStr, time::Duration};

use crate::Country;

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Config {
    #[arg(short, long)]
    token: String,

    #[arg(required = true)]
    channel_id: ChannelId,

    #[arg(required = true, num_args = 1.., value_parser = Country::from_str, value_enum)]
    countries: Vec<Country>,
}

impl Config {
    pub const DELAY: Duration = Duration::from_secs(900);
    pub const INTENTS: GatewayIntents = GatewayIntents::GUILD_MEMBERS;
    pub const ENDPOINT: &'static str = "https://nfs.faireconomy.media/ff_calendar_thisweek.json";

    pub fn new() -> Self {
        Self::parse()
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    pub fn countries(&self) -> &[Country] {
        self.countries.as_slice()
    }
}
