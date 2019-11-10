//! Manipulations and data types that represent polynomial.

#![warn(bad_style)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

use num_traits;

use self::num_traits::{One, Zero};
use std::ops::{Add, Mul, Neg, Sub};
use std::{cmp, fmt};

/// Polynomial expression
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Polynomial<T> {
    data: Vec<T>,
}

impl<T: Zero> Polynomial<T> {
    /// Creates new `Polynomial`.
    ///
    /// ```rust
    /// use polynomial::Polynomial;
    /// let poly = Polynomial::new(vec![1, 2, 3]);
    /// assert_eq!("1+2*x+3*x^2", poly.pretty("x"));
    /// ```
    #[inline]
    pub fn new(mut data: Vec<T>) -> Polynomial<T> {
        while let Some(true) = data.last().map(|x| x.is_zero()) {
            let _ = data.pop();
        }
        Polynomial { data: data }
    }
}

impl<T: Zero + One + Clone> Polynomial<T> {
    /// Evaluates the polynomial value.
    ///
    /// ```rust
    /// use polynomial::Polynomial;
    /// let poly = Polynomial::new(vec![1, 2, 3]);
    /// assert_eq!(1, poly.eval(0));
    /// assert_eq!(6, poly.eval(1));
    /// assert_eq!(17, poly.eval(2));
    /// ```
    #[inline]
    pub fn eval(&self, x: T) -> T {
        let mut sum: T = Zero::zero();
        let mut x_n: T = One::one();
        for n in self.data.iter() {
            sum = sum + n.clone() * x_n.clone();
            x_n = x_n * x.clone();
        }
        sum
    }
}

impl<T> Polynomial<T> {
    /// Gets the slice of internal data.
    #[inline]
    pub fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T> Polynomial<T>
where
    T: Zero + One + Neg<Output = T> + PartialEq + PartialOrd + fmt::Display + Clone,
{
    /// Pretty prints the polynomial.
    pub fn pretty(&self, x: &str) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let one = One::one();
        let mut s = Vec::new();
        for (i, n) in self.data.iter().enumerate() {
            // output n*x^i / -n*x^i
            if n.is_zero() {
                continue;
            }

            let term = if i.is_zero() {
                n.to_string()
            } else if i == 1 {
                if (*n) == one {
                    x.to_string()
                } else if (*n) == -one.clone() {
                    format!("-{}", x)
                } else {
                    format!("{}{}", n.to_string(), x)
                }
            } else {
                if (*n) == one {
                    format!("{}^{}", x, i)
                } else if (*n) == -one.clone() {
                    format!("-{}^{}", x, i)
                } else {
                    format!("{}{}^{}", n.to_string(), x, i)
                }
            };

            if s.len() > 0 && (*n) > Zero::zero() {
                s.push("+".to_string());
            }
            s.push(term);
        }

        s.concat()
    }
}

impl<'a, T> Neg for Polynomial<T>
where
    T: Neg + Zero + Clone,
    <T as Neg>::Output: Zero,
{
    type Output = Polynomial<<T as Neg>::Output>;

    #[inline]
    fn neg(self) -> Polynomial<<T as Neg>::Output> {
        -&self
    }
}

impl<'a, T> Neg for &'a Polynomial<T>
where
    T: Neg + Zero + Clone,
    <T as Neg>::Output: Zero,
{
    type Output = Polynomial<<T as Neg>::Output>;

    #[inline]
    fn neg(self) -> Polynomial<<T as Neg>::Output> {
        Polynomial::new(self.data.iter().map(|x| -x.clone()).collect())
    }
}

macro_rules! forward_val_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<Lhs, Rhs> $imp<Polynomial<Rhs>> for Polynomial<Lhs>
        where
            Lhs: Zero + $imp<Rhs> + Clone,
            Rhs: Zero + Clone,
            <Lhs as $imp<Rhs>>::Output: Zero,
        {
            type Output = Polynomial<<Lhs as $imp<Rhs>>::Output>;

            #[inline]
            fn $method(self, other: Polynomial<Rhs>) -> Polynomial<<Lhs as $imp<Rhs>>::Output> {
                (&self).$method(&other)
            }
        }
    };
}

macro_rules! forward_ref_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, Lhs, Rhs> $imp<Polynomial<Rhs>> for &'a Polynomial<Lhs>
        where
            Lhs: Zero + $imp<Rhs> + Clone,
            Rhs: Zero + Clone,
            <Lhs as $imp<Rhs>>::Output: Zero,
        {
            type Output = Polynomial<<Lhs as $imp<Rhs>>::Output>;

            #[inline]
            fn $method(self, other: Polynomial<Rhs>) -> Polynomial<<Lhs as $imp<Rhs>>::Output> {
                self.$method(&other)
            }
        }
    };
}

