use crate::types::{Context, Error};

#[poise::command(slash_command, description_localized("en-US", "Responds with Pong!"))]
pub(crate) async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}
