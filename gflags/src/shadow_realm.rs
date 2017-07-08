#![feature(optin_builtin_traits)]

#[macro_use]
extern crate lazy_static;

#[macro_export]
mod define;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

trait ParseFromString {
}

pub enum FlagParseErr {
}

pub enum FlagParseAndSetErr {
  ParseErr(FlagParseErr),
  SetErr(FlagParseErr),
}

struct FlagStr;

trait FlagParseAndSetable: Sync + Send {
  fn parse_and_set(&mut self, s: &str) -> Result<(), FlagParseAndSetErr>;
}

impl <T> FlagParseAndSetable for Flag<T> where T: FlagParsable {
  fn parse_and_set(&mut self, s: &str) -> Result<(), FlagParseAndSetErr> {
    T::parse(s)
      .map_err(FlagParseAndSetErr::ParseErr)
      .and_then(|v| self.set_internal(v).map(FlagParseAndSetErr::SetErr));
  }
}

trait FlagParsable : Sync + Send {
  fn parse(s: &str) -> Result<Box<Self>, FlagParseErr>;
}

lazy_static!{
  static ref FLAG_REGISTRY: FlagRegistry = {
    FlagRegistry::new()
  };
}

pub enum FlagRegistryErr {
  AlreadyExists
}

pub struct FlagResolveResult {
  errors: HashMap<String, FlagResolveErr>,
}

pub enum FlagResolveErr {
  ParseAndSetErr(FlagParseAndSetErr),
  UndefinedFlag,
}

struct CommandStr {
  entries: HashMap<String, Option<String>>,
}

pub struct FlagRegistry {
  flags: HashMap<String, Arc<Mutex<FlagParseAndSetable>>>,
}

impl FlagRegistry {
  pub fn new() -> FlagRegistry {
    FlagRegistry {
      flags: HashMap::new(),
    }
  }

  pub unsafe fn register_flag<T>(&mut self, name: String, flag: *'static Flag<T>>>) -> Result<(), FlagRegistryErr> {
    // TODO(acmcarther): Check if we're equal to the negation of another flag
    if self.flags.contains(&name) {
      return Err(FlagRegistryErr::AlreadyExists);
    }
    self.flags.insert(name, flag);
    Ok(())
  }

  pub unsafe fn resolve_flags_from_command(&mut self, input: &HashMap<String, Option<String>>) -> HashMap<String, Result<(), FlagResolveErr>> {
    let mut result: HashMap::new();
    for (key, value) in input.iter() {
      let parse_and_set_result = {
        if self.flags.contains(key) {
          let sure_value = value.unwrap_or_else(|| "true".to_owned());
          self.flags.get_mut(key).lock().parse_and_set("false").map_err(FlagResolveErr::ParseAndSetErr)
        } else if value.is_none() && key.starts_with("no-") {
          let negative_key = key.chars().skip(3 /* "no-" */).collect::<String>();
          let negative_value = "false";

          if self.flags.contains(negative_key) {
            self.flags.get_mut(negative_key).parse_and_set("false").map_err(FlagResolveErr::ParseAndSetErr)
          } else {
            Err(FlagResolveErr::UndefinedFlag)
          }
        } else {
          Err(FlagResolveErr::UndefinedFlag)
        }
      };
      result.insert(key.clone(), parse_and_set_result)
    }

    result
  }
}

struct Flag<T> {
  name: String,
  initialized: bool,
  allow_overwrite: bool,
  value: Option<T>
}

pub enum FlagSetErr {
  Uninitialized,
  OverwriteNotAllowed,
}

pub enum FlagInitErr {
  AlreadyInitialized,
}

impl <T> Flag<T> {
  pub fn with_default<S>(name: S, value: T) -> Flag<T> where S: Into<String> {
    Flag::new_internal(name.into(), Some(value))
  }

  pub fn new<S>(name: S) -> Flag<T> where S: Into<String> {
    Flag::new_internal(name.into(), None)
  }

  pub fn set_overwritable(&mut self, setting: bool) {
    self.allow_overwrite = setting;
  }

  pub fn set(&mut self, v: T) -> Result<(), FlagSetErr>{
    if !self.initialized {
      return Err(FlagSetErr::Uninitialized)
    }

    if !self.allow_overwrite {
      return Err(FlagSetErr::OverwriteNotAllowed)
    }

    self.value = Some(v);
    return Ok(())
  }

  fn set_internal(&mut self, value: T) -> Result<(), FlagInitErr> {
    if self.initialized {
      return Err(FlagInitErr::AlreadyInitialized)
    }


    self.value = Some(value);
    Ok(())
  }

  fn new_internal(name: String, value: Option<T>) -> Flag<T> {
    Flag {
      name: name,
      initialized: false,
      allow_overwrite: false,
      value: value,
    }
  }
}
