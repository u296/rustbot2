use super::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    msg.channel_id.say(ctx, "ping").await?;
    Ok(())
}
