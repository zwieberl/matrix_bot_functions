use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{Message, MessageHandler, extract_command, HandleResult};

use crate::languages::*;
use crate::tr;

pub fn help_str_short() -> String {
   tr!("!{stash|del|show} [X ..] - Stash or show one or more messages").to_string() + "\n"
}

pub fn help_str() -> String {
    let mut message = tr!("Stash message").to_string() + ":\n";
    message += tr!("!stash [text] - Stashes text");
    message += "\n";
    message += tr!("!show [X] - Shows the full message at index X");
    message += "\n";
    message += tr!("    Without X: Show all stashed messages in short with index.");
    message += "\n";
    message += tr!("!del X - Delete message at index X.");
    message += "\n";
    message += tr!("\nExample:\n!stash cake recipe: http://my.cookbook.com/applecake");
    message += "\n";
    message += tr!("!show => [0] cake recipe: ...");
    message += "\n";
    message += tr!("!show 0 => [0] cake recipe: http://my.cookbook.com/applecake");
    message += "\n";
    message += tr!("!del 0 => Deletes cake recipe");
    message += "\n";
    message
}


// Our handler wants a mutable state (here represented by a little counter-variable)
// This counter can be increased or decreased by users giving the bot a command.
pub struct StashHandler {
    stashes: Vec<String>,
    cmd_prefix: String,
}

impl StashHandler {
    pub fn new() -> StashHandler {
        StashHandler{stashes: Vec::new(), cmd_prefix: "!".to_string()}
    }

    /// With what prefix commands to the bot will start
    /// Default: "!"
    pub fn set_cmd_prefix(&mut self, prefix: &str) {
        self.cmd_prefix = prefix.to_string();
    }

    fn list(&mut self) -> String {
    	let mut message = String::new();
    	if self.stashes.len() == 0 {
			message += tr!("Stash is empty.");
    	} else {
	    	for (idx, stash) in &mut self.stashes.iter().enumerate() {
	    	    message += &format!("[{}] - {:.25}\n", idx, &stash);
	    	}
    	}
    	message
    }

    pub fn del(&mut self, msg: &str)  -> String {
    	let idx: usize = match msg.parse() {
    	    Ok(x) => x,
    	    Err(_) => { return format!("\"{}\" {}.", msg, tr!("is not a number.")); },
    	};

    	let mut message = String::new();
    	let length = self.stashes.len();
    	if length == 0 {
    		message += tr!("Stash is empty.");
    	} else if idx < length {
    		self.stashes.remove(idx);
    		message += &format!("{} {}", tr!("Deleted index"), idx);
    	} else {
    		message += tr!("Index out of range!");
    	}

    	message
    }

    pub fn show(&mut self, msg: &str)  -> String {
    	if msg.is_empty() {
    		return self.list();
    	}

    	let idx: usize = match msg.parse() {
    	    Ok(x) => x,
    	    Err(_) => return format!("{} {}", msg, tr!("is not a number.")),
    	};

    	let mut message = String::new();
    	let length = self.stashes.len();
    	if length == 0 {
    		message += tr!("Stash is empty.");
    	} else if idx < length {
    		message += &self.stashes[idx];
    	} else {
    		message += tr!("Index out of range!");
    	}

    	message
    }

    pub fn stash(&mut self, msg: &str) -> String {
    	self.stashes.push(msg.to_string());
    	let length = self.stashes.len();
    	format!("{} {}", tr!("Added text at index"), length - 1)
    }
}

// Implement the trait MessageHandler, to be able to give it to our MatrixBot.
// This trait only has one function: handle_message() and will be called on each
// new (text-)message in the room the bot is in.
impl MessageHandler for StashHandler {
    fn handle_message(&mut self, bot: &MatrixBot, message: &Message) -> HandleResult {
        // extract_command() will split the message by whitespace and remove the prefix (here "!")
        // from the first entry. If the message does not start with the given prefix, None is returned.
        let command = match extract_command(&message.body, &self.cmd_prefix) {
            Some(x) => x,
            None => return HandleResult::ContinueHandling,
        };

		let end_of_prefix = self.cmd_prefix.len() + command.len();

        // Now we have the current command (some text prefixed with our prefix !)
        // Your handler could have a HashMap with the command as the key
        // and a specific function for it (like StatelessHandler does it),
        // or you can use a simple match-statement, to act on the given command:
        let answer = match command {
          x if x == tr!("list")  => self.list(),
          x if x == tr!("stash") => self.stash(&message.body[end_of_prefix..].trim()),
          x if x == tr!("show")  => self.show(&message.body[end_of_prefix..].trim()),
          x if x == tr!("del")   => self.del(&message.body[end_of_prefix..].trim()),
          _ => { return HandleResult::ContinueHandling; } /* Not a known command */
        };

        bot.send_message(&answer, &message.room, MessageType::RoomNotice);
        HandleResult::StopHandling
    }
}


pub fn register_handler(bot: &mut MatrixBot, prefix: &Option<&str>) {
    let mut handler = StashHandler::new();
    match prefix {
        Some(x) => handler.set_cmd_prefix(x),
        None => {/* Nothing */},
    }
    bot.add_handler(handler);
}
