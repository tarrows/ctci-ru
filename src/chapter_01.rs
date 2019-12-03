pub fn q01(input: &str) -> String {
  String::from(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn q_01_test() {
    assert_eq!(q01("apple"), "apple");
  }
}
