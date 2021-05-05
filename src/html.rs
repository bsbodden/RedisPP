use build_html::*;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub(crate) struct TextNode {
  pub content: String,
}

impl Html for TextNode {
  fn to_html_string(&self) -> String {
    format!("{}", self.content)
  }
}

pub(crate) fn html_table_from_hashmap(hashmap: HashMap<String, String>) -> String {
  let titles = Vec::from_iter(hashmap.keys());
  let values = Vec::from_iter(hashmap.values());

  let source_table = [values];
  let html_table = build_html::Table::from(&source_table)
    .add_header_row(&titles)
    .to_html_string();

  return html_table;
}

pub(crate) fn html_list_from_vector(list: Vec<String>, ctype: ContainerType) -> String {
  let mut html_list = build_html::Container::new(ctype);
  for e in list {
    let text_node = TextNode { content: e };
    html_list = html_list.add_html(Box::new(text_node));
  }

  return html_list.to_html_string();
}
