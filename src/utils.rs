extern crate clipboard;

use crate::json::to_colorized_json;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use itertools::Itertools;
use itertools::Tuples;
use redis_module::{RedisError, RedisValue};
use std::collections::HashMap;
use std::collections::HashSet;

pub(crate) fn is_string(v: RedisValue) -> Option<String> {
  match v {
    RedisValue::SimpleString(s) => Some(s),
    RedisValue::BulkString(s) => Some(s),
    _ => None,
  }
}

pub(crate) fn extract_strings(mut values: Vec<RedisValue>) -> Vec<String> {
  values.drain(..).filter_map(is_string).collect()
}

pub(crate) fn vec_to_hashmap(values: Vec<RedisValue>) -> HashMap<String, String> {
  let tuples: Tuples<
    std::vec::IntoIter<std::string::String>,
    (std::string::String, std::string::String),
  > = extract_strings(values).into_iter().tuples::<(_, _)>();

  let hashmap: HashMap<String, String> = tuples.collect();
  hashmap
}

pub(crate) fn pb_copy(content: &str) {
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  ctx.set_contents(String::from(content)).unwrap();
}

#[derive(Debug, Hash, Copy, Clone)]
#[repr(u8)]
pub enum PPOptions {
  PBCopy,
}

impl PartialEq for PPOptions {
  fn eq(&self, other: &PPOptions) -> bool {
    return *self as u8 == *other as u8;
  }
}
impl Eq for PPOptions {}

#[derive(Debug, Hash, Copy, Clone)]
#[repr(u8)]
pub enum PPCommands {
  PPJ,
  PPT,
  PPC,
  PPH,
}

impl PartialEq for PPCommands {
  fn eq(&self, other: &PPCommands) -> bool {
    return *self as u8 == *other as u8;
  }
}
impl Eq for PPCommands {}

pub(crate) fn process_redis_result(
  cmd: PPCommands,
  source: Result<RedisValue, RedisError>,
  options: HashSet<PPOptions>,
  handler: fn(Vec<redis_module::RedisValue>) -> String,
) -> Result<RedisValue, RedisError> {
  match source {
    Ok(RedisValue::Array(array)) => {
      let mut result = handler(array);

      // process options - turn into a loop once there is more than one option
      if let Some(o) = options.iter().next() {
        match o {
          PPOptions::PBCopy => {
            pb_copy(&result);
          }
        }
      }

      // post-processing
      match cmd {
        PPCommands::PPJ => result = to_colorized_json(&result).unwrap(),
        _ => {}
      }

      return Ok(RedisValue::SimpleString(result));
    }
    Ok(_) => return Ok(RedisValue::Null),
    Err(_) => return Err(RedisError::Str("ERR key not found")),
  }
}
