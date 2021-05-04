pub mod json {
  use serde::ser::Serialize;
  use json_color::{Color, Colorizer};

  pub(crate) fn to_colorized_json<T>(value: &T) -> Result<String, std::io::Error>
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
}