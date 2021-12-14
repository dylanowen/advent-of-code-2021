mod grid;

pub use grid::*;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

pub trait PointLike<Num>
where
    Num: Add<Output = Num> + Sub<Output = Num> + Ord,
{
    fn new(x: Num, y: Num) -> Self
    where
        Self: Sized;

    fn x(&self) -> Num;

    fn y(&self) -> Num;

    fn neighbors(&self) -> Vec<Self>
    where
        Self: Sized;

    #[inline]
    fn add(&self, other: &dyn PointLike<Num>) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }

    #[inline]
    fn sub(&self, other: &dyn PointLike<Num>) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x() - other.x(), self.y() - other.y())
    }

    #[inline]
    fn distance(&self, other: &dyn PointLike<Num>) -> Num {
        let x_distance = if self.x() > other.x() {
            self.x() - other.x()
        } else {
            other.x() - self.x()
        };
        let y_distance = if self.y() > other.y() {
            self.y() - other.y()
        } else {
            other.y() - self.y()
        };

        x_distance + y_distance
    }
}

impl<T> PartialEq for dyn PointLike<T>
where
    T: PartialEq + Add<Output = T> + Sub<Output = T> + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
}

impl<T> Display for dyn PointLike<T>
where
    T: Display + PartialEq + Add<Output = T> + Sub<Output = T> + Ord,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

#[macro_export]
macro_rules! point_like {
    ($primitive:ty) => {
        impl PointLike<$primitive> for ($primitive, $primitive) {
            #[inline]
            fn new(x: $primitive, y: $primitive) -> Self
            where
                Self: Sized,
            {
                (x, y)
            }

            #[inline]
            fn x(&self) -> $primitive {
                self.0
            }

            #[inline]
            fn y(&self) -> $primitive {
                self.1
            }

            fn neighbors(&self) -> Vec<Self> {
                let mut result = Vec::with_capacity(4);

                if let Some(x) = self.x().checked_sub(1) {
                    result.push((x, self.y()));
                }
                if let Some(x) = self.x().checked_add(1) {
                    result.push((x, self.y()));
                }
                if let Some(y) = self.y().checked_sub(1) {
                    result.push((self.x(), y));
                }
                if let Some(y) = self.y().checked_add(1) {
                    result.push((self.x(), y));
                }

                result
            }
        }
    };

    ($primitive:ty, $($other:ty),+) => {
        point_like! { $primitive }
        point_like! { $($other),+ }
    };
}

point_like!(usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8);
