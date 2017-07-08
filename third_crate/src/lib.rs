#![feature(used)]
#[macro_use]
extern crate gflags;

#[macro_use]
extern crate lazy_static;

pub mod FLAGS {
  define_flag!(im_very_configurable, Some("yeah boi".to_owned()));
}
