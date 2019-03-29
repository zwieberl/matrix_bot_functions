use std::collections::HashMap;
use lazy_static::lazy_static;

use once_cell::sync::OnceCell;

mod de;
mod en;

pub static SELECTED_LANG: OnceCell<String> = OnceCell::INIT;

lazy_static! {
    pub static ref TR_MAP: HashMap<&'static str, &'static str> = {
        let selected_lang = match SELECTED_LANG.get().expect("No language set!").to_uppercase().as_ref() {
            "EN" => &self::en::EN,
            "DE" => &self::de::DE,
            x => panic!("Unknown language {}", x),
        };

        let mut m = HashMap::new();
        for (key, val) in *selected_lang {
          m.insert(*key, *val);
        }
        m
    };
}

#[macro_export]
macro_rules! tr {
    ( $x:expr ) => {
        {
            if TR_MAP.contains_key( &$x) {
              TR_MAP[$x]
            } else {
              $x
            }
        }
    };
}