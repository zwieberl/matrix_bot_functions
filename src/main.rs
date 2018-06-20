
extern crate config;
extern crate matrix_bot_api;

use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{StatelessHandler, HandleResult};
pub mod handler_funcs;
use handler_funcs::{dice, leave};

fn general_help_func (bot: &MatrixBot, room: &str, cmd: &str) -> HandleResult {
    let cmd_split : Vec<&str> = cmd.split_whitespace().collect();
    match cmd_split.len() {
      0 => {
                bot.send_message(&general_help_str(), room, MessageType::RoomNotice);
           },
      1 => {
                // return HandleResult::ContinueHandling;
                match cmd_split[0] {
                   "rolle" => { dice::dice_help(bot, room, cmd); },
                   _ => bot.send_message("Tut mir leid, diesen Befehl gibt es nicht.", room, MessageType::RoomNotice),
                }
           },
      _ => {
               bot.send_message("Tut mir leid, das geht nicht. Nutze \"!hilfe\" oder \"!hilfe BEFEHL\" für mehr Informationen.", room, MessageType::RoomNotice);
           }
    };
    HandleResult::StopHandling
}

fn general_help_str() -> String {
    let mut message = "Hallo, ich bin ein freundlicher Automat und biete diese Optionen:\n".to_string();
    message += "!hilfe          - Schreibe diese Hilfe\n";
    message += "!hilfe BEFEHL   - Gib zusätzliche Hilfe über einen der unten stehenden Befehle\n";
    message += &dice::help_str_short();
    message
}


fn main() {
    // ================== Loading credentials ==================
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let user = settings.get_str("user").unwrap();
    let password  = settings.get_str("password").unwrap();
    let homeserver_url = settings.get_str("homeserver_url").unwrap();
    // =========================================================

    // Defining Prefix
    let prefix = None; // No special prefix at the moment. Replace by Some("myprefix")

    // Defining the first handler for general help
    let mut handler = StatelessHandler::new();
    match prefix {
        Some(x) => handler.set_cmd_prefix(x),
        None => {/* Nothing */},
    }
    handler.register_handle("hilfe",    general_help_func);

    // Creating the bot
    let mut bot = MatrixBot::new(handler);

    // Registering all other handlers
    dice::register_handler(&mut bot, &prefix);
    leave::register_handler(&mut bot, &prefix);

    bot.run(&user, &password, &homeserver_url);
}
