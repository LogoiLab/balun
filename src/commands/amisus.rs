use std::ops::Deref;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::{Config, ConfigData};

pub async fn run(_ctx: &mut Context, command: &ApplicationCommandInteraction) -> String {
    let calling_member = command
        .member
        .clone()
        .expect("failed to get command member.")
        .user
        .id;
    return get_sus_factor(calling_member.as_u64()).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("amisus")
        .description("Very suspicious command")
}

async fn get_sus_factor(user_id: &u64) -> String {
    let impostor = get_sus_impostor(user_id).await;
    let color = get_sus_color(user_id).await;
    match user_id % 6 {
        0 => return format!("<@{}> is a cute {}{} Caught in 4k on security cams:\n{}{}", user_id, color.0, impostor, color.1, SUS_CUTE),
        1 => return format!("<@{}> is a small {}{} Caught in 4k on security cams:\n{}{}", user_id, color.0, impostor, color.1, SUS_SMALL),
        2 => return format!("<@{}> is a lucky {}{} Caught in 4k on security cams:\n{}{}", user_id, color.0, impostor, color.1, SUS_LUCKY),
        3 => return format!("<@{}> is a thicc {}{} Caught in 4k on security cams:\n{}{}", user_id, color.0, impostor, color.1, SUS_THICC),
        4 => return format!("<@{}> is a {} {} Caught in 4k on security cams:\n{}{}", user_id, color.0, impostor, color.1, SUS_DEFAULT),
        5 => return format!("<@{}> is not very sus.", user_id),
        _ =>  return format!("<@{}> is an impostor!", user_id),
    }
}

async fn get_sus_impostor(user_id: &u64) -> String {
    match user_id % 10 {
        0 => return "impostor!".into(),
        _ => return "crewmate.".into(),
    }
}

async fn get_sus_color(user_id: &u64) -> (String, String) {
    match user_id % 4 {
        0 => return ("".into(), "```\n".into()),
        1 => return ("".into(), "```\n".into()),
        2 => return ("".into(), "```\n".into()),
        3 => return ("".into(), "```\n".into()),
        _ => return ("".into(), "```\n".into()),
    }
}

