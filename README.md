# matrix_bot_functions

Some (somewhat) usefull functions to create a matrix bot. 

It has weather-forcast, rolling dice, stashing messages.

# To test
Create a `botconfig.toml` file like this:
```
user = "botname"
password = "bot_password"
homeserver_url = "https://your.homeserver.com"
openweatherapi = "your_openweather_key"
```

and run `cargo run`. 

# To use
Simply use the crate and import the functions you want / need and build your own bot.
