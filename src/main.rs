#![feature(used)]
extern crate third_crate;
extern crate gflags;


fn main() {
  println!("heres a value: {:?}", third_crate::FLAG_im_very_configurable().get_value());
  gflags::populate_flags();

  println!("heres a value: {:?}", third_crate::FLAG_im_very_configurable().get_value());
}
