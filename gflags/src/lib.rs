#![feature(used)]

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

#[macro_export]
macro_rules! define_flag {
  ($name: ident) => {
    mod $name {
      use std::sync::Arc;
      use std::sync::Mutex;
      use gflags;

      lazy_static! {
        pub static ref FLAG: Arc<Mutex<gflags::FlagPrototype>> = {
          Arc::new(Mutex::new(gflags::FlagPrototype::new(stringify!($name).to_owned())))
        };
      }

      pub fn get_FLAG() -> gflags::FlagPrototype {
        FLAG.lock().unwrap().clone()
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
}
lazy_static! {
  pub static ref STATIC_FLAG_INIT_FNS: Mutex<Vec<fn(&mut Vec<Arc<Mutex<FlagPrototype>>>)>> = {
    Mutex::new(Vec::new())
  };
}

#[derive(Clone)]
pub struct FlagPrototype {
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

  pub fn get_value(&self) -> Option<String> {
    self.value.clone()
  }
}

pub fn populate_flags() {
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
