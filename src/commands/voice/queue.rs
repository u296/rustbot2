use super::prelude::*;

#[command]
#[only_in(guilds)]
#[aliases("s", "skip")]
async fn skip_command(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    skip_command_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn skip_command_proxy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();
    let manager = songbird::get(ctx).await.unwrap();

    match manager.get(guild.id) {
        Some(call) => {
            info!("call exists");
            match call.lock().await.queue().skip() {
                Ok(_) => (),
                Err(e) => {
                    error!("failed to skip queue: {}", e);
                    return Err(e.into());
                }
            }
        },
        None => {
            info!("call does not exist");
            msg.channel_id.say(ctx, "not in a call").await?;
        }
    }

    Ok(())
}