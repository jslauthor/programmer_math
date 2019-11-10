pub mod polynomial;
use crate::poly::polynomial::*;
use std::cmp::Ordering;

pub struct Point(f64, f64);

impl Point {
  pub fn new(point: (f64, f64)) -> Point {
    Point(point.0, point.1)
  }

  pub fn x(&self) -> f64 {
    self.0
  }

  pub fn y(&self) -> f64 {
    self.1
  }
}

impl Ord for Point {
  fn cmp(&self, other: &Self) -> Ordering {
    // order by distance from 0, 0
    let distance1 = (self.0.powf(2.0) + self.1.powf(2.0)).sqrt();
    let distance2 = (other.0.powf(2.0) + other.1.powf(2.0)).sqrt();
    if distance1 > distance2 {
      return Ordering::Greater;
    } else if distance1 < distance2 {
      return Ordering::Less;
    }

    Ordering::Equal
  }
}

impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

impl Eq for Point {}

impl Clone for Point {
  fn clone(&self) -> Self {
    Point(self.0, self.1)
  }
}

pub fn interpolate(points: &Vec<Point>) -> Polynomial<f64> {
  if points.len() == 0 {
    panic!("Must provide at least one point!");
  }

  let mut new_vec = points.iter().fold(vec![], |mut acc, point| {
    acc.push(point.0);
    acc
  });
  new_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
  new_vec.dedup();

  if points.len() != new_vec.len() {
    panic!("Not all X values are distinct.");
  }

  // This is such a dumb approach
  points
    .iter()
    .enumerate()
    .map(|(index, _point)| single_term(&points, index))
    .fold(Polynomial::<f64>::new(vec![]), |acc, term| acc + term)
}

pub fn single_term(points: &Vec<Point>, index: usize) -> Polynomial<f64> {
  let term = Polynomial::<f64>::new(vec![1.]);
  match points.get(index) {
    Some(Point(xi, yi)) => {
      points.iter().enumerate().fold(term, |acc, p| match p {
        (idx, point) => {
          if idx == index {
            return acc;
          }
          let xj = point.0;
          acc * Polynomial::<f64>::new(vec![-xj / (xi - xj), 1. / (xi - xj)])
        }
      }) * Polynomial::<f64>::new(vec![*yi])
    }
    None => term,
  }
}
