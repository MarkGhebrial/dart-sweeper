mod config;
use config::*;

use std::env;

use serenity::all::{ChannelId, GuildId, RoleId};
use serenity::async_trait;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::channel::Message;

use serenity::model::gateway::Ready;
use serenity::prelude::*;

use regex::Regex;

static TOKEN: &str = "MTIyNDc2NDYxNjI1NzcwNDAwNw.G5onuT.qB0N6EN9Sm_xqKPtThITN18TLKSwus2aoV7Z30";

fn message_contains_invite(msg: &str) -> bool {
    let re = Regex::new(r"discord\.gg/\S*").unwrap();

    re.find(msg).is_some()
}
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return,
        };

        let config: BotConfig = get_config(guild_id);

        let mut role_ids: Vec<RoleId> = vec![];

        let roles = guild_id.roles(&ctx.http).await.unwrap();
        for (role_id, role) in roles {
            println!("{} {}", role_id, role.name);
            if config.whitelisted_roles.contains(&role.name) {
                role_ids.push(role_id);
            }
        }

        let mut author_is_verified = false;
        for role_id in role_ids {
            if msg
                .author
                .has_role(&ctx.http, guild_id, role_id)
                .await
                .unwrap()
            {
                author_is_verified = true;
                break;
            }
        }

        if !author_is_verified && message_contains_invite(&msg.content) {
            let embed = CreateEmbed::new()
                .title("Deleted message")
                .description(msg.content.clone());

            let message_to_mods = CreateMessage::new()
                .content(format!(
                    "Deleted the following message sent by @{} in #{}",
                    msg.author.name,
                    msg.channel_id.name(&ctx.http).await.unwrap()
                ))
                .embed(embed.clone());

            // Post a message in the moderator log
            if let Some(channel_id) = config.mod_log_channel_id {
                ChannelId::from(channel_id)
                    .send_message(&ctx.http, message_to_mods)
                    .await
                    .unwrap();
            }

            let message_to_author = CreateMessage::new()
                .content("Unverified members are not allowed to post invites. Your message has been deleted")
                .add_embed(embed);

            msg.author
                .direct_message(&ctx.http, message_to_author)
                .await
                .unwrap();

            msg.delete(&ctx.http).await.unwrap();
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // // Configure the client with your Discord bot token in the environment.
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
