extern crate reqwest;
extern crate serde_json;
extern crate openweathermap_api;
extern crate chrono;

use self::openweathermap_api::{OWMResponse};
use std::collections::HashMap;

use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{Message, MessageHandler, extract_command, HandleResult};
use self::chrono::{DateTime, NaiveDateTime, Utc, Local};


pub fn help_str_short() -> String {
   "!wetter X - Zeigt Wetterbericht für Stadt X an\n".to_string()
}

pub fn help_str() -> String {
    let mut message = "Wetterbericht:\n".to_string();
    message += "!wetter STADT\n";
    message
}


// Our handler wants a mutable state (here represented by a little counter-variable)
// This counter can be increased or decreased by users giving the bot a command.
pub struct WeatherHandler {
    symbols: HashMap<&'static str, &'static str>,
    cmd_prefix: String,
    apikey: String,
}

impl WeatherHandler {
    pub fn new(apikey: String) -> WeatherHandler {
    	let mut syms = HashMap::new();
        syms.insert("01d", "☀️"); // clear sky day (sun)
        syms.insert("02d", "⛅"); // sun behind cloud
        syms.insert("03d", "☁️"); // cloud
        syms.insert("04d", "☁️"); // cloud
        syms.insert("09d", "🌧️"); // cloud rain
        syms.insert("10d", "🌦️"); // light rain
        syms.insert("11d", "⛈️"); // thunder cloud rain
        syms.insert("13d", "🌨️"); // cloud with snow
        syms.insert("50d", "🌫️"); // fog (day)

        syms.insert("01n", "🌔"); // clear sky night (moon)
        syms.insert("02n", "⛅"); // sun behind cloud
        syms.insert("03n", "☁️"); // cloud
        syms.insert("04n", "☁️"); // cloud
        syms.insert("09n", "🌧️"); // cloud rain
        syms.insert("10n", "🌦️"); // light rain
        syms.insert("11n", "⛈️"); // thunder cloud rain
        syms.insert("13n", "🌨️"); // cloud with snow
        syms.insert("50n", "🌫️"); // fog (day)
        WeatherHandler{symbols: syms, cmd_prefix: "!".to_string(), apikey: apikey}
    }

    /// With what prefix commands to the bot will start
    /// Default: "!"
    pub fn set_cmd_prefix(&mut self, prefix: &str) {
        self.cmd_prefix = prefix.to_string();
    }

    pub fn get_data(&self, url: &str) -> Result<String, reqwest::Error> {
        let mut message = String::new();
		let mut data = reqwest::get(url)?;
        let resp : OWMResponse = match data.json() {
            Ok(resp) => resp,
            Err(_) => { return Ok(format!("{:?}", data.status())); }
        };
        message += &format!("Wetter für {} ({}):\n", resp.city.name, resp.city.country);
        for item in resp.list {
            let utc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(item.dt, 0), Utc);
            let date = utc.with_timezone(&Local);
            let symbol = match self.symbols.get::<str>(&item.weather[0].icon) {
                Some(sym) => sym,
                None => "",
            };
            // message += &format!("{}: {:3.1} °C {} ({})\n", date.format("%d-%m %H:%M").to_string(), item.main.temp, symbol, item.weather[0].description);
            let temp = format!("{:.1}", item.main.temp);
            message += &format!("{}: {:>4} °C {} ({})\n", date.format("%a %H:%M").to_string(), temp, symbol, item.weather[0].description);
        }
        Ok(message)
    }

    pub fn weather(&self, location: &str) -> String {
    	match self.get_data(&format!("http://api.openweathermap.org/data/2.5/forecast?q={}&units=metric&cnt=12&lang=de&appid={}", location, self.apikey)) {
            Ok(msg) => msg,
            Err(msg) => format!("{:?}", msg),
        }
    }
}

// Implement the trait MessageHandler, to be able to give it to our MatrixBot.
// This trait only has one function: handle_message() and will be called on each
// new (text-)message in the room the bot is in.
impl MessageHandler for WeatherHandler {
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
          "wetter" | "Wetter" => self.weather(&message.body[end_of_prefix..].trim()),
          _ => { return HandleResult::ContinueHandling; } /* Not a known command */
        };

        bot.send_message(&answer, &message.room, MessageType::TextMessage);
        HandleResult::StopHandling
    }
}


pub fn register_handler(bot: &mut MatrixBot, prefix: &Option<&str>, apikey: String) {
    let mut handler = WeatherHandler::new(apikey);
    match prefix {
        Some(x) => handler.set_cmd_prefix(x),
        None => {/* Nothing */},
    }
    bot.add_handler(handler);
}
