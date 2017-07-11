#![feature(used)]
#[macro_use]
extern crate gflags;

#[macro_use]
extern crate lazy_static;

pub mod flags {
  define_flag!(some_flag);
  define_flag!(some_other_flag);
}
