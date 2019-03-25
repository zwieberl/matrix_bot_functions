extern crate rand;

use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{HandleResult, StatelessHandler};
use matrix_bot_api::handlers::HandleResult::{ContinueHandling, StopHandling};

pub fn help_str_short() -> String {
   "!rolle X [X ..] - Rolle einen (oder mehrere) Würfel mit X Augen\n".to_string()
}

pub fn help_str() -> String {
    let mut message = "Würfle Würfel:\n".to_string();
    message += "!rolle X [X ..]\n";
    message += "mit\n";
    message += "X = irgend eine Zahl. Dies entspricht der Augenzahl des Würfels.\n";
    message += "Wenn mehrere Zahlen angegeben werden, werden mehrere Würfel gewürfelt.\n";
    message += "\nBeispiel: !rolle 6 6 => Rollt 2 Würfel mit je 6 Seiten.\n";
    message
}

fn dice_help (bot: &MatrixBot, room: &str, _cmd: &str) -> HandleResult {
    bot.send_message(&help_str(), room, MessageType::RoomNotice);
    ContinueHandling
}

pub fn dice_func (bot: &MatrixBot, room: &str, cmd: &str) -> HandleResult {
    let cmd_split = cmd.split_whitespace();

    let mut results: Vec<u32> = vec![];
    for dice in cmd_split {
        let sides = match dice.parse::<u32>() {
            Ok(x) => x,
            Err(_) => { bot.send_message(&format!("{} ist leider keine Zahl.", dice), room, MessageType::RoomNotice); return StopHandling; }
        };
        results.push((rand::random::<u32>() % sides) + 1);
    }

    if results.len() == 0 {
        dice_help(bot, room, cmd);
        return StopHandling;
    }

    if results.len() == 1 {
        bot.send_message(&format!("{}", results[0]), room, MessageType::RoomNotice);
    } else {
       // make string from results:
       let str_res : Vec<String> = results.iter().map(|x| x.to_string()).collect();
       bot.send_message(&format!("{} = {}", str_res.join(" + "), results.iter().sum::<u32>()), room, MessageType::RoomNotice);
    }
    StopHandling
}


pub fn register_handler(bot: &mut MatrixBot, prefix: &Option<&str>) {
    let mut handler = StatelessHandler::new();
    match prefix {
        Some(x) => handler.set_cmd_prefix(x),
        None => {/* Nothing */},
    }

    handler.register_handle("hilfe", dice_help);
    handler.register_handle("rolle", dice_func);
    bot.add_handler(handler);
}
