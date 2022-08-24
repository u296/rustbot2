use super::prelude::*;
use songbird::tracks::LoopState;

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
        }
        None => {
            info!("call does not exist");
            msg.channel_id.say(ctx, "not in a call").await?;
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("loop")]
async fn loop_command(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    loop_command_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn loop_command_proxy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let manager = songbird::get(ctx).await.unwrap();

    match manager.get(msg.guild_id.unwrap()) {
        None => {
            info!("there is no handler");
            msg.channel_id.say(ctx, "not in a call").await?;
            return Ok(());
        }
        Some(call) => {
            info!("there is a handler");

            let lock = call.lock().await;

            if lock.current_channel().is_none() {
                drop(lock); // in case await blocks for a while
                info!("not connected to a channel");
                msg.channel_id.say(ctx, "not in a call").await?;
                return Ok(());
            }

            if lock.queue().is_empty() {
                drop(lock);
                info!("queue is empty");
                msg.channel_id.say(ctx, "nothing playing").await?;
                return Ok(());
            }

            let track = lock.queue().current().unwrap();

            match track.get_info().await {
                Ok(info) => match info.loops {
                    LoopState::Finite(_) => match track.enable_loop() {
                        Ok(()) => {
                            msg.channel_id.say(ctx, "enabled looping").await?;
                        }
                        Err(e) => {
                            error!("failed to enable looping: {}", e);
                            return Err(e.into());
                        }
                    },
                    LoopState::Infinite => match track.disable_loop() {
                        Ok(()) => {
                            msg.channel_id.say(ctx, "disabled looping").await?;
                        }
                        Err(e) => {
                            error!("failed to disable looping: {}", e);
                            return Err(e.into());
                        }
                    },
                },
                Err(e) => {
                    error!("failed to get track info: {}", e);
                    return Err(e.into());
                }
            }
        }
    }

    Ok(())
}
