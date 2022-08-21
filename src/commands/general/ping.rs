use super::prelude::*;

use tracing::*;

#[command]
async fn ping(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("ping");
    ping_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn ping_proxy(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    println!("ping proxy");
    msg.channel_id.say(ctx, "ping").await?;
    Ok(())
}