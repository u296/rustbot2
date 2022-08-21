use super::prelude::*;

#[command]
async fn echo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("ping");
    echo_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn echo_proxy(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    println!("ping proxy");
    msg.channel_id.say(ctx, "ping").await?;
    Ok(())
}