// 
pub fn parse(content: &str) -> usize {
  // TODO
  let count = content.len();
  count
}

#[cfg(test)]
mod tests {
  use super::parse;
  #[test]
  fn parse_simple() {
    assert_eq!(5, parse("world"));
  }
}