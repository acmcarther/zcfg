#![feature(used)]
extern crate third_crate;
extern crate other_crate;
extern crate gflags;


fn main() {
  println!("-------- First init values");
  print_flags();

  other_crate::flags::some_flag::FLAG.set_for_testing("some_flag got overridden".to_owned());

  println!("-------- Values after an override");
  print_flags();

  gflags::populate_flags();

  println!("-------- Values after populate_flags()");
  print_flags();

  gflags::populate_flags();

  println!("-------- Values after populate_flags() again");
  print_flags();
}

fn print_flags() {
  use other_crate::flags::some_flag::FLAG as FLAG_some_flag;
  use other_crate::flags::some_other_flag::FLAG as FLAG_some_other_flag;
  use third_crate::flags::third_flag::FLAG as FLAG_third_flag;
  use third_crate::flags::another_flag::FLAG as FLAG_another_flag;
  println!("Flag {} has value: {:?}", FLAG_some_flag.get_name(), FLAG_some_flag.get_value());
  println!("Flag {} has value: {:?}", FLAG_some_other_flag.get_name(), FLAG_some_other_flag.get_value());
  println!("Flag {} has value: {:?}", FLAG_third_flag.get_name(), FLAG_third_flag.get_value());
  println!("Flag {} has value: {:?}", FLAG_another_flag.get_name(), FLAG_another_flag.get_value());
}
