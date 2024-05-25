use chrono::Local;
use serenity::{
    all::{CacheHttp, Context, CreateMessage, EventHandler, Ready, ResumedEvent},
    async_trait,
};
use tokio::time::sleep;

use crate::{get_items, Config, Error, ErrorKind, Items, Result};

pub struct Handler {
    cfg: Config,
}

impl Handler {
    pub const fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    async fn _routine(&self, ctx: &Context) -> Result<()> {
        // retrieve and filter the events from this current week
        let new_items = get_items(&self.cfg).await?;

        // retrieve current map of data
        let mut data = ctx.data.write().await;
        let cnt_items = data
            .get_mut::<Items>()
            .ok_or(Error::from(ErrorKind::Unexpected))?;

        // add/replace new/updated items
        for item in new_items {
            cnt_items.insert(item.title().to_string(), item);
        }

        // filter distant events
        let now = Local::now();
        let upcoming = cnt_items
            .values()
            .filter(|item| (item.date() - now).num_hours() <= 1);

        // the configured channel
        let channel = ctx.http().get_channel(self.cfg.channel_id()).await?.id();

        // send updates to channel
        for item in upcoming {
            channel
                .send_message(ctx.http(), CreateMessage::new().content(item))
                .await?;
        }
        Ok(())
    }

    pub async fn routine(&self, ctx: Context) {
        loop {
            if let Err(e) = self._routine(&ctx).await {
                eprintln!("{:?}", e)
            }
            sleep(Config::DELAY).await
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        self.routine(ctx).await
    }

    async fn resume(&self, ctx: Context, _: ResumedEvent) {
        self.routine(ctx).await
    }
}
