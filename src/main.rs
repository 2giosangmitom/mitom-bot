mod commands;
mod types;
mod utils;

use crate::types::Data;

use anyhow::{Context, Result};

use serenity::prelude::*;
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let token = env::var("DISCORD_TOKEN").context("missing DISCORD_TOKEN environment variable")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping::ping(), commands::calc::calc()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!(
                    bot_name = %ready.user.name,
                    "Bot connected"
                );

                poise::builtins::register_globally(ctx, &framework.options().commands)
                    .await
                    .context("failed to register commands")?;

                Ok(Data {})
            })
        })
        .build();

    let intents = GatewayIntents::empty();

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await
        .context("failed to create discord client")?;

    if let Err(err) = client.start().await {
        error!(error = ?err, "discord client error");

        return Err(err).context("discord client crashed");
    }

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
}