static SUS_CUTE: &str = r#"
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣠⣴⣶⣿⣿⣷⣶⣄⣀⣀⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⣰⣾⣿⣿⡿⢿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⢀⣾⣿⣿⡟⠁⣰⣿⣿⣿⡿⠿⠻⠿⣿⣿⣿⣿⣧⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣾⣿⣿⠏⠄⣴⣿⣿⣿⠉⠄⠄⠄⠄⠄⠈⢻⣿⣿⣇⠄⠄⠄
⠄⠄⠄⠄⢀⣠⣼⣿⣿⡏⠄⢠⣿⣿⣿⠇⠄⠄⠄⠄⠄⠄⠄⠈⣿⣿⣿⡀⠄⠄
⠄⠄⠄⣰⣿⣿⣿⣿⣿⡇⠄⢸⣿⣿⣿⡀⠄⠄⠄⠄⠄⠄⠄⠄⣿⣿⣿⡇⠄⠄
⠄⠄⢰⣿⣿⡿⣿⣿⣿⡇⠄⠘⣿⣿⣿⣧⠄⠄⠄⠄⠄⠄⢀⣸⣿⣿⣿⠁⠄⠄
⠄⠄⣿⣿⣿⠁⣿⣿⣿⡇⠄⠄⠻⣿⣿⣿⣷⣶⣶⣶⣶⣶⣿⣿⣿⣿⠃⠄⠄⠄
⠄⢰⣿⣿⡇⠄⣿⣿⣿⠄⠄⠄⠄⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠄⠄⠄⠄
⠄⢸⣿⣿⡇⠄⣿⣿⣿⠄⠄⠄⠄⠄⠄⠄⠉⠛⠛⠛⠉⢉⣿⣿⠄⠄⠄⠄⠄⠄
⠄⢸⣿⣿⣇⠄⣿⣿⣿⠄⠄⠄⠄⠄⢀⣤⣤⣤⡀⠄⠄⢸⣿⣿⣿⣷⣦⠄⠄⠄
⠄⠄⢻⣿⣿⣶⣿⣿⣿⠄⠄⠄⠄⠄⠈⠻⣿⣿⣿⣦⡀⠄⠉⠉⠻⣿⣿⡇⠄⠄
⠄⠄⠄⠛⠿⣿⣿⣿⣿⣷⣤⡀⠄⠄⠄⠄⠈⠹⣿⣿⣇⣀⠄⣠⣾⣿⣿⡇⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠹⣿⣿⣿⣿⣦⣤⣤⣤⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⡟⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠉⠻⢿⣿⣿⣿⣿⣿⣿⠿⠋⠉⠛⠋⠉⠉⠁⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⠉⠉⠉⠁⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
```
"#;
static SUS_SMALL: &str = r#"
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣠⣤⣤⣤⣤⣤⣶⣦⣤⣄⡀⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⢀⣴⣿⡿⠛⠉⠙⠛⠛⠛⠛⠻⢿⣿⣷⣤⡀⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⣼⣿⠋⠄⠄⠄⠄⠄⠄⠄⢀⣀⣀⠈⢻⣿⣿⡄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣸⣿⡏⠄⠄⠄⣠⣶⣾⣿⣿⣿⠿⠿⠿⢿⣿⣿⣿⣄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠁⠄⠄⢰⣿⣿⣯⠁⠄⠄⠄⠄⠄⠄⠄⠈⠙⢿⣷⡄⠄
⠄⠄⣀⣤⣴⣶⣶⣿⡟⠄⠄⠄⢸⣿⣿⣿⣆⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣿⣷⠄
⠄⢰⣿⡟⠋⠉⣹⣿⡇⠄⠄⠄⠘⣿⣿⣿⣿⣷⣦⣤⣤⣤⣶⣶⣶⣶⣿⣿⣿⠄
⠄⢸⣿⡇⠄⠄⣿⣿⡇⠄⠄⠄⠄⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠄
⠄⣸⣿⡇⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠉⠻⠿⣿⣿⣿⣿⡿⠿⠿⠛⢻⣿⡇⠄⠄
⠄⠸⣿⣧⡀⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣿⣿⠃⠄⠄
⠄⠄⠛⢿⣿⣿⣿⣿⣇⠄⠄⠄⠄⠄⣰⣿⣿⣷⣶⣶⣶⣶⠶⠄⢠⣿⣿⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠄⠄⠄⠄⠄⣿⣿⡇⠄⣽⣿⡏⠁⠄⠄⢸⣿⡇⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠄⠄⠄⠄⠄⣿⣿⡇⠄⢹⣿⡆⠄⠄⠄⣸⣿⠇⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⢿⣿⣦⣄⣀⣠⣴⣿⣿⠁⠄⠈⠻⣿⣿⣿⣿⡿⠏⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠈⠛⠻⠿⠿⠿⠿⠋⠁⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
```
"#;
static SUS_LUCKY: &str = r#"
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢀⣀⣀⣴⣆⣠⣤⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⣻⣿⣯⣘⠹⣧⣤⡀⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠛⠿⢿⣿⣷⣾⣯⠉⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣾⣿⠜⣿⡍⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣸⣿⠁⠄⠘⣿⣆⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢠⣿⡟⠃⡄⠄⠘⢿⣆⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣾⣿⣁⣋⣈⠄⣤⣮⣿⣧⡀⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣠⣤⣤⣤⣤⣤⣶⣦⣤⣄⡀⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⢀⣴⣿⡿⠛⠉⠙⠛⠛⠛⠛⠻⢿⣿⣷⣤⡀⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⣼⣿⠋⠄⠄⠄⠄⠄⠄⠄⢀⣀⣀⠈⢻⣿⣿⡄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣸⣿⡏⠄⠄⠄⣠⣶⣾⣿⣿⣿⠿⠿⠿⢿⣿⣿⣿⣄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠁⠄⠄⢰⣿⣿⣯⠁⠄⠄⠄⠄⠄⠄⠄⠈⠙⢿⣷⡄⠄
⠄⠄⣀⣤⣴⣶⣶⣿⡟⠄⠄⠄⢸⣿⣿⣿⣆⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣿⣷⠄
⠄⢰⣿⡟⠋⠉⣹⣿⡇⠄⠄⠄⠘⣿⣿⣿⣿⣷⣦⣤⣤⣤⣶⣶⣶⣶⣿⣿⣿⠄
⠄⢸⣿⡇⠄⠄⣿⣿⡇⠄⠄⠄⠄⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠄
⠄⣸⣿⡇⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠉⠻⠿⣿⣿⣿⣿⡿⠿⠿⠛⢻⣿⡇⠄⠄
⠄⣿⣿⠁⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢸⣿⣧⠄⠄
⠄⣿⣿⠄⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢸⣿⣿⠄⠄
⠄⣿⣿⠄⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢸⣿⣿⠄⠄
⠄⢿⣿⡆⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢸⣿⡇⠄⠄
⠄⠸⣿⣧⡀⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣿⣿⠃⠄⠄
⠄⠄⠛⢿⣿⣿⣿⣿⣇⠄⠄⠄⠄⠄⣰⣿⣿⣷⣶⣶⣶⣶⠶⠄⢠⣿⣿⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠄⠄⠄⠄⠄⣿⣿⡇⠄⣽⣿⡏⠁⠄⠄⢸⣿⡇⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠄⠄⠄⠄⠄⣿⣿⡇⠄⢹⣿⡆⠄⠄⠄⣸⣿⠇⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⢿⣿⣦⣄⣀⣠⣴⣿⣿⠁⠄⠈⠻⣿⣿⣿⣿⡿⠏⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠈⠛⠻⠿⠿⠿⠿⠋⠁⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
```
"#;
static SUS_THICC: &str = r#"
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣀⣀⣐⡀⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⢠⠄⣠⣶⣿⣿⣿⠿⠿⣛⣂⣀⣀⡒⠶⣶⣤⣤⣬⣀⡀⠄⢀⠄⠄⠄⠄⠄⠄⠄
⠄⠄⢀⣾⣿⣿⣿⡟⢡⢾⣿⣿⣿⣿⣿⣿⣶⣌⠻⣿⣿⣿⣿⣷⣦⣄⡀⠄⠄⠄⠄⠄
⠄⠄⣈⣉⡛⣿⣿⣿⡌⢇⢻⣿⣿⣿⣿⣿⠿⠛⣡⣿⣿⣿⣿⣿⣿⣿⣿⣦⣄⠄⠄⠄
⠄⠺⠟⣉⣴⡿⠛⣩⣾⣎⠳⠿⠛⣋⣩⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣆⠄⠄
⠄⠄⠄⠘⢋⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠄
⠄⠄⢀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠄
⠄⠄⠄⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⣀
⠄⠄⠄⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠘⠛
⠄⠄⠄⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠋⣀⣀⣠⣤
⠄⠄⣀⣀⡙⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⢛⣩⠤⠾⠄⠛⠋⠉⢉
⠄⠺⠿⠛⠛⠃⠄⠉⠙⠛⠛⠻⠿⠿⠿⠟⠛⠛⠛⠉⠁⠄⠄⣀⣀⣠⣤⣠⣴⣶⣼⣿
```
"#;
static SUS_DEFAULT: &str = r#"
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣠⣤⣤⣤⣤⣤⣶⣦⣤⣄⡀⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⢀⣴⣿⡿⠛⠉⠙⠛⠛⠛⠛⠻⢿⣿⣷⣤⡀⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠄⣼⣿⠋⠄⠄⠄⠄⠄⠄⠄⢀⣀⣀⠈⢻⣿⣿⡄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣸⣿⡏⠄⠄⠄⣠⣶⣾⣿⣿⣿⠿⠿⠿⢿⣿⣿⣿⣄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠁⠄⠄⢰⣿⣿⣯⠁⠄⠄⠄⠄⠄⠄⠄⠈⠙⢿⣷⡄⠄
⠄⠄⣀⣤⣴⣶⣶⣿⡟⠄⠄⠄⢸⣿⣿⣿⣆⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣿⣷⠄
⠄⢰⣿⡟⠋⠉⣹⣿⡇⠄⠄⠄⠘⣿⣿⣿⣿⣷⣦⣤⣤⣤⣶⣶⣶⣶⣿⣿⣿⠄
⠄⢸⣿⡇⠄⠄⣿⣿⡇⠄⠄⠄⠄⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠄
⠄⣸⣿⡇⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠉⠻⠿⣿⣿⣿⣿⡿⠿⠿⠛⢻⣿⡇⠄⠄
⠄⣿⣿⠁⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢸⣿⣧⠄⠄
⠄⣿⣿⠄⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢸⣿⣿⠄⠄
⠄⣿⣿⠄⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢸⣿⣿⠄⠄
⠄⢿⣿⡆⠄⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢸⣿⡇⠄⠄
⠄⠸⣿⣧⡀⠄⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⣿⣿⠃⠄⠄
⠄⠄⠛⢿⣿⣿⣿⣿⣇⠄⠄⠄⠄⠄⣰⣿⣿⣷⣶⣶⣶⣶⠶⠄⢠⣿⣿⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠄⠄⠄⠄⠄⣿⣿⡇⠄⣽⣿⡏⠁⠄⠄⢸⣿⡇⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⣿⠄⠄⠄⠄⠄⣿⣿⡇⠄⢹⣿⡆⠄⠄⠄⣸⣿⠇⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⢿⣿⣦⣄⣀⣠⣴⣿⣿⠁⠄⠈⠻⣿⣿⣿⣿⡿⠏⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠈⠛⠻⠿⠿⠿⠿⠋⠁⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
```
"#;
