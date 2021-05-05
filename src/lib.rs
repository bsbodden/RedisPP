#[macro_use]
extern crate redis_module;
extern crate build_html;
extern crate json_color;
extern crate prettytable;

mod ascii;
mod html;
mod json;
mod utils;

use build_html::*;
use redis_module::raw::KeyType;
use redis_module::{Context, NextArg, RedisError, RedisResult};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use ascii::{to_ascii_table, to_titled_ascii_table};
use html::{html_list_from_vector, html_table_from_hashmap};
use json::{to_json};
use utils::{extract_strings, process_redis_result, vec_to_hashmap, PPOptions, PPCommands};

const MIN_ARGS: usize = 1;
const MAX_ARGS: usize = 2;

fn pp_j(ctx: &Context, args: Vec<String>) -> RedisResult {
  let mut args = args.into_iter().skip(1);
  let args_card = args.len();
  if args_card < MIN_ARGS || args_card > MAX_ARGS {
    return Err(RedisError::WrongArity);
  }

  let src = args.next_string()?;
  let mut options = HashSet::new();

  while let Some(s) = args.next() {
    match s.to_uppercase().as_str() {
      "PB" => options.insert(PPOptions::PBCopy),
      _ => break,
    };
  }

  let key = ctx.open_key(&src);
  let ktype = key.key_type();

  match ktype {
    KeyType::Hash => {
      let hgetall = ctx.call("HGETALL", &[&src]);
      return process_redis_result(PPCommands::PPJ, hgetall, options, |array| {
        let hashmap: HashMap<String, String> = vec_to_hashmap(array);
        return to_json(&hashmap).unwrap();
      });
    }
    KeyType::List => {
      let lrange = ctx.call("LRANGE", &[&src, "0", "-1"]);
      return process_redis_result(PPCommands::PPJ, lrange, options, |array| {
        let list: Vec<String> = extract_strings(array);

        return to_json(&list).unwrap();
      });
    }
    KeyType::Set => {
      let smembers = ctx.call("SMEMBERS", &[&src]);
      return process_redis_result(PPCommands::PPJ, smembers, options, |array| {
        let set: Vec<String> = extract_strings(array);

        return to_json(&set).unwrap();
      });
    }
    _ => return Err(RedisError::WrongType),
  };
}

fn pp_t(ctx: &Context, args: Vec<String>) -> RedisResult {
  let mut args = args.into_iter().skip(1);
  let args_card = args.len();
  if args_card < MIN_ARGS || args_card > MAX_ARGS {
    return Err(RedisError::WrongArity);
  }

  let src = args.next_string()?;
  let mut options = HashSet::new();

  while let Some(s) = args.next() {
    match s.to_uppercase().as_str() {
      "PB" => options.insert(PPOptions::PBCopy),
      _ => break,
    };
  }

  let key = ctx.open_key(&src);
  let ktype = key.key_type();

  match ktype {
    KeyType::Hash => {
      let hgetall = ctx.call("HGETALL", &[&src]);
      return process_redis_result(PPCommands::PPT, hgetall, options, |array| {
        let hashmap: HashMap<String, String> = vec_to_hashmap(array);
        let titles = Vec::from_iter(hashmap.keys());
        let values = Vec::from_iter(hashmap.values());

        return to_titled_ascii_table(values, titles).to_string();
      });
    }
    KeyType::List => {
      let lrange = ctx.call("LRANGE", &[&src, "0", "-1"]);
      return process_redis_result(PPCommands::PPT, lrange, options, |array| {
        let strings = extract_strings(array);
        let list = strings
          .iter()
          .map(|s| {
            let s: &String = s;
            s
          })
          .collect();

        return to_ascii_table(list).to_string();
      });
    }
    KeyType::Set => {
      let smembers = ctx.call("SMEMBERS", &[&src]);
      return process_redis_result(PPCommands::PPT, smembers, options, |array| {
        let strings: Vec<String> = extract_strings(array);
        let set = strings
          .iter()
          .map(|s| {
            let s: &String = s;
            s
          })
          .collect();

        return to_ascii_table(set).to_string();
      });
    }
    _ => return Err(RedisError::WrongType),
  };
}

