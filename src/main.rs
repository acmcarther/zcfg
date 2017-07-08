#![feature(used)]
extern crate third_crate;
extern crate other_crate;
extern crate gflags;


fn main() {
  print_flags();

  other_crate::FLAGS::some_flag::set_for_testing("aww sheit".to_owned());

  print_flags();

  gflags::populate_flags();

  print_flags();

  gflags::populate_flags();

  print_flags();
}

fn print_flags() {
  let some_flag = other_crate::FLAGS::some_flag::get();
  let im_very_configurable = third_crate::FLAGS::im_very_configurable::get();
  println!("Flag {} has default value: {:?}", some_flag.get_name(), some_flag.get_value());
  println!("Flag {} has default value {:?}", im_very_configurable.get_name(), im_very_configurable.get_value())
}
