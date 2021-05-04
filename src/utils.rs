pub mod utils {
  use itertools::Itertools;
  use itertools::Tuples;
  use redis_module::RedisValue;
  use std::collections::HashMap;

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
}
