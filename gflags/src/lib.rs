#![feature(used)]

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::ops::Deref;

#[macro_export]
macro_rules! define_flag {
  (pub $name:ident : $t:ty) => {
    define_flag!(pub $name: $t = None);
  };
  ($name:ident: $t:ty) => {
    define_flag!($name: $t = None);
  };
  (pub $name:ident : $t:ty = $value:expr) => {
    pub mod $name {
      _flag_boilerplate!($name, gflags::FlagInner::new(stringify!($name).to_owned(), $value));
    }
  };
  ($name:ident : $t:ty = $value:expr) => {
    mod $name {
      _flag_boilerplate!($name, gflags::FlagInner::new(stringify!($name).to_owned(), $value));
    }
  }
}


#[macro_export]
macro_rules! _flag_boilerplate {
  ($name: ident, $flag_value: expr) => {
    use std::sync::Arc;
    use std::sync::Mutex;
    use gflags;

    lazy_static! {
      pub static ref FLAG: gflags::FlagRef = {
        gflags::FlagRef::new(&_FLAG_INNER)
      };

      static ref _FLAG_INNER: Arc<Mutex<gflags::FlagInner>> = {
        Arc::new(Mutex::new($flag_value))
      };
    }

    extern "C" fn enqueue_static_flag_init() {
      gflags::STATIC_FLAG_INIT_FNS.lock()
        .unwrap()
        .push(push_static_flag)
    }

    fn push_static_flag(flag_vec: &mut Vec<Arc<Mutex<gflags::FlagInner>>>) {
      flag_vec.push(_FLAG_INNER.clone())
    }

    #[used]
    #[link_section = ".init_array"]
    #[allow(dead_code)]
    static INIT_ARRAY: [extern "C" fn(); 1] = [enqueue_static_flag_init];
  }
}

lazy_static! {
  pub static ref STATIC_FLAG_INIT_FNS: Mutex<Vec<fn(&mut Vec<Arc<Mutex<FlagInner>>>)>> = {
    Mutex::new(Vec::new())
  };
}

pub enum FlagContents {
  String(String),
  Bool(bool),
  U32(u32),
  U64(u64),
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
}

pub struct FlagRef {
  name: String,
  inner: Arc<Mutex<FlagInner>>,
}

impl FlagRef {
  pub fn new<T: Deref<Target = Arc<Mutex<FlagInner>>>>(inner: &T) -> FlagRef {
    let name = inner.deref().lock().unwrap().get_name();
    FlagRef {
      name: name,
      inner: inner.deref().clone(),
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_value(&self) -> Option<String> {
    self.inner.lock().unwrap().get_value().clone()
  }

  pub fn set_for_testing(&self, value: String) {
    self.inner.lock().unwrap().set_for_testing(value)
  }

}

#[derive(Clone)]
pub struct FlagInner {
  name: String,
  initialized: bool,
  value: Option<String>,
}

impl FlagInner {
  pub fn new(name: String, value: Option<String>) -> FlagInner {
    FlagInner {
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
