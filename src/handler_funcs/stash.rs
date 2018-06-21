use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{MessageHandler, extract_command, HandleResult};

pub fn help_str_short() -> String {
   "!{stash|del|show} [X ..] - Speichere einen (oder mehrere) Nachrichten speichern/anzeigen\n".to_string()
}

pub fn help_str() -> String {
    let mut message = "Speichere Nachrichten:\n".to_string();
    message += "!stash [Text] - speichert Text.\n";
    message += "!show [X] - Zeigt die volle Nachricht passend zur gegebenen Nummer X.\n";
    message += "    Ohne X: Zeigt alle gespeicherten Texte in Kurzform mit Nummer an.\n";
    message += "!del X - Löscht die Nachricht zur gegebenen Nummer X.\n";
    message += "\nBeispiel:\n!stash Kuchenrezept: http://mein.kochbuch.de/erdbeerkuchen\n";
    message += "!show => [0] Kuchenrezept: ...\n";
    message += "!show 0 => [0] Kuchenrezept: http://mein.kochbuch.de/erdbeerkuchen\n";
    message += "!del 0 => Löscht Kuchenrezept\n";
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
			message += &format!("Stash ist leer.");
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
    	    Err(_) => { return format!("\"{}\" ist keine Zahl.", msg); },
    	};

    	let mut message = String::new();
    	let length = self.stashes.len();
    	if length == 0 {
    		message += &format!("Stash bereits leer.");
    	} else if idx < length {
    		self.stashes.remove(idx);
    		message += &format!("Index {} entfernt.", idx);
    	} else {
    		message += &format!("Index {} falsch. Nur zwischen 0 und {} möglich.", idx, length - 1);
    	}

    	message
    }

    pub fn show(&mut self, msg: &str)  -> String {
    	if msg.is_empty() {
    		return self.list();
    	}

    	let idx: usize = match msg.parse() {
    	    Ok(x) => x,
    	    Err(_) => return format!("{} ist keine Zahl.", msg),
    	};

    	let mut message = String::new();
    	let length = self.stashes.len();
    	if length == 0 {
    		message += &format!("Stash ist leer.");
    	} else if idx < length {
    		message += &self.stashes[idx];
    	} else {
    		message += &format!("Index {} falsch. Nur zwischen 0 und {} möglich.", idx, length - 1);
    	}

    	message
    }

    pub fn stash(&mut self, msg: &str) -> String {
    	self.stashes.push(msg.to_string());
    	let length = self.stashes.len();
    	format!("Text an Index {} hinzugefügt.", length - 1)
    }
}

// Implement the trait MessageHandler, to be able to give it to our MatrixBot.
// This trait only has one function: handle_message() and will be called on each
// new (text-)message in the room the bot is in.
impl MessageHandler for StashHandler {
    fn handle_message(&mut self, bot: &MatrixBot, room: &str, message: &str) -> HandleResult {
        // extract_command() will split the message by whitespace and remove the prefix (here "!")
        // from the first entry. If the message does not start with the given prefix, None is returned.
        let command = match extract_command(message, &self.cmd_prefix) {
            Some(x) => x,
            None => return HandleResult::ContinueHandling,
        };

		let end_of_prefix = self.cmd_prefix.len() + command.len();

        // Now we have the current command (some text prefixed with our prefix !)
        // Your handler could have a HashMap with the command as the key
        // and a specific function for it (like StatelessHandler does it),
        // or you can use a simple match-statement, to act on the given command:
        let answer = match command {
          "list"  => self.list(),
          "stash" => self.stash(&message[end_of_prefix..].trim()),
          "show"  => self.show(&message[end_of_prefix..].trim()),
          "del"   => self.del(&message[end_of_prefix..].trim()),
          _ => { return HandleResult::ContinueHandling; } /* Not a known command */
        };

        bot.send_message(&answer, room, MessageType::RoomNotice);
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
