mod poly;
use crate::poly::polynomial::*;
use crate::poly::*;

fn main() {
  let points3 = vec![
    Point::new((1., 1.)),
    Point::new((2., 4.)),
    Point::new((7., 9.)),
  ];

  println!("{:?}", Polynomial::<f64>::new(vec![1., 2., 3.]).pretty("x"));
  println!(
    "{:?}",
    Polynomial::<f64>::new(vec![-8., 17., 0., 5.]).pretty("x")
  );

  println!("{:?}", single_term(&points3, 2).pretty("x"));
  println!(
    "{:?}",
    interpolate(&vec![Point::new((1., 1.)), Point::new((2., 0.))]).pretty("x")
  );

  let poly = interpolate(&points3);
  println!("{:?}", poly.pretty("x"));
  points3
    .iter()
    .for_each(|p| println!("{:?}", poly.eval(p.x())));
}
