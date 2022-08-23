use std::time::Duration;

use serenity::utils::MessageBuilder;

use super::prelude::*;

struct MentionableAdapter<'a> {
    internal: &'a dyn Mentionable,
}

impl<'a> MentionableAdapter<'a> {
    fn from(x: &'a dyn Mentionable) -> Self {
        Self { internal: x }
    }
}

impl<'a> Mentionable for MentionableAdapter<'a> {
    fn mention(&self) -> serenity::model::mention::Mention {
        self.internal.mention()
    }
}

#[command]
#[aliases("ping")]
#[only_in(guilds)]
async fn spam(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    spam_proxy(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn spam_proxy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let message = {
        let guild = msg.guild(ctx).unwrap();

        let target_name = args.message();

        /*  target order:
         * mentioned users
         * mentioned roles
         * role names
         * member names
         */

        let targets: Vec<MentionableAdapter> = if !msg.mentions.is_empty() {
            msg.mentions
                .iter()
                .map(|x| MentionableAdapter::from(x))
                .collect()
        } else if !msg.mention_roles.is_empty() {
            msg.mention_roles
                .iter()
                .map(|x| MentionableAdapter::from(x))
                .collect()
        } else if let Some(role) = guild.role_by_name(target_name) {
            vec![MentionableAdapter::from(role)]
        } else if let Some(m) = guild.member_named(target_name) {
            vec![MentionableAdapter::from(m)]
        } else {
            msg.channel_id
                .say(ctx, format!("no user named {}", target_name))
                .await?;
            return Ok(());
        };

        let mut m = MessageBuilder::new();

        for i in targets.iter() {
            m.mention(i);
        }

        m.build()
    };

    for _ in 0..10 {
        msg.channel_id.say(ctx, &message).await?;
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }

    Ok(())
}
