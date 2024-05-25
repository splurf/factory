mod base;

use base::*;

#[tokio::main]
async fn main() -> serenity::Result<()> {
    let cfg = Config::new();

    let mut client = serenity::Client::builder(cfg.token(), Config::INTENTS)
        .event_handler(Handler::new(cfg))
        .await?;

    {
        client
            .data
            .write()
            .await
            .insert::<Items>(Default::default());
    }

    client.start_autosharded().await
}
