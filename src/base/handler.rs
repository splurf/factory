use chrono::Local;
use futures::future::try_join_all;
use serenity::{
    all::{CacheHttp, Context, CreateMessage, EventHandler, Ready, ResumedEvent},
    async_trait,
};
use tokio::time::sleep;

use crate::{get_events, Config, Entry, Error, ErrorKind, Events, Result};

pub struct Handler {
    cfg: Config,
}

impl Handler {
    pub const fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    async fn _routine(&self, ctx: &Context) -> Result<()> {
        // retrieve and filter the events from this current week
        let new_events = get_events(&self.cfg).await?;

        // retrieve current map of data
        let mut data = ctx.data.write().await;
        let cnt_events = data
            .get_mut::<Events>()
            .ok_or(Error::from(ErrorKind::Unexpected))?;

        // add/replace new/updated Events
        for value in new_events {
            let key = value.title().to_string();

            if let Some(entry) = cnt_events.get_mut(&key) {
                if entry.date() < value.date() {
                    entry.update(value);
                }
                continue;
            }
            cnt_events.insert(key, Entry::new(value));
        }

        // filter and map raw info for each event
        let now = Local::now();
        let upcoming = cnt_events.values_mut().filter_map(|event| {
            (!event.is_flagged() && (event.date() - now).num_hours() <= 1).then_some({
                event.flag();
                event.to_string()
            })
        });

        // directly retrieve users
        let users = try_join_all(
            self.cfg
                .users()
                .into_iter()
                .map(|user| ctx.http().get_user(*user)),
        )
        .await?;

        // send each new event to each user
        for raw_event in upcoming {
            let builder = CreateMessage::new().content(raw_event);

            for user in users.as_slice() {
                user.dm(ctx.http(), builder.clone()).await?;
            }
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
