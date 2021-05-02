#[macro_use] extern crate redis_module;
extern crate json_color;
extern crate prettytable;

use itertools::Itertools;
use redis_module::raw::KeyType;
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisValue};
use std::collections::HashMap;
use std::iter::FromIterator;
use itertools::Tuples;
use json_color::{Colorizer, Color};
use prettytable::{Table, Row, format};

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
  let tuples: Tuples<std::vec::IntoIter<std::string::String>, (std::string::String, std::string::String)> = extract_strings(values)
    .into_iter()
    .tuples::<(_, _)>();

  let hashmap: HashMap<String, String> = tuples.collect();
  hashmap
}

fn pp_j(ctx: &Context, args: Vec<String>) -> RedisResult {
  let colorizer = Colorizer::new()
            .null(Color::Cyan)
            .boolean(Color::Yellow)
            .number(Color::Magenta)
            .string(Color::Green)
            .key(Color::Blue)
            .build();

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
          let json = serde_json::to_string_pretty(&hashmap)?;
          let colorized = colorizer.colorize_json_str(&json.to_string());

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
          let titles_row = Row::from(titles);
          let values = Vec::from_iter(hashmap.values());
          let values_row = Row::from(values);
          let mut table = Table::new();
          table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
          table.set_titles(titles_row);
          table.add_row(values_row);

          return Ok(RedisValue::SimpleString(table.to_string()));
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
        ["pp.t", pp_t, "", 0, 0, 0]
    ],
}

//////////////////////////////////////////////////////
