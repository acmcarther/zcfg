#![feature(used)]
#[macro_use]
extern crate gflags;

#[macro_use]
extern crate lazy_static;

pub mod flags {
  define_flag!(pub some_flag: String);
  define_flag!(pub some_other_flag: String);
}
