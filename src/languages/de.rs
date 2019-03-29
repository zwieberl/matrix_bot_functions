
pub const DE: &[(&str, &str)] = &[
  // main.rs
  ("roll", "rolle",),
  ("stash", "stash"),
  ("weather", "wetter"),
  ("Sorry, unknown command", "Tut mir leid, diesen Befehl gibt es nicht."),
  ("help", "hilfe"),
  ("Sorry, that is not possible. Please use \"!help\" or \"!help COMMAND\" for more information.", "Tut mir leid, das geht nicht. Nutze \"!hilfe\" oder \"!hilfe BEFEHL\" für mehr Informationen."),
  ("Hi, I'm a friendly robot and provide these options:", "Hallo, ich bin ein freundlicher Automat und biete diese Optionen:"),
  ("!help         - Print this help",                               "!hilfe          - Schreibe diese Hilfe"),
  ("!help COMMAND - Print add. help for one of the commands below", "!hilfe BEFEHL   - Gib zusätzliche Hilfe über einen der unten stehenden Befehle"),
  // dice.rs
  ("is not a number.", "ist leider keine Zahl."),
  ("!roll X [X ..] - Roll one (or more) dice with X sides", "!rolle X [X ..] - Rolle einen (oder mehrere) Würfel mit X Augen"),
  ("Roll dice:", "Würfle Würfel:"),
  ("!roll X [X ..]", "!rolle X [X ..]"),
  ("with", "mit"),
  ("X = some number. These are the number of sides of the die.", "X = irgend eine Zahl. Dies entspricht der Augenzahl des Würfels."),
  ("If more than one number is given, multiple dice are rolled.", "Wenn mehrere Zahlen angegeben werden, werden mehrere Würfel gewürfelt."),
  ("Example: !roll 6 6 => Rolling 2 dice with both having 6 sides", "Beispiel: !rolle 6 6 => Rollt 2 Würfel mit je 6 Seiten."),
  // weather.rs
  ("Weather for", "Wetter für"),
  ("!weather X - Show weather forcast for city X", "!wetter X - Zeigt Wetterbericht für Stadt X an"),
  ("Forecast", "Wetterbericht"),
  ("!weather CITY", "!wetter STADT"),
  // stash.rs
  ("!{stash|del|show} [X ..] - Stash or show one or more messages", "!{stash|del|show} [X ..] - Eine (oder mehrere) Nachrichten speichern/anzeigen"),
  ("Stash message", "Speichere Nachrichten"),
  ("!stash [text] - Stashes text", "!stash [Text] - speichert Text."),
  ("!show [X] - Shows the full message at index X", "!show [X] - Zeigt die volle Nachricht passend zur gegebenen Nummer X."),
  ("    Without X: Show all stashed messages in short with index.", "    Ohne X: Zeigt alle gespeicherten Texte in Kurzform mit Nummer an."),
  ("!del X - Delete message at index X.", "!del X - Löscht die Nachricht zur gegebenen Nummer X."),
  ("\nExample:\n!stash cake recipe: http://my.cookbook.com/applecake", "\nBeispiel:\n!stash Kuchenrezept: http://mein.kochbuch.de/erdbeerkuchen"),
  ("!show => [0] cake recipe: ...", "!show => [0] Kuchenrezept: ..."),
  ("!show 0 => [0] cake recipe: http://my.cookbook.com/applecake", "!show 0 => [0] Kuchenrezept: http://mein.kochbuch.de/erdbeerkuchen"),
  ("!del 0 => Deletes cake recipe", "!del 0 => Löscht Kuchenrezept"),
  ("Stash is empty.", "Stash ist leer."),
  ("list", "list"),
  ("stash", "stash"),
  ("show", "show"),
  ("del", "del"),
  ("Deleted index", "Entferne Index"),
  ("Index out of range!", "Index nicht im zulässigen Bereich!"),
  ("Added text at index", "Text hinzugefügt bei Index")

];
 