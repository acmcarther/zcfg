#![feature(used)]
#[macro_use]
extern crate gflags;

#[macro_use]
extern crate lazy_static;

pub mod flags {
  define_flag!(third_flag, Some("third_flag here".to_owned()));
  define_flag!(another_flag, Some("another flag right here".to_owned()));
}
