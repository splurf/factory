use futures::future::try_join_all;
use serenity::{
    all::{CacheHttp, Context, CreateMessage, EventHandler, Ready, ResumedEvent, User},
    async_trait,
};
use std::collections::HashMap;
use tokio::time::sleep;

use crate::{get_events, Config, Entry, Error, ErrorKind, Events, Result};

pub struct Handler {
    cfg: Config,
}

impl Handler {
    pub const fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    fn update_events(
        cnt_events: &mut HashMap<String, Entry>,
        new_events: impl Iterator<Item = crate::Event>,
    ) {
        for value in new_events {
            let key = value.title().to_string();

            if let Some(entry) = cnt_events.get_mut(&key) {
                // unflag event if updated time is greater than current
                if entry.date() < value.date() {
                    entry.update(value);
                }
                continue;
            }
            cnt_events.insert(key, Entry::new(value));
        }
    }

    fn get_upcoming(cnt_events: &mut HashMap<String, Entry>) -> impl Iterator<Item = String> + '_ {
        cnt_events.values_mut().filter_map(|event| {
            if event.is_flagged() {
                return None;
            }
            event.flag();
            Some(event.to_string())
        })
    }

    async fn get_users(&self, ctx: &Context) -> Result<Vec<User>> {
        try_join_all(
            self.cfg
                .users()
                .iter()
                .map(|user| ctx.http().get_user(*user)),
        )
        .await
        .map_err(Into::into)
    }

    async fn distribute_events(
        ctx: &Context,
        upcoming: impl Iterator<Item = String>,
        users: Vec<User>,
    ) -> Result<()> {
        for raw_event in upcoming {
            let builder = CreateMessage::new().content(raw_event);

            for user in users.as_slice() {
                user.dm(ctx.http(), builder.clone()).await?;
            }
        }
        Ok(())
    }

    async fn _routine(&self, ctx: &Context) -> Result {
        // retrieve and filter the events from this current week
        let new_events = get_events(&self.cfg).await?;

        // retrieve current map of data
        let mut data = ctx.data.write().await;
        let cnt_events = data
            .get_mut::<Events>()
            .ok_or(Error::from(ErrorKind::Unexpected))?;

        // add/replace new/updated Events
        Self::update_events(cnt_events, new_events);

        // filter and map raw info for each event
        let upcoming = Self::get_upcoming(cnt_events);

        // directly retrieve users
        let users = self.get_users(ctx).await?;

        // send each new event to each user
        Self::distribute_events(ctx, upcoming, users).await
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
