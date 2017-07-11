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
    _define_flag_inner!(pub $name, $t, None);
  };
  ($name:ident : $t:ty) => {
    _define_flag_inner!($name, $t, None);
  };
  (pub $name:ident : $t:ty = $value:expr) => {
    _define_flag_inner!(pub $name, $t, $value);
  };
  ($name:ident : $t:ty = $value:expr) => {
    _define_flag_inner!($name, $t, $value);
  }
}

#[macro_export]
macro_rules! _define_flag_inner {
  (pub $name:ident, $t:ty, $value:expr) => {
    pub mod $name {
      _flag_boilerplate!($name, $t, $value);
    }
  };
  ($name:ident, $t:ty, $value:expr) => {
    mod $name {
      _flag_boilerplate!($name, $t, $value);
    }
  };
}

#[macro_export]
macro_rules! _flag_boilerplate {
  ($name:ident, $t:ty, $flag_value:expr) => {
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::ops::Deref;

    fn __inner_try_set_statically(s: &str) -> Result<(), $crate::InitErr> {
      <$t as $crate::FlagParseable>::parse_from_str(s)
        .map_err(|e| $crate::InitErr::FailedToParse(e.to_string()))
        .and_then(|out| {
          if !_FLAG_INNER.lock().expect("somebody soiled a flag").initialize(out) {
            Err($crate::InitErr::AlreadyInitOnce)
          } else {
            Ok(())
          }
        })
    }

    fn __inner_get() -> Option<$t> {
      _FLAG_INNER.deref().lock().expect("somebody soiled a flag").get()
    }

    fn __inner_set_for_testing(v: $t) {
      _FLAG_INNER.deref().lock().expect("somebody soiled a flag").set_raw(v)
    }

    lazy_static! {
      pub static ref FLAG: $crate::Flag<$t> = {
        $crate::Flag::new_statically(stringify!($name).to_owned(), __inner_get, __inner_set_for_testing)
      };

      #[allow(dead_code)]
      pub static ref FLAG_INITIALIZER: $crate::FlagInitializer = {
        $crate::FlagInitializer::new(
          stringify!($name).to_owned(),
          file!().to_owned(),
          line!(),
          __inner_try_set_statically,
        )
      };

      static ref _FLAG_INNER: Arc<Mutex<$crate::FlagValue<$t>>> = {
        Arc::new(Mutex::new($crate::FlagValue::new($flag_value)))
      };
    }

    extern "C" fn enqueue_static_flag_init() {
      $crate::STATIC_FLAG_INITIALIZERS.lock()
        .unwrap()
        .push($crate::FlagInitializer::new(
          stringify!($name).to_owned(),
          file!().to_owned(),
          line!(),
          __inner_try_set_statically,
        ))
    }

    #[used]
    #[link_section = ".init_array"]
    #[allow(dead_code)]
    static INIT_ARRAY: [extern "C" fn(); 1] = [enqueue_static_flag_init];
  }
}

pub struct Flag<T: Clone> {
  name: String,
  _inner_get_value: fn() -> Option<T>,
  _inner_set_for_testing: fn(T),
}

