use std::env;

use serenity::all::{GuildId, RoleId};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::guild::Guild;
use serenity::model::gateway::Ready;
use serenity::builder::CreateMessage;
use serenity::prelude::*;

static TOKEN: &str = "MTIyNDc2NDYxNjI1NzcwNDAwNw.G5onuT.qB0N6EN9Sm_xqKPtThITN18TLKSwus2aoV7Z30";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return
        };

        let mut role_ids: Vec<RoleId> = vec![];
        
        let roles = guild_id.roles(&ctx.http).await.unwrap();
        for (role_id, role) in roles {
            println!("{} {}", role_id, role.name);
            if role.name == "Verified" || role.name == "Trusted" {
                role_ids.push(role_id);
            }
        }

        let mut author_is_verified = false;
        for role_id in role_ids {
            if msg.author.has_role(&ctx.http, guild_id, role_id).await.unwrap() {
                author_is_verified = true;
                break;
            }
        }

        if msg.content == "invite" && !author_is_verified {

            //msg.reply(&ctx.http, "Hey!").await.unwrap();

            msg.author.direct_message(&ctx.http, CreateMessage::new().content("Hello")).await.unwrap();

            msg.delete(&ctx.http).await.unwrap();

            //msg.channel_id.

            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
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
    let mut client =
        Client::builder(&TOKEN, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}