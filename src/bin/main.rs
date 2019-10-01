use matrix_bot_api::{ActiveBot, MatrixBot, MessageType, Message};
use matrix_bot_api::handlers::{StatelessHandler, HandleResult};
use matrix_bot_functions::{dice, leave, stash, weather};
use config;

use matrix_bot_functions::languages::*;
use matrix_bot_functions::tr;

fn general_help_func (bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let cmd_split : Vec<&str> = cmd.split_whitespace().collect();
    match cmd_split.len() {
      0 => {
                bot.send_message(&general_help_str(), &message.room, MessageType::RoomNotice);
           },
      1 => {
                // return HandleResult::ContinueHandling;
                match cmd_split[0] {
                   k if k == tr!("roll") => { bot.send_message(&dice::help_str(), &message.room, MessageType::RoomNotice) },
                   k if k == tr!("stash") => { bot.send_message(&stash::help_str(), &message.room, MessageType::RoomNotice) },
                   k if k == tr!("weather") => { bot.send_message(&weather::help_str(), &message.room, MessageType::RoomNotice) },
                   _ => bot.send_message(tr!("Sorry, unknown command"), &message.room, MessageType::RoomNotice),
                }
           },
      _ => {
               bot.send_message(tr!("Sorry, that is not possible. Please use \"!help\" or \"!help COMMAND\" for more information."), &message.room, MessageType::RoomNotice);
           }
    };
    HandleResult::StopHandling
}

fn general_help_str() -> String {
    let mut message = tr!("Hi, I'm a friendly robot and provide these options:").to_string();
    message += "\n";
    message += tr!("!help         - Print this help");
    message += "\n";
    message += tr!("!help COMMAND - Print add. help for one of the commands below");
    message += "\n";
    message += &dice::help_str_short();
    message += &stash::help_str_short();
    message += &weather::help_str_short();
    message
}


fn main() {
    // ================== Loading credentials ==================
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("botconfig")).unwrap();

    let user = settings.get_str("user").unwrap();
    let password  = settings.get_str("password").unwrap();
    let homeserver_url = settings.get_str("homeserver_url").unwrap();
    let openweatherapi = settings.get_str("openweatherapi").unwrap();
    SELECTED_LANG.set(settings.get_str("language").unwrap()).unwrap();
    // =========================================================

    // Defining Prefix - default: "!"
    let prefix = None; // No special prefix at the moment. Replace by Some("myprefix")

    // Defining the first handler for general help
    let mut handler = StatelessHandler::new();
    match prefix {
        Some(x) => handler.set_cmd_prefix(x),
        None => {/* Nothing */},
    }
    handler.register_handle(tr!("help"),    general_help_func);

    // Creating the bot
    let mut bot = MatrixBot::new(handler);

    // Registering all other handlers
    dice::register_handler(&mut bot, &prefix);
    leave::register_handler(&mut bot, &prefix);
    stash::register_handler(&mut bot, &prefix);
    weather::register_handler(&mut bot, &prefix, openweatherapi);

    bot.run(&user, &password, &homeserver_url);
}
