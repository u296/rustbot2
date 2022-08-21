use super::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

#[command]
#[aliases("pick", "random")]
async fn select_random(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    select_random_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn select_random_proxy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return Ok(())
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