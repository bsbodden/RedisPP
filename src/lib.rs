#[macro_use]
extern crate redis_module;
extern crate build_html;
extern crate json_color;
extern crate prettytable;

mod ascii;
mod html;
mod json;
mod utils;

use redis_module::raw::KeyType;
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisValue};
use std::collections::HashMap;
use std::iter::FromIterator;
use build_html::*;

use json::json::to_colorized_json;
use ascii::ascii::{to_ascii_table,to_titled_ascii_table};
use html::html::{html_table_from_hashmap,html_list_from_vector};
use utils::utils::{extract_strings,vec_to_hashmap};

fn pp_j(ctx: &Context, args: Vec<String>) -> RedisResult {
  let mut args = args.into_iter().skip(1);
  if (args.len()) != 1 {
    return Err(RedisError::WrongArity);
  }

  let src = args.next_string()?;
  let key = ctx.open_key(&src);
  let ktype = key.key_type();

  match ktype {
    KeyType::Hash => {
      let hgetall = ctx.call("HGETALL", &[&src]);
      match hgetall {
        Ok(RedisValue::Array(array)) => {
          let hashmap: HashMap<String, String> = vec_to_hashmap(array);
          let colorized = to_colorized_json(&hashmap);

          return Ok(RedisValue::SimpleString(colorized.unwrap()));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    KeyType::List => {
      let lrange = ctx.call("LRANGE", &[&src, "0", "-1"]);
      match lrange {
        Ok(RedisValue::Array(array)) => {
          let list: Vec<String> = extract_strings(array);
          let colorized = to_colorized_json(&list);

          return Ok(RedisValue::SimpleString(colorized.unwrap()));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    KeyType::Set => {
      let smembers = ctx.call("SMEMBERS", &[&src]);
      match smembers {
        Ok(RedisValue::Array(array)) => {
          let list: Vec<String> = extract_strings(array);
          let colorized = to_colorized_json(&list);

          return Ok(RedisValue::SimpleString(colorized.unwrap()));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    _ => return Err(RedisError::WrongType),
  };
}

fn pp_t(ctx: &Context, args: Vec<String>) -> RedisResult {
  let mut args = args.into_iter().skip(1);
  if (args.len()) != 1 {
    return Err(RedisError::WrongArity);
  }

  let src = args.next_string()?;
  let key = ctx.open_key(&src);
  let ktype = key.key_type();

  match ktype {
    KeyType::Hash => {
      let hgetall = ctx.call("HGETALL", &[&src]);
      match hgetall {
        Ok(RedisValue::Array(array)) => {
          let hashmap: HashMap<String, String> = vec_to_hashmap(array);
          let titles = Vec::from_iter(hashmap.keys());
          let values = Vec::from_iter(hashmap.values());

          return Ok(RedisValue::SimpleString(to_titled_ascii_table(values, titles).to_string()));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    KeyType::List => {
      let lrange = ctx.call("LRANGE", &[&src, "0", "-1"]);
      match lrange {
        Ok(RedisValue::Array(array)) => {
          let strings = extract_strings(array);
          let list = strings.iter().map(|s| { let s: &String = s; s }).collect();

          return Ok(RedisValue::SimpleString(to_ascii_table(list).to_string()));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    KeyType::Set => {
      let smembers = ctx.call("SMEMBERS", &[&src]);
      match smembers {
        Ok(RedisValue::Array(array)) => {
          let strings: Vec<String> = extract_strings(array);
          let set = strings.iter().map(|s| { let s: &String = s; s }).collect();

          return Ok(RedisValue::SimpleString(to_ascii_table(set).to_string()));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    _ => return Err(RedisError::WrongType),
  };
}

fn pp_c(ctx: &Context, args: Vec<String>) -> RedisResult {
  let mut args = args.into_iter().skip(1);
  if (args.len()) != 1 {
    return Err(RedisError::WrongArity);
  }

  let src = args.next_string()?;
  let key = ctx.open_key(&src);
  let ktype = key.key_type();

  match ktype {
    KeyType::Hash => {
      let hgetall = ctx.call("HGETALL", &[&src]);
      match hgetall {
        Ok(RedisValue::Array(array)) => {
          let hashmap: HashMap<String, String> = vec_to_hashmap(array);
          let titles = Vec::from_iter(hashmap.keys());
          let values = Vec::from_iter(hashmap.values());

          let to_csv =
            String::from_utf8(to_titled_ascii_table(values, titles).to_csv(Vec::new()).unwrap().into_inner().unwrap()).unwrap();

          return Ok(RedisValue::SimpleString(to_csv));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    KeyType::List => {
      let lrange = ctx.call("LRANGE", &[&src, "0", "-1"]);
      match lrange {
        Ok(RedisValue::Array(array)) => {
          let strings: Vec<String> = extract_strings(array);
          let list = strings.iter().map(|s| { let s: &String = s; s }).collect();
          let to_csv =
            String::from_utf8(to_ascii_table(list).to_csv(Vec::new()).unwrap().into_inner().unwrap()).unwrap();

          return Ok(RedisValue::SimpleString(to_csv));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    KeyType::Set => {
      let smembers = ctx.call("SMEMBERS", &[&src]);
      match smembers {
        Ok(RedisValue::Array(array)) => {
          let strings: Vec<String> = extract_strings(array);
          let set = strings.iter().map(|s| { let s: &String = s; s }).collect();
          let to_csv =
            String::from_utf8(to_ascii_table(set).to_csv(Vec::new()).unwrap().into_inner().unwrap()).unwrap();

          return Ok(RedisValue::SimpleString(to_csv));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    _ => return Err(RedisError::WrongType),
  };
}

fn pp_h(ctx: &Context, args: Vec<String>) -> RedisResult {
  let mut args = args.into_iter().skip(1);
  if (args.len()) != 1 {
    return Err(RedisError::WrongArity);
  }

  let src = args.next_string()?;
  let key = ctx.open_key(&src);
  let ktype = key.key_type();

  match ktype {
    KeyType::Hash => {
      let hgetall = ctx.call("HGETALL", &[&src]);
      match hgetall {
        Ok(RedisValue::Array(array)) => {
          let hashmap: HashMap<String, String> = vec_to_hashmap(array);

          return Ok(RedisValue::SimpleString(html_table_from_hashmap(hashmap)));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    KeyType::List => {
      let lrange = ctx.call("LRANGE", &[&src, "0", "-1"]);
      match lrange {
        Ok(RedisValue::Array(array)) => {
          let list: Vec<String> = extract_strings(array);

          return Ok(RedisValue::SimpleString(html_list_from_vector(list, ContainerType::OrderedList)));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    KeyType::Set => {
      let smembers = ctx.call("SMEMBERS", &[&src]);
      match smembers {
        Ok(RedisValue::Array(array)) => {
          let set: Vec<String> = extract_strings(array);

          return Ok(RedisValue::SimpleString(html_list_from_vector(set, ContainerType::UnorderedList)));
        }
        Ok(_) => return Ok(RedisValue::Null),
        Err(_) => return Err(RedisError::Str("ERR key not found")),
      }
    }
    _ => return Err(RedisError::WrongType),
  };
}

//////////////////////////////////////////////////////

redis_module! {
    name: "pp",
    version: 1,
    data_types: [],
    commands: [
        ["pp.j", pp_j, "", 0, 0, 0],
        ["pp.t", pp_t, "", 0, 0, 0],
        ["pp.c", pp_c, "", 0, 0, 0],
        ["pp.h", pp_h, "", 0, 0, 0]
    ],
}

//////////////////////////////////////////////////////
