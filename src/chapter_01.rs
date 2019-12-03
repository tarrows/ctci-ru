pub fn q_01(input: &str) -> String {
  format!("{}", input)
}

#[cfg(test)]
mod tests {
  #[test]
  fn q_01() {
    assert_eq!("apple", "apple");
  }
}
