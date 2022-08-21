use super::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use serenity::model::channel::{ChannelType, Channel};

#[command]
#[aliases("pick", "random")]
async fn select_random(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    select_random_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn select_random_proxy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return Ok(());
    }

    let words: Vec<String> = args.iter().filter_map(|m| m.ok()).collect();

    let s = match words.choose(&mut thread_rng()) {
        Some(s) => s,
        None => return Ok(()),
    };

    if !s.trim().is_empty() {
        msg.channel_id.say(ctx, s).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn split(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    split_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn split_proxy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();

    match guild.voice_states.get(&msg.author.id) {
        None => {
            info!("author was not in guild voice states");
            msg.channel_id
                .say(ctx, "you are not in a voice channel")
                .await?;
        }
        Some(voice_state) => {
            if voice_state.channel_id.is_none() {
                warn!("author was in guild voice states, but it did not have a channel");
                msg.channel_id
                    .say(ctx, "you are not in a voice channel")
                    .await?;
                return Ok(());
            }

            let target_voice_channel = match guild
                .channels
                .iter()
                .filter_map(|(_, c)| match c {
                    Channel::Guild(c) => Some(c),
                    _ => None,
                })
                .filter_map(|channel| {
                    if channel.kind == ChannelType::Voice {
                        Some(channel)
                    } else {
                        None
                    }
                })
                .find(|c| c.name == args.message()) {
                    Some(c) => {
                        c
                    }
                    None => {
                        msg.channel_id.say(ctx, "you need to specify a voice channel to split to").await?;
                        return Ok(())
                    }
                };

            let call_channel_id = voice_state.channel_id.unwrap();

            let mut users_in_call: Vec<_> = guild
                .voice_states
                .iter()
                .filter_map(|(userid, voicestate)| {
                    if voicestate.channel_id == Some(call_channel_id) {
                        Some(*userid)
                    } else {
                        None
                    }
                })
                .collect();

            users_in_call.shuffle(&mut thread_rng());

            let users_to_move = &users_in_call[0..(users_in_call.len() / 2)];

            for u in users_to_move.iter() {
                guild.move_member(ctx, u, target_voice_channel.id).await?;
            }
        }
    }

    Ok(())
}
