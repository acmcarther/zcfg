#![feature(used)]
#[macro_use]
extern crate gflags;

#[macro_use]
extern crate lazy_static;

pub mod flags {
  define_flag!(pub third_flag: String = Some("third_flag here".to_owned()));
  define_flag!(pub another_flag: String = Some("another flag right here".to_owned()));
}
