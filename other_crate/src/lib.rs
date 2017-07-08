#![feature(used)]
#[macro_use]
extern crate gflags;

#[macro_use]
extern crate lazy_static;

pub mod FLAGS {
  define_flag!(some_flag);
}
