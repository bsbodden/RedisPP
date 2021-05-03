use build_html::Html;

#[derive(Debug)]
pub struct TextNode {
  pub content: String
}

impl Html for TextNode {
  fn to_html_string(&self) -> String {
    format!("{}", self.content)
  }
}