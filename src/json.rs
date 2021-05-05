use json_color::{Color, Colorizer};
use serde::ser::Serialize;

pub(crate) fn to_colorized_json(json: &str) -> Result<String, std::io::Error> {
  let colorizer = Colorizer::new()
    .null(Color::Cyan)
    .boolean(Color::Yellow)
    .number(Color::Magenta)
    .string(Color::Green)
    .key(Color::Blue)
    .build();
  return colorizer.colorize_json_str(json);
}

pub(crate) fn to_json<T>(value: &T) -> Result<String, serde_json::Error>
where
  T: ?Sized + Serialize,
{
  return serde_json::to_string_pretty(value);
}