impl<T: Clone> Flag<T> {
  pub fn new_statically(name: String, get_value: fn() -> Option<T>, set_for_testing: fn(T)) -> Flag<T> {
    Flag {
      name: name,
      _inner_get_value: get_value,
      _inner_set_for_testing: set_for_testing,
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_value(&self) -> Option<T> {
    (self._inner_get_value)()
  }

  pub fn set_for_testing(&self, v: T) {
    (self._inner_set_for_testing)(v)
  }
}

type ParseErr = String;

pub trait FlagParseable {
  type Output;
  fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr>;
}

impl FlagParseable for String {
  type Output = String;
  fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr> {
    Ok(s.to_owned())
  }
}

macro_rules! decl_flag_parsable_from_str {
  ($t:tt) => {
    _inner_decl_flag_parsable_from_str!($t, $t);
  }
}

macro_rules! _inner_decl_flag_parsable_from_str {
  ($t:ty, $t_ident:ident) => {
    impl FlagParseable for $t {
      type Output = $t;
      fn parse_from_str(s: &str) -> Result<Self::Output, ParseErr> {
        use std::str::FromStr;
        $t_ident::from_str(s).map_err(|e| e.to_string())
      }
    }
  }
}

decl_flag_parsable_from_str!(bool);
decl_flag_parsable_from_str!(u8);
decl_flag_parsable_from_str!(u32);
decl_flag_parsable_from_str!(u64);
decl_flag_parsable_from_str!(i8);
decl_flag_parsable_from_str!(i32);
decl_flag_parsable_from_str!(i64);
decl_flag_parsable_from_str!(f32);
decl_flag_parsable_from_str!(f64);

pub struct FlagValue<T: Clone> {
  value: Option<T>,
  initialized: bool
}

impl<T: Clone> FlagValue<T> {
  pub fn new(default: Option<T>) -> FlagValue<T> {
    FlagValue {
      value: default,
      initialized: false
    }
  }
  pub fn get(&self) -> Option<T> {
    self.value.clone()
  }

  pub fn set_raw(&mut self, t: T) {
    self.value = Some(t);
  }

  pub fn initialize(&mut self, t: T) -> bool {
    if self.initialized {
      return false
    }

    self.set_raw(t);
    self.initialized = true;
    true
  }
}

pub enum InitErr {
  AlreadyInitOnce,
  FailedToParse(String),
}

pub struct FlagInitializer {
  name: String,
  file_name: String,
  line_number: u32,
  internal_set_statically: fn(&str) -> Result<(), InitErr>,
}

impl FlagInitializer {
  pub fn new(name: String, file_name: String, line_number: u32, initialize: fn(&str) -> Result<(), InitErr>) -> FlagInitializer {
    FlagInitializer {
      name: name,
      file_name: file_name,
      line_number: line_number,
      internal_set_statically: initialize,
    }
  }

  pub fn flag_name(&self) -> &str {
    &self.name
  }

  pub fn file(&self) -> &str {
    &self.file_name
  }

  pub fn line(&self) -> u32 {
    self.line_number
  }

  pub fn set_statically(&self, s: &str) -> Result<(), InitErr> {
    (self.internal_set_statically)(s)
  }
}

lazy_static! {
  pub static ref STATIC_FLAG_INITIALIZERS: Mutex<Vec<FlagInitializer>> = {
    Mutex::new(Vec::new())
  };
}

pub enum GlobalInitErr {
  SharesNameWith{
    file_name: String,
    line_number: u32
  },
  InitErr(InitErr),
}

pub fn populate_flags() -> Result<(), Vec<(String, GlobalInitErr)>> {
  let mut errs = Vec::new();

  // TODO: Detect name conflicts
  for initializer in STATIC_FLAG_INITIALIZERS.lock().unwrap().iter() {
    println!("Initializing flag [{}] from [{}:{}]", initializer.flag_name(), initializer.file(), initializer.line());
    if let Err(init_err) = initializer.set_statically("TODO") {
      errs.push((initializer.flag_name().to_owned(), GlobalInitErr::InitErr(init_err)))
    }
  }

  if errs.is_empty() {
    Ok(())
  } else {
    Err(errs)
  }
}

#[cfg(test)]
mod test {
  define_flag!(pub example_1: String = Some("hello".to_owned()));
  define_flag!(pub example_2: u32 = Some(5));
  define_flag!(pub example_3: String);
  define_flag!(pub example_4: u32);
  use self::example_1::FLAG as FLAG_example1;
  use self::example_2::FLAG as FLAG_example2;
  use self::example_3::FLAG as FLAG_example3;
  use self::example_4::FLAG as FLAG_example4;

  #[test]
  fn go() {
    // TODO
  }
}