fn pp_c(ctx: &Context, args: Vec<String>) -> RedisResult {
  let mut args = args.into_iter().skip(1);
  let args_card = args.len();
  if args_card < MIN_ARGS || args_card > MAX_ARGS {
    return Err(RedisError::WrongArity);
  }

  let src = args.next_string()?;
  let mut options = HashSet::new();

  while let Some(s) = args.next() {
    match s.to_uppercase().as_str() {
      "PB" => options.insert(PPOptions::PBCopy),
      _ => break,
    };
  }

  let key = ctx.open_key(&src);
  let ktype = key.key_type();

  match ktype {
    KeyType::Hash => {
      let hgetall = ctx.call("HGETALL", &[&src]);
      return process_redis_result(PPCommands::PPC, hgetall, options, |array| {
        let hashmap: HashMap<String, String> = vec_to_hashmap(array);
        let titles = Vec::from_iter(hashmap.keys());
        let values = Vec::from_iter(hashmap.values());

        let to_csv = String::from_utf8(
          to_titled_ascii_table(values, titles)
            .to_csv(Vec::new())
            .unwrap()
            .into_inner()
            .unwrap(),
        )
        .unwrap();

        return to_csv;
      });
    }
    KeyType::List => {
      let lrange = ctx.call("LRANGE", &[&src, "0", "-1"]);
      return process_redis_result(PPCommands::PPC, lrange, options, |array| {
        let strings: Vec<String> = extract_strings(array);
        let list = strings
          .iter()
          .map(|s| {
            let s: &String = s;
            s
          })
          .collect();
        let to_csv = String::from_utf8(
          to_ascii_table(list)
            .to_csv(Vec::new())
            .unwrap()
            .into_inner()
            .unwrap(),
        )
        .unwrap();

        return to_csv;
      });
    }
    KeyType::Set => {
      let smembers = ctx.call("SMEMBERS", &[&src]);
      return process_redis_result(PPCommands::PPC, smembers, options, |array| {
        let strings: Vec<String> = extract_strings(array);
        let set = strings
          .iter()
          .map(|s| {
            let s: &String = s;
            s
          })
          .collect();
        let to_csv = String::from_utf8(
          to_ascii_table(set)
            .to_csv(Vec::new())
            .unwrap()
            .into_inner()
            .unwrap(),
        )
        .unwrap();

        return to_csv;
      });
    }
    _ => return Err(RedisError::WrongType),
  };
}

fn pp_h(ctx: &Context, args: Vec<String>) -> RedisResult {
  let mut args = args.into_iter().skip(1);
  let args_card = args.len();
  if args_card < MIN_ARGS || args_card > MAX_ARGS {
    return Err(RedisError::WrongArity);
  }

  let src = args.next_string()?;
  let mut options = HashSet::new();

  while let Some(s) = args.next() {
    match s.to_uppercase().as_str() {
      "PB" => options.insert(PPOptions::PBCopy),
      _ => break,
    };
  }

  let key = ctx.open_key(&src);
  let ktype = key.key_type();

  match ktype {
    KeyType::Hash => {
      let hgetall = ctx.call("HGETALL", &[&src]);
      return process_redis_result(PPCommands::PPH, hgetall, options, |array| {
        let hashmap: HashMap<String, String> = vec_to_hashmap(array);

        return html_table_from_hashmap(hashmap);
      });
    }
    KeyType::List => {
      let lrange = ctx.call("LRANGE", &[&src, "0", "-1"]);
      return process_redis_result(PPCommands::PPH, lrange, options, |array| {
        let list: Vec<String> = extract_strings(array);

        return html_list_from_vector(list, ContainerType::OrderedList);
      });
    }
    KeyType::Set => {
      let smembers = ctx.call("SMEMBERS", &[&src]);
      return process_redis_result(PPCommands::PPH, smembers, options, |array| {
        let set: Vec<String> = extract_strings(array);

        return html_list_from_vector(set, ContainerType::UnorderedList);
      });
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
