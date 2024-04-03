use serenity::all::CommandOptionType;
use serenity::all::Permissions;
use serenity::all::{RoleId, Role};
use serenity::builder::CreateCommand;
use serenity::builder::CreateCommandOption;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {

    // let role: Role = _options[0].value.try_into().unwrap();

    "Hey, I'm alive!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("A ping command")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Role, "role", "Role to whitelist")
                .required(true),
        ).default_member_permissions(Permissions::MANAGE_MESSAGES)
}
