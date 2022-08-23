use super::prelude::*;
use songbird::{input::{Input, ytdl_search, cached}, ytdl, driver::Bitrate};

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

    let (manager, source) = join!(songbird::get(ctx), get_yt_source(args.message()));

    let manager = manager.unwrap();
    let source = source?;


    let channel = match guild.voice_states.get(&msg.author.id) {
        None => {
            error!("author does not have voice state");
            return Ok(())
        },
        Some(vs) => {
            match vs.channel_id {
                None => {
                    msg.channel_id.say(ctx, "you are not in a voice channel").await?;
                    info!("author was not in voice channel");
                    return Ok(());
                },
                Some(c) => c
            }
        }
    };

    let mut maybe_call = None;

    let purge_call = if let Some(call) = manager.get(guild.id) {
        if call.lock().await.current_channel() == Some(channel.into()) {
            maybe_call = Some(call);
            false
            
        } else {
            true
        }
    } else {
        true
    };



    if purge_call {
        match manager.remove(guild.id).await {
            Err(e) => {
                error!("failed to remove handler: {}", e);
                return Err(e.into());
            },
            Ok(_) => ()
        }

        let (call, result) = manager.join(guild.id, channel).await;
        match result {
            Ok(_) => (),
            Err(e) => {
                error!("failed to join: {}", e);
                return Err(e.into());
            }
        }
        maybe_call = Some(call);
    }

    let call = maybe_call.unwrap();

    call.lock().await.enqueue_source(source);

    Ok(())
}



#[instrument]
async fn get_yt_source(text: &str) -> Result<Input, Box<dyn Error + Send + Sync>> {
    let src = match if text.starts_with("https://") {
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