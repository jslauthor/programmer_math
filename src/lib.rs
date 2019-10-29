extern crate polynomial;
pub use polynomial::*;

mod poly;
pub use poly::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn interpolate_should_panic_with_zero_points() {
    //    let p1: Point = Point::new((1., 1.));
    interpolate(vec![]);
  }

  #[test]
  #[should_panic]
  fn interpolate_points_should_contain_unique_values() {
    // let p: Polynomial = Polynomial::new(vec![3., 2.]);
    interpolate(vec![Point::new((1., 1.)), Point::new((1., 1.))]);
  }

  #[test]
  #[should_panic]
  fn oh_hello() {
    // let p: Polynomial = Polynomial::new(vec![3., 2.]);
    interpolate(vec![Point::new((1., 1.)), Point::new((1., 1.))]);
  }
}
