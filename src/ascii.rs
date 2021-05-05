use prettytable::{format, Row, Table};

pub(crate) fn to_ascii_table(values: Vec<&String>) -> Table {
  let values_row = Row::from(values);
  let mut table = Table::new();
  table.set_format(*format::consts::FORMAT_NO_LINESEP);
  table.add_row(values_row);
  table
}

pub(crate) fn to_titled_ascii_table(values: Vec<&String>, titles: Vec<&String>) -> Table {
  let titles_row = Row::from(titles);
  let values_row = Row::from(values);
  let mut table = Table::new();
  table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
  table.set_titles(titles_row);
  table.add_row(values_row);
  table
}
