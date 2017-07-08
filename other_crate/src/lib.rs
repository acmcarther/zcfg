#![feature(used)]

#[macro_use]
extern crate lazy_static;
extern crate libc;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::cell::RefCell;

#[allow(dead_code)]
#[no_mangle]
extern "C" fn enqueue_static_flag_init() {
  STATIC_FLAG_INIT_FNS.lock()
    .unwrap()
    .push(push_static_flag)
}

fn push_static_flag(flag_vec: &mut Vec<Arc<Mutex<FlagPrototype>>>) {
  flag_vec.push(FLAG_test.clone())
}

#[used]
#[link_section = ".init_array"]
#[allow(dead_code)]
static INIT_ARRAY: [extern "C" fn(); 1] = [enqueue_static_flag_init];

lazy_static! {
  static ref STATIC_FLAG_INIT_FNS: Mutex<Vec<fn(&mut Vec<Arc<Mutex<FlagPrototype>>>)>> = {
    Mutex::new(Vec::new())
  };

  static ref FLAG_test: Arc<Mutex<FlagPrototype>> = {
    Arc::new(Mutex::new(FlagPrototype::new("test".to_owned())))
  };
}



struct FlagPrototype {
  name: String,
  value: Option<String>,
}

impl FlagPrototype {
  pub fn new(name: String) -> FlagPrototype {
    FlagPrototype {
      name: name,
      value: None,
    }
  }

  pub fn set_value(&mut self, v: String) {
    self.value = Some(v)
  }
}

fn populate_flags() {
  let mut all_flags = Vec::new();
  {
    for f in STATIC_FLAG_INIT_FNS.lock().unwrap().iter() {
      f(&mut all_flags)
    }
  }

  for flag in all_flags.iter_mut() {
    flag.lock().unwrap().set_value("hello".to_owned())
  }
}
