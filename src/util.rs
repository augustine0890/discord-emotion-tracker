use regex::Regex;
use serenity::model::prelude::ChannelId;
use serenity::model::{channel::Channel, channel::Message};
use serenity::prelude::*;

pub async fn replace_mentions(ctx: &Context, msg: &Message) -> String {
    let mut content = msg.content.clone();

    // Replace user mentions
    for user_mention in &msg.mentions {
        let mention_text = format!("<@{}>", user_mention.id);
        let mention_text_nick = format!("<@!{}>", user_mention.id);
        content = content.replace(&mention_text, &user_mention.name);
        content = content.replace(&mention_text_nick, &user_mention.name);
    }

    // Replace channel mentions using regex
    let channel_mention_regex = Regex::new(r"<#(\d+)>").unwrap();
    let mut new_content = String::new();
    let mut last_end = 0;
    for capture in channel_mention_regex.find_iter(&content) {
        let start = capture.start();
        let end = capture.end();
        new_content.push_str(&content[last_end..start]);

        if let Some(channel_id_str) = channel_mention_regex
            .captures(&content[start..end])
            .unwrap()
            .get(1)
        {
            if let Ok(channel_id_num) = channel_id_str.as_str().parse::<u64>() {
                let channel_id = ChannelId::from(channel_id_num);
                if let Ok(channel) = channel_id.to_channel(&ctx.http).await {
                    let channel_name = match &channel {
                        Channel::Guild(channel) => channel.name.clone(),
                        Channel::Private(channel) => channel.name().to_string(),
                        _ => "".to_string(),
                    };
                    new_content.push_str(&channel_name);
                }
            }
        }

        last_end = end;
    }
    new_content.push_str(&content[last_end..]);
    content = new_content;

    // Replace role mentions
    for role_mention in &msg.mention_roles {
        if let Some(guild_id) = msg.guild_id {
            if let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await {
                if let Some(role) = guild.roles.get(role_mention) {
                    let mention_text = format!("<@&{}>", role_mention);
                    content = content.replace(&mention_text, &role.name);
                }
            }
        }
    }

    content
}