macro_rules! forward_val_ref_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, Lhs, Rhs> $imp<&'a Polynomial<Rhs>> for Polynomial<Lhs>
        where
            Lhs: Zero + $imp<Rhs> + Clone,
            Rhs: Zero + Clone,
            <Lhs as $imp<Rhs>>::Output: Zero,
        {
            type Output = Polynomial<<Lhs as $imp<Rhs>>::Output>;

            #[inline]
            fn $method(self, other: &Polynomial<Rhs>) -> Polynomial<<Lhs as $imp<Rhs>>::Output> {
                (&self).$method(other)
            }
        }
    };
}

macro_rules! forward_all_binop {
    (impl $imp:ident, $method:ident) => {
        forward_val_val_binop!(impl $imp, $method);
        forward_ref_val_binop!(impl $imp, $method);
        forward_val_ref_binop!(impl $imp, $method);
    };
}

forward_all_binop!(impl Add, add);

impl<'a, 'b, Lhs, Rhs> Add<&'b Polynomial<Rhs>> for &'a Polynomial<Lhs>
where
    Lhs: Zero + Add<Rhs> + Clone,
    Rhs: Zero + Clone,
    <Lhs as Add<Rhs>>::Output: Zero,
{
    type Output = Polynomial<<Lhs as Add<Rhs>>::Output>;

    fn add(self, other: &Polynomial<Rhs>) -> Polynomial<<Lhs as Add<Rhs>>::Output> {
        let max_len = cmp::max(self.data.len(), other.data.len());
        let min_len = cmp::min(self.data.len(), other.data.len());

        let mut sum = Vec::with_capacity(max_len);
        for i in 0..min_len {
            sum.push(self.data[i].clone() + other.data[i].clone());
        }

        if self.data.len() <= other.data.len() {
            for i in min_len..max_len {
                sum.push(num_traits::zero::<Lhs>() + other.data[i].clone());
            }
        } else {
            for i in min_len..max_len {
                sum.push(self.data[i].clone() + num_traits::zero::<Rhs>());
            }
        }

        Polynomial::new(sum)
    }
}

forward_all_binop!(impl Sub, sub);

impl<'a, 'b, Lhs, Rhs> Sub<&'b Polynomial<Rhs>> for &'a Polynomial<Lhs>
where
    Lhs: Zero + Sub<Rhs> + Clone,
    Rhs: Zero + Clone,
    <Lhs as Sub<Rhs>>::Output: Zero,
{
    type Output = Polynomial<<Lhs as Sub<Rhs>>::Output>;

    fn sub(self, other: &Polynomial<Rhs>) -> Polynomial<<Lhs as Sub<Rhs>>::Output> {
        let min_len = cmp::min(self.data.len(), other.data.len());
        let max_len = cmp::max(self.data.len(), other.data.len());

        let mut sub = Vec::with_capacity(max_len);
        for i in 0..min_len {
            sub.push(self.data[i].clone() - other.data[i].clone());
        }
        if self.data.len() <= other.data.len() {
            for i in min_len..max_len {
                sub.push(num_traits::zero::<Lhs>() - other.data[i].clone())
            }
        } else {
            for i in min_len..max_len {
                sub.push(self.data[i].clone() - num_traits::zero::<Rhs>())
            }
        }
        Polynomial::new(sub)
    }
}

forward_all_binop!(impl Mul, mul);

impl<'a, 'b, Lhs, Rhs> Mul<&'b Polynomial<Rhs>> for &'a Polynomial<Lhs>
where
    Lhs: Zero + Mul<Rhs> + Clone,
    Rhs: Zero + Clone,
    <Lhs as Mul<Rhs>>::Output: Zero,
{
    type Output = Polynomial<<Lhs as Mul<Rhs>>::Output>;

    fn mul(self, other: &Polynomial<Rhs>) -> Polynomial<<Lhs as Mul<Rhs>>::Output> {
        if self.is_zero() || other.is_zero() {
            return Polynomial::new(vec![]);
        }

        let slen = self.data.len();
        let olen = other.data.len();
        let prod = (0..slen + olen - 1)
            .map(|i| {
                let mut p = num_traits::zero::<<Lhs as Mul<Rhs>>::Output>();
                let kstart = cmp::max(olen, i + 1) - olen;
                let kend = cmp::min(slen, i + 1);
                for k in kstart..kend {
                    p = p + self.data[k].clone() * other.data[i - k].clone();
                }
                p
            })
            .collect();
        Polynomial::new(prod)
    }
}

impl<T: Zero + Clone> Zero for Polynomial<T> {
    #[inline]
    fn zero() -> Polynomial<T> {
        Polynomial { data: vec![] }
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: Zero + One + Clone> One for Polynomial<T> {
    #[inline]
    fn one() -> Polynomial<T> {
        Polynomial {
            data: vec![One::one()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn new() {
        fn check(dst: Vec<i32>, src: Vec<i32>) {
            assert_eq!(dst, Polynomial::new(src).data);
        }
        check(vec![1, 2, 3], vec![1, 2, 3]);
        check(vec![1, 2, 3], vec![1, 2, 3, 0, 0]);
        check(vec![], vec![0, 0, 0]);
    }

    #[test]
    fn neg_add_sub() {
        fn check(a: &[i32], b: &[i32], c: &[i32]) {
            fn check_eq(a: &Polynomial<i32>, b: &Polynomial<i32>) {
                assert_eq!(*a, *b);
                assert_eq!(-a, -b);
            }
            fn check_add(sum: &Polynomial<i32>, a: &Polynomial<i32>, b: &Polynomial<i32>) {
                check_eq(sum, &(a + b));
                check_eq(sum, &(b + a));
            }
            fn check_sub(sum: &Polynomial<i32>, a: &Polynomial<i32>, b: &Polynomial<i32>) {
                check_eq(a, &(sum - b));
                check_eq(b, &(sum - a));
            }

            let a = &Polynomial::new(a.to_vec());
            let b = &Polynomial::new(b.to_vec());
            let c = &Polynomial::new(c.to_vec());
            check_add(c, a, b);
            check_add(&(-c), &(-a), &(-b));
            check_sub(c, a, b);
            check_sub(&(-c), &(-a), &(-b));
        }
        check(&[], &[], &[]);
        check(&[], &[1], &[1]);
        check(&[1], &[1], &[2]);
        check(&[1, 0, 1], &[1], &[2, 0, 1]);
        check(&[1, 0, -1], &[-1, 0, 1], &[]);
    }

    #[test]
    fn mul() {
        fn check(a: &[i32], b: &[i32], c: &[i32]) {
            let a = Polynomial::new(a.to_vec());
            let b = Polynomial::new(b.to_vec());
            let c = Polynomial::new(c.to_vec());
            assert_eq!(c, &a * &b);
            assert_eq!(c, &b * &a);
        }
        check(&[], &[], &[]);
        check(&[0, 0], &[], &[]);
        check(&[0, 0], &[1], &[]);
        check(&[1, 0], &[1], &[1]);
        check(&[1, 0, 1], &[1], &[1, 0, 1]);
        check(&[1, 1], &[1, 1], &[1, 2, 1]);
        check(&[1, 1], &[1, 0, 1], &[1, 1, 1, 1]);
        check(&[0, 0, 1], &[0, 0, 1], &[0, 0, 0, 0, 1]);
    }

    #[test]
    fn eval() {
        fn check<F: Fn(i32) -> i32>(pol: &[i32], f: F) {
            for n in -10..10 {
                assert_eq!(f(n), Polynomial::new(pol.to_vec()).eval(n));
            }
        }
        check(&[], |_x| 0);
        check(&[1], |_x| 1);
        check(&[1, 1], |x| x + 1);
        check(&[0, 1], |x| x);
        check(&[10, -10, 10], |x| 10 * x * x - 10 * x + 10);
    }

    #[test]
    fn pretty() {
        fn check(slice: &[i32], s: &str) {
            assert_eq!(s.to_string(), Polynomial::new(slice.to_vec()).pretty("x"));
        }
        check(&[0], "0");
        check(&[1], "1");
        check(&[1, 1], "1+x");
        check(&[1, 1, 1], "1+x+x^2");
        check(&[2, 2, 2], "2+2x+2x^2");
        check(&[0, 0, 0, 1], "x^3");
        check(&[0, 0, 0, -1], "-x^3");
        check(&[-1, 0, 0, -1], "-1-x^3");
        check(&[-1, 1, 0, -1], "-1+x-x^3");
        check(&[-1, 1, -1, -1], "-1+x-x^2-x^3");
    }
}
