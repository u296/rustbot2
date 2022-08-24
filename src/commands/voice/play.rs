use std::fmt::Debug;
use std::sync::Arc;

use super::prelude::*;
use songbird::{
    driver::Bitrate,
    input::{cached, ytdl_search, Input},
    ytdl, Call,
};

const BITRATE: Bitrate = Bitrate::BitsPerSecond(128000);

#[command]
#[only_in(guilds)]
#[aliases("play", "p")]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    play_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn play_proxy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();

    let download_audio_handle = tokio::spawn(get_yt_source::<String>(args.message().into()));

    let manager = songbird::get(ctx).await.unwrap();

    let call_retrieve = async move {
        let channel = match guild.voice_states.get(&msg.author.id) {
            None => {
                info!("author does not have voice state");
                msg.channel_id
                    .say(ctx, "you are not in a voice channel")
                    .await?;
                return Ok::<Option<Arc<Mutex<Call>>>, Box<dyn Error + Send + Sync>>(None);
            }
            Some(vs) => match vs.channel_id {
                None => {
                    info!("author was not in voice channel");
                    msg.channel_id
                        .say(ctx, "you are not in a voice channel")
                        .await?;
                    return Ok(None);
                }
                Some(c) => c,
            },
        };

        if let Some(call) = manager.get(guild.id) {
            if call.lock().await.current_channel() == Some(channel.into()) {
                info!("call in the right channel already exists");
                Ok(Some(call))
            } else {
                info!("call exists, but is in the wrong channel. Purging.");

                match manager.remove(guild.id).await {
                    Err(e) => {
                        error!("failed to remove handler: {}", e);
                        return Err(e.into());
                    }
                    Ok(_) => (),
                }

                let (newcall, result) = manager.join(guild.id, channel).await;
                match result {
                    Ok(_) => (),
                    Err(e) => {
                        error!("failed to join: {}", e);
                        return Err(e.into());
                    }
                }

                Ok(Some(newcall))
            }
        } else {
            info!("call doesn't exist");
            let (call, result) = manager.join(guild.id, channel).await;
            match result {
                Ok(_) => (),
                Err(e) => {
                    error!("failed to join: {}", e);
                    return Err(e.into());
                }
            }
            Ok(Some(call))
        }
    };

    let call = match call_retrieve.await? {
        Some(c) => c,
        None => {
            download_audio_handle.abort();
            return Ok(());
        }
    };

    msg.channel_id.say(ctx, "searching").await?;

    let source = download_audio_handle.await??;

    let mut lock = call.lock().await;
    lock.enqueue_source(source);

    if lock.queue().len() > 1 {
        msg.channel_id.say(ctx, "placed in queue").await?;
    } else {
        msg.channel_id.say(ctx, "playing").await?;
    }

    Ok(())
}

#[instrument]
async fn get_yt_source<S>(text: S) -> Result<Input, Box<dyn Error + Send + Sync>>
where
    S: AsRef<str> + Debug,
{
    let src = match if text.as_ref().starts_with("https://") {
        info!("received link");
        ytdl(text).await
    } else {
        info!("received search term");
        ytdl_search(text).await
    } {
        Ok(s) => s,
        Err(e) => {
            error!("youtube-dl failed: {}", e);
            return Err(e.into());
        }
    };

    info!("finished streaming");

    let compressed = match cached::Compressed::new(src, BITRATE) {
        Ok(c) => c,
        Err(e) => {
            error!("failed to compress stream: {}", e);
            return Err(e.into());
        }
    };

    info!("finished compressing");

    Ok(compressed.into())
}
