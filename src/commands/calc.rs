use crate::types::{Context, Error};
use crate::utils::calculator::Parser;

#[poise::command(
    slash_command,
    description_localized("en-US", "Calculate a math expression")
)]
pub(crate) async fn calc(
    ctx: Context<'_>,
    #[description = "Expression"] expression: String,
) -> Result<(), Error> {
    let result = Parser::new(&expression).parse();
    match result {
        Ok(v) => {
            ctx.say(format!("{expression} = {v}")).await?;
        }
        Err(e) => {
            ctx.say(e.to_string()).await?;
        }
    }
    Ok(())
}
