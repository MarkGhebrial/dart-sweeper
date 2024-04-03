mod config;
use config::*;

mod commands;

use std::env;

use serenity::all::{ChannelId, GuildId, Interaction, RoleId, UserId};
use serenity::builder::{
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};
use serenity::model::channel::Message;
use serenity::{async_trait, Result};

use serenity::model::gateway::Ready;
use serenity::prelude::*;

use regex::Regex;

static TOKEN: &str = "MTIyNDc2NDYxNjI1NzcwNDAwNw.G5onuT.qB0N6EN9Sm_xqKPtThITN18TLKSwus2aoV7Z30";

fn message_contains_invite(msg: &str) -> bool {
    let re = Regex::new(r"discord\.gg/\S*").unwrap();

    re.find(msg).is_some()
}

fn handle_result(r: Result<()>) {}

async fn handle_message_with_invite(ctx: &Context, msg: &Message, config: &BotConfig) {
    // Create an embed whose contents are that of the message that's being deleted
    let embed = CreateEmbed::new()
        .title("Deleted message")
        .description(msg.content.clone());

    // Delete the message
    msg.delete(&ctx.http).await.unwrap();

    // Compose the message for the moderator log
    let message_to_mods = CreateMessage::new()
        .content(format!(
            "Deleted the following message sent by @{} in #{}",
            msg.author.name,
            msg.channel_id.name(&ctx.http).await.unwrap()
        ))
        .embed(embed.clone());

    // Post the message in the moderator log
    if let Some(channel_id) = config.mod_log_channel_id {
        ChannelId::from(channel_id)
            .send_message(&ctx.http, message_to_mods)
            .await
            .unwrap();
    }

    // Compose a message to the author of the deleted message
    let message_to_author = CreateMessage::new()
        .content(
            "Unverified members are not allowed to post invites. Your message has been deleted",
        )
        .add_embed(embed);

    // DM that message to the author of the deleted message
    msg.author
        .direct_message(&ctx.http, message_to_author)
        .await
        .unwrap();
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return,
        };

        let config: BotConfig = get_config(&guild_id);

        let mut role_ids: Vec<RoleId> = vec![];

        let roles = guild_id.roles(&ctx.http).await.unwrap();
        for (role_id, role) in roles {
            println!("{} {}", role_id, role.name);
            if config.whitelisted_roles.contains(&role_id) {
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

        // let mut author_is_mod = false;
        // for role_id in mod_role_ids {
        //     if user has the role:
        //         author_is_mod = true;
        // }

        if !author_is_verified && message_contains_invite(&msg.content) {
            handle_message_with_invite(&ctx, &msg, &config).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild in ready.guilds {
            let commands = guild
                .id
                .set_commands(&ctx.http, vec![commands::register()])
                .await;

            println!("Created the following commands in guild {guild:?}: {commands:?}");
        }
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
