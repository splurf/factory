use chrono::{DateTime, Local};
use reqwest::Client;

use crate::{Config, ErrorKind, Event, Result};

fn is_valid_event(cfg: &Config, event: &Event, now: DateTime<Local>) -> bool {
    cfg.countries().contains(&event.country()) && event.is_normal() && now < event.date()
}

pub async fn get_events(cfg: &Config) -> Result<impl Iterator<Item = Event> + '_> {
    let client = Client::default();
    let res = client.get(Config::ENDPOINT).send().await?;

    if !res.status().is_success() {
        return Err(ErrorKind::ConnectionFailed.into());
    }

    let json = res.json::<Vec<Event>>().await?;

    let data = json
        .into_iter()
        .filter(|event| is_valid_event(cfg, event, Local::now()));

    Ok(data)
}
