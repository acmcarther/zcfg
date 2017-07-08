#![feature(used)]

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

#[macro_export]
macro_rules! define_flag {
  ($name: ident) => {
    pub mod $name {
      _flag_boilerplate!(gflags::FlagPrototype::new(stringify!($name).to_owned(), None));
    }
  };
  ($name: ident, $value: expr) => {
    pub mod $name {
      _flag_boilerplate!(gflags::FlagPrototype::new(stringify!($name).to_owned(), $value));
    }
  }
}

#[macro_export]
macro_rules! _flag_boilerplate {
  ($flag_value: expr) => {
    use std::sync::Arc;
    use std::sync::Mutex;
    use gflags;

    lazy_static! {
      pub static ref FLAG: Arc<Mutex<gflags::FlagPrototype>> = {
        Arc::new(Mutex::new($flag_value))
      };
    }

    pub fn get() -> gflags::FlagPrototype {
      FLAG.lock().unwrap().clone()
    }

    pub fn set_for_testing(value: String) {
      FLAG.lock().unwrap().set_for_testing(value)
    }

    extern "C" fn enqueue_static_flag_init() {
      gflags::STATIC_FLAG_INIT_FNS.lock()
        .unwrap()
        .push(push_static_flag)
    }

    fn push_static_flag(flag_vec: &mut Vec<Arc<Mutex<gflags::FlagPrototype>>>) {
      flag_vec.push(FLAG.clone())
    }

    #[used]
    #[link_section = ".init_array"]
    #[allow(dead_code)]
    static INIT_ARRAY: [extern "C" fn(); 1] = [enqueue_static_flag_init];
  }
}

lazy_static! {
  pub static ref STATIC_FLAG_INIT_FNS: Mutex<Vec<fn(&mut Vec<Arc<Mutex<FlagPrototype>>>)>> = {
    Mutex::new(Vec::new())
  };
}

#[derive(Clone)]
pub struct FlagPrototype {
  name: String,
  initialized: bool,
  value: Option<String>,
}

impl FlagPrototype {
  pub fn new(name: String, value: Option<String>) -> FlagPrototype {
    FlagPrototype {
      name: name,
      initialized: false,
      value: value,
    }
  }

  fn initialize(&mut self, v: String) {
    assert!(!self.initialized, format!("Flag {} was already initialized once", self.name));
    self.initialized = true;
    self.value = Some(v)
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_value(&self) -> Option<String> {
    self.value.clone()
  }

  pub fn set_for_testing(&mut self, v: String) {
    self.value = Some(v)
  }
}

pub fn populate_flags() {
  println!("populating flags");
  let mut all_flags = Vec::new();
  {
    for f in STATIC_FLAG_INIT_FNS.lock().unwrap().iter() {
      f(&mut all_flags)
    }
  }

  for flag_mutex in all_flags.iter_mut() {
    flag_mutex.lock().unwrap().initialize("hello".to_owned());
  }
}
