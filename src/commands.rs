use serenity::all::ChannelId;
use serenity::all::Command;
use serenity::all::CommandOptionType;
use serenity::all::Context;
use serenity::all::GuildId;
use serenity::all::Permissions;
use serenity::builder::CreateCommand;
use serenity::builder::CreateCommandOption;
use serenity::model::application::{ResolvedOption, ResolvedValue};
use serenity::prelude::CacheHttp;

use crate::{get_config, write_config};

pub fn whitelist_role(options: &[ResolvedOption], guild: &GuildId) -> String {
    let role = if let Some(ResolvedOption {
        name: _name,
        value: ResolvedValue::Role(role),
        ..
    }) = options.first()
    {
        role
    } else {
        return "Please provide a valid role.".to_string();
    };

    // Update the configuration
    let mut config = get_config(&guild);
    config.whitelisted_roles.push(role.id);
    write_config(&guild, &config);

    format!("Role {} can now post invites.", role.name)
}

pub fn unwhitelist_role(options: &[ResolvedOption], guild: &GuildId) -> String {
    let role = if let Some(ResolvedOption {
        name: _name,
        value: ResolvedValue::Role(role),
        ..
    }) = options.first()
    {
        role
    } else {
        return "Please provide a valid role.".to_string();
    };

    let mut config = get_config(&guild);
    config.whitelisted_roles = config
        .whitelisted_roles
        .into_iter()
        .filter(|entry| entry != &role.id) // Remove the role from the list
        .collect();
    write_config(&guild, &config);

    format!("Role {} can no longer post invites.", role.name)
}

//                   Rust gets cranky if this lifetime is omitted ----v
pub async fn set_mod_channel(
    ctx: &Context,
    options: &[ResolvedOption<'_>],
    guild: &GuildId,
) -> String {
    let channel = if let Some(ResolvedOption {
        name: _name,
        value: ResolvedValue::Channel(channel),
        ..
    }) = options.first()
    {
        channel
    } else {
        return "Please provide a valid channel".to_string();
    };

    let channel_id: ChannelId = channel.id.into();

    let mut config = get_config(&guild);
    config.mod_log_channel_id = Some(channel_id.into());
    write_config(&guild, &config);

    if let Err(_e) = channel_id
        .say(&ctx.http, "Bot actions will now be logged here")
        .await
    {
        return format!(
            "Looks like the bot does not have permission to access #{}.",
            channel.name.as_ref().unwrap()
        );
    }

    format!(
        "Bot actions will now be logged to channel #{}.",
        channel.name.as_ref().unwrap()
    )
}

pub async fn register_commands(cache_http: impl CacheHttp) {
    let whitelist_role_command = CreateCommand::new("whitelist")
        .description("Allow a role to send invites")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Role, "role", "Role to add to whitelist")
                .required(true),
        )
        .default_member_permissions(Permissions::MANAGE_MESSAGES);

    Command::create_global_command(&cache_http, whitelist_role_command)
        .await
        .unwrap();

    //------------------------------------------------------------------------------------
    let unwhitelist_role_command = CreateCommand::new("unwhitelist")
        .description("Disallow a role from sending invites")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Role,
                "role",
                "Role to remove from whitelist",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::MANAGE_MESSAGES);

    Command::create_global_command(&cache_http, unwhitelist_role_command)
        .await
        .unwrap();

    //------------------------------------------------------------------------------------
    let set_mod_channel_command = CreateCommand::new("setmodchannel")
        .description("Choose which channel to log the bot's actions to")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Channel, "channel", "Channel to set")
                .required(true),
        )
        .default_member_permissions(Permissions::MANAGE_MESSAGES);

    Command::create_global_command(&cache_http, set_mod_channel_command)
        .await
        .unwrap();
}
