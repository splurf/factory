use chrono::{DateTime, Local};
use reqwest::Client;

use crate::{Config, ErrorKind, Item, Result};

fn is_valid_item(cfg: &Config, item: &Item, now: DateTime<Local>) -> bool {
    cfg.countries().contains(&item.country()) && item.is_normal() && now < item.date()
}

pub async fn get_items(cfg: &Config) -> Result<impl Iterator<Item = Item> + '_> {
    let client = Client::default();
    let res = client.get(Config::ENDPOINT).send().await?;

    if !res.status().is_success() {
        return Err(ErrorKind::ConnectionFailed.into());
    }

    let json = res.json::<Vec<Item>>().await?;

    let data = json
        .into_iter()
        .filter(|item| is_valid_item(cfg, item, Local::now()));

    Ok(data)
}
