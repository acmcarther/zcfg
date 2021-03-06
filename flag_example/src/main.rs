#![feature(used)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate zcfg;
extern crate zcfg_flag_parser;

use std::env;
use zcfg_flag_parser::FlagParser;

define_cfg!(greeting, String, "Hello".to_owned(),
            "Defines what the greeter should say (such as \"Hello\")");
define_cfg!(multigreeting, Option<Vec<String>>, None,
            "A comma-separated set of greetings to use. Overrides `--greeting`, if set.");
define_cfg!(greeting_target, String, "World".to_owned(),
            "Defines what the greeter should say hello to (such as \"World\")");

fn main() {
  let errs = FlagParser::new().parse_from_args(env::args().skip(1));
  assert_eq!(errs, Ok(()));

  let mut greeting = greeting::CONFIG.get_value();
  if let Some(greetings) =  multigreeting::CONFIG.get_value() {
    greeting = format!("[{}]", greetings.join(","));
  }

  println!("{} {}!", greeting, greeting_target::CONFIG.get_value());
}
