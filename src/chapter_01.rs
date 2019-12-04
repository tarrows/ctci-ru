use std::collections::HashSet;

pub fn q01_a(input: &str) -> bool {
  let mut charset = HashSet::new();

  for c in input.chars() {
    if charset.contains(&c) {
      return false;
    }

    charset.insert(c);
  }

  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn q01_test() {
    assert_eq!(q01_a("apple"), false);
    assert_eq!(q01_a("orange"), true);
  }
}
