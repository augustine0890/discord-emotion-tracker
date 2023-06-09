use regex::Regex;
use serenity::model::prelude::ChannelId;
use serenity::model::{channel::Channel, channel::Message};
use serenity::prelude::*;
use std::env;

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

pub fn should_ignore_user(msg: &Message) -> bool {
    let ignored_user_ids: &[&str] = &[
        "983924510220779550",  // wen
        "1026733912778625026", // corrie
        "912897330213179402",  // rosie
        "885891259053531176",  // semi
        "948825318515425280",  // sky
        "1060788078266036305", // TweetShiftBOT
                               // "623155071735037982",
    ];

    ignored_user_ids
        .iter()
        .any(|&id| id == &msg.author.id.to_string())
}

pub fn should_ignore_channel(msg: &Message) -> bool {
    let ignored_channel_ids: &[&str] = &[
        "1021958640829210674", // test server (attendance)
        "1069854617011224637", // attendance-beta-version
        "808621206718251058",  // moderator-only
        "537522976963166218",  // announcements.
        "583944383083184129",  // playdapp-sns.
        "570896878858665984",  // welcome.
        "583944743655047178",  // rules-and-admin-team.
        "920238004147204177",  // filipino.
        "585672690111610880",  // chinese.
        "585672615683686419",  // russian.
        "583934248512258059",  // japanese.
        "585672591449260032",  // vietnamese.
        "1016194558926803075", // indonesia
        "1054296641651347486", // notify
        "1021958640829210674", // attendance
    ];

    ignored_channel_ids
        .iter()
        .any(|&id| id == &msg.channel_id.to_string())
}

pub fn has_minimum_word_count(msg: &Message, min_word_count: usize) -> bool {
    msg.content.split_whitespace().count() >= min_word_count
}

pub fn should_not_ignore_guild(msg: &Message) -> bool {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return false,
    };

    let guild_ids = &[
        "537515978561683466",
        "1019782712799805440", // testing guild
    ];

    !guild_ids.iter().any(|&id| id == guild_id.to_string())
}

pub fn filter_guild(msg: &Message) -> bool {
    let guild = env::var("DISCORD_GUILD").unwrap();
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return false,
    };

    guild == guild_id.to_string()
}

pub fn remove_urls(content: &str) -> Option<String> {
    let url_pattern = Regex::new(r"(https?://[^\s]+)").unwrap();

    // Check if the entire text is a URL
    if let Some(m) = url_pattern.find(&content) {
        if m.start() == 0 && m.end() == content.len() {
            return None;
        }
    }

    Some(content.to_string())
}
