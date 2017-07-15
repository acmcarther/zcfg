#![feature(used)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate gconfig;
extern crate gconfig_flag_parser;

use std::env;
use gconfig::NoneableCfg;
use gconfig_flag_parser::FlagParser;

define_cfg!(greeting, String, "Hello".to_owned(),
            "Defines what the greeter should say (such as \"Hello\")");
define_cfg!(multigreeting, ::gconfig::NoneableCfg<::gconfig::CommaSeparatedCfgs<String>>, None,
            "A comma-separated set of greetings to use. Overrides `--greeting`, if set.");
define_cfg!(greeting_target, String, "World".to_owned(),
            "Defines what the greeter should say hello to (such as \"World\")");

fn main() {
  let errs = FlagParser::new().parse_from_args(env::args().skip(1));
  assert_eq!(errs, Ok(()));

  let mut greeting = greeting::VALUE.get_value();
  if let NoneableCfg(Some(greetings)) =  multigreeting::VALUE.get_value() {
    greeting = format!("[{}]", greetings.join(","));
  }

  println!("{} {}!", greeting, greeting_target::VALUE.get_value());
}
