use rand;

use matrix_bot_api::{Message, ActiveBot, MatrixBot, MessageType};
use matrix_bot_api::handlers::{HandleResult, StatelessHandler};
use matrix_bot_api::handlers::HandleResult::{ContinueHandling, StopHandling};

use crate::languages::*;
use crate::tr;

pub fn help_str_short() -> String {
   tr!("!roll X [X ..] - Roll one (or more) dice with X sides").to_string() + "\n"
}

pub fn help_str() -> String {
    let mut message = tr!("Roll dice:").to_string() + "\n";
    message += tr!("!roll X [X ..]");
    message += "\n";
    message += tr!("with");
    message += "\n";
    message += tr!("X = some number. These are the number of sides of the die.");
    message += "\n";
    message += tr!("If more than one number is given, multiple dice are rolled.");
    message += "\n";
    message += "\n";
    message += tr!("Example: !roll 6 6 => Rolling 2 dice with both having 6 sides");
    message += "\n";
    message
}

fn dice_help (bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message(&help_str(), &message.room, MessageType::RoomNotice);
    ContinueHandling
}

pub fn dice_func (bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let cmd_split = cmd.split_whitespace();

    let mut results: Vec<u32> = vec![];
    for dice in cmd_split {
        let sides = match dice.parse::<u32>() {
            Ok(x) => x,
            Err(_) => { bot.send_message(&format!("{} {}", dice, tr!("is not a number.")), &message.room, MessageType::RoomNotice); return StopHandling; }
        };
        results.push((rand::random::<u32>() % sides) + 1);
    }

    if results.len() == 0 {
        dice_help(bot, &message, cmd);
        return StopHandling;
    }

    if results.len() == 1 {
        bot.send_message(&format!("{}", results[0]), &message.room, MessageType::RoomNotice);
    } else {
       // make string from results:
       let str_res : Vec<String> = results.iter().map(|x| x.to_string()).collect();
       bot.send_message(&format!("{} = {}", str_res.join(" + "), results.iter().sum::<u32>()), &message.room, MessageType::RoomNotice);
    }
    StopHandling
}


pub fn register_handler(bot: &mut MatrixBot, prefix: &Option<&str>) {
    let mut handler = StatelessHandler::new();
    match prefix {
        Some(x) => handler.set_cmd_prefix(x),
        None => {/* Nothing */},
    }

    handler.register_handle(tr!("help"), dice_help);
    handler.register_handle(tr!("roll"), dice_func);
    bot.add_handler(handler);
}
