mod math;
pub use math::utils::test;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hello() {
    assert_eq!(math::utils::test(), 5);
  }
}
