mod config;
use config::*;

mod commands;
use commands::*;

use std::env;

use serenity::all::{ChannelId, Interaction};
use serenity::async_trait;
use serenity::builder::{
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};
use serenity::model::channel::Message;

use serenity::model::gateway::Ready;
use serenity::prelude::*;

use regex::Regex;

fn message_contains_invite(msg: &str) -> bool {
    let re = Regex::new(r"discord\.gg/\S*").unwrap();

    re.find(msg).is_some()
}

async fn handle_message_with_invite(
    ctx: &Context,
    msg: &Message,
    config: &BotConfig,
) -> serenity::Result<()> {
    // Create an embed whose contents are that of the message that's being deleted
    let embed = CreateEmbed::new()
        .title("Deleted message")
        .description(msg.content.clone());

    // Delete the message
    msg.delete(&ctx.http).await?;

    // Compose the message for the moderator log
    let message_to_mods = CreateMessage::new()
        .content(format!(
            "Deleted the following message sent by @{} in #{}",
            msg.author.tag(),
            msg.channel_id.name(&ctx.http).await?
        ))
        .embed(embed.clone());

    // Post the message in the moderator log
    if let Some(channel_id) = config.mod_log_channel_id {
        // We ignore any failures to post in the mod log
        let _result = ChannelId::from(channel_id)
            .send_message(&ctx.http, message_to_mods)
            .await;
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
        .await?;

    serenity::Result::Ok(())
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

        let mut author_is_verified = false;
        for role_id in &config.whitelisted_roles {
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
            if let Err(e) = handle_message_with_invite(&ctx, &msg, &config).await {
                println!("Encountered an error while processing a message with an invite: {e:#?}");
            };
        }
    }

    // Handle slash commands
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "whitelist" => Some(commands::whitelist_role(
                    &command.data.options(),
                    &command.guild_id.unwrap(),
                )),
                "unwhitelist" => Some(commands::unwhitelist_role(
                    &command.data.options(),
                    &command.guild_id.unwrap(),
                )),
                "setmodchannel" => Some(
                    commands::set_mod_channel(
                        &ctx,
                        &command.data.options(),
                        &command.guild_id.unwrap(),
                    )
                    .await,
                ),
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

        // Register the bot's slash commands
        register_commands(&ctx.http).await;
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
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
