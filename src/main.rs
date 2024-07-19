mod base;

use base::*;

#[tokio::main]
async fn main() -> Result {
    let cfg = Config::new();

    let mut client = serenity::Client::builder(cfg.token(), Config::INTENTS)
        .event_handler(Handler::new(cfg))
        .await?;

    {
        client
            .data
            .write()
            .await
            .insert::<Events>(Default::default());
    }

    client.start_autosharded().await.map_err(Into::into)
}
