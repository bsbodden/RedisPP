#[macro_use]
extern crate redis_module;
extern crate build_html;
extern crate json_color;
extern crate prettytable;

mod utils;

use build_html::*;
use itertools::Itertools;
use itertools::Tuples;
use json_color::{Color, Colorizer};
use prettytable::{format, Row, Table};
use redis_module::raw::KeyType;
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisValue};
use serde::ser::Serialize;
use std::collections::HashMap;
use std::iter::FromIterator;
use utils::TextNode;

fn is_string(v: RedisValue) -> Option<String> {
  match v {
    RedisValue::SimpleString(s) => Some(s),
    RedisValue::BulkString(s) => Some(s),
    _ => None,
  }
}

fn extract_strings(mut values: Vec<RedisValue>) -> Vec<String> {
  values.drain(..).filter_map(is_string).collect()
}

fn vec_to_hashmap(values: Vec<RedisValue>) -> HashMap<String, String> {
  let tuples: Tuples<
    std::vec::IntoIter<std::string::String>,
    (std::string::String, std::string::String),
  > = extract_strings(values).into_iter().tuples::<(_, _)>();

  let hashmap: HashMap<String, String> = tuples.collect();
  hashmap
}

fn to_colorized_json<T>(value: &T) -> Result<String, std::io::Error>
where
  T: ?Sized + Serialize,
{
  let colorizer = Colorizer::new()
    .null(Color::Cyan)
    .boolean(Color::Yellow)
    .number(Color::Magenta)
    .string(Color::Green)
    .key(Color::Blue)
    .build();
  let json = serde_json::to_string_pretty(value)?;
  return colorizer.colorize_json_str(&json.to_string());
}

fn to_ascii_table(values: Vec<&String>) -> Table {
  let values_row = Row::from(values);
  let mut table = Table::new();
  table.set_format(*format::consts::FORMAT_NO_LINESEP);
  table.add_row(values_row);
  table
}

fn to_titled_ascii_table(values: Vec<&String>, titles: Vec<&String>) -> Table {
  let titles_row = Row::from(titles);
  let values_row = Row::from(values);
  let mut table = Table::new();
  table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
  table.set_titles(titles_row);
  table.add_row(values_row);
  table
}

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
          let titles = Vec::from_iter(hashmap.keys());
          let values = Vec::from_iter(hashmap.values());

          let source_table = [values];
          let html_table = build_html::Table::from(&source_table)
            .add_header_row(&titles)
            .to_html_string();

          return Ok(RedisValue::SimpleString(html_table));
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
          let mut html_list = build_html::Container::new(ContainerType::OrderedList);
          for e in list {
            let text_node = TextNode { content: e };
            html_list = html_list.add_html(Box::new(text_node));
          }

          return Ok(RedisValue::SimpleString(html_list.to_html_string()));
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
          let mut html_list = build_html::Container::new(ContainerType::UnorderedList);
          for e in set {
            let text_node = TextNode { content: e };
            html_list = html_list.add_html(Box::new(text_node));
          }

          return Ok(RedisValue::SimpleString(html_list.to_html_string()));
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
