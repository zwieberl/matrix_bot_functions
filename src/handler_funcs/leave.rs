use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{HandleResult, StatelessHandler};
use matrix_bot_api::handlers::HandleResult::{ContinueHandling, StopHandling};


pub fn shutdown (bot: &MatrixBot, _room: &str, _cmd: &str) -> HandleResult {
	bot.shutdown();
	ContinueHandling
}

pub fn leave (bot: &MatrixBot, room: &str, _cmd: &str) -> HandleResult {
	bot.send_message("Bye!", room, MessageType::RoomNotice);
	bot.leave_room(room);
	StopHandling
}

pub fn register_handler(bot: &mut MatrixBot, prefix: &Option<&str>) {
    let mut handler = StatelessHandler::new();
    match prefix {
        Some(x) => handler.set_cmd_prefix(x),
        None => {/* Nothing */},
    }

    handler.register_handle("leave",    leave);
    handler.register_handle("shutdown", shutdown);
    bot.add_handler(handler);
}
