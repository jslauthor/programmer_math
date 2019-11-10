// extern crate polynomial;
mod poly;
pub use crate::poly::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn interpolate_should_panic_with_zero_points() {
    //    let p1: Point = Point::new((1., 1.));
    interpolate(&vec![]);
  }

  #[test]
  #[should_panic]
  fn interpolate_points_should_contain_unique_x_values() {
    interpolate(&vec![Point::new((1., 1.)), Point::new((1., 1.))]);
  }

  #[test]
  fn single_term_should_return_one_term_of_interpolated_points() {
    let points3 = vec![
      Point::new((1., 1.)),
      Point::new((2., 4.)),
      Point::new((7., 9.)),
    ];
    assert_eq!(single_term(&points3, 2).pretty("x"), "0.6-0.9x+0.3x^2");
  }

  #[test]
  fn interpolate_should_accurately_evaluate_points() {
    let points3 = vec![
      Point::new((1., 1.)),
      Point::new((2., 4.)),
      Point::new((7., 9.)),
    ];
    let poly = interpolate(&points3);
    let x_values = points3.iter().fold(vec![], |mut acc, p| {
      acc.push(poly.eval(p.x()));
      acc
    });
    assert_eq!(vec![1., 3.999999999999999, 8.99999999999999], x_values);
  }
}
