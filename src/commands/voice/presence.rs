use super::prelude::*;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    join_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn join_proxy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();

    let channel = match guild.voice_states.get(&msg.author.id) {
        None => {
            info!("author has no voice state");
            msg.channel_id.say(ctx, "you are not in a channel").await?;
            return Ok(());
        }
        Some(voice_state) => match voice_state.channel_id {
            None => {
                info!("author is not in a channel");
                msg.channel_id.say(ctx, "you are not in a channel").await?;
                return Ok(());
            }
            Some(c) => c,
        },
    };

    let manager = songbird::get(ctx).await.unwrap();

    if let Some(call) = manager.get(guild.id) {
        if call.lock().await.current_channel() == Some(channel.into()) {
            info!("already in channel");
        } else {
            info!("call exists, but not in right channel");
            match manager.remove(guild.id).await {
                Ok(()) => (),
                Err(e) => {
                    error!("failed to remove handler: {}", e);
                    return Err(e.into());
                }
            }

            let (_call, result) = manager.join(guild.id, channel).await;
            match result {
                Ok(()) => (),
                Err(e) => {
                    error!("failed to join: {}", e);
                    return Err(e.into());
                }
            }
        }
    } else {
        info!("call does not exist");
        let (_call, result) = manager.join(guild.id, channel).await;
        match result {
            Ok(()) => (),
            Err(e) => {
                error!("failed to join: {}", e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("stfu", "stop", "shutup")]
async fn leave(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    leave_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn leave_proxy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();

    let manager = songbird::get(ctx).await.unwrap();

    if manager.get(guild.id).is_some() {
        info!("there is a call");
        match manager.remove(guild.id).await {
            Ok(_) => (),
            Err(e) => {
                error!("failed to remove handler: {}", e);
                return Err(e.into());
            }
        }
    } else {
        info!("there is no call");
    }

    Ok(())
}
