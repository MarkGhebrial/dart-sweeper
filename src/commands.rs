use serenity::all::CommandOptionType;
use serenity::all::Permissions;
use serenity::all::GuildId;
use serenity::builder::CreateCommand;
use serenity::builder::CreateCommandOption;
use serenity::model::application::{ResolvedOption, ResolvedValue};

use crate::{get_config, write_config};

pub fn run(options: &[ResolvedOption], guild: &GuildId) -> String {

    let role = if let Some(ResolvedOption {
        name: _name,
        value: ResolvedValue::Role(role),
        ..
    }) = options.first() {
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

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("A ping command")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Role, "role", "Role to whitelist")
                .required(true),
        ).default_member_permissions(Permissions::MANAGE_MESSAGES)
}
