use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

pub trait PointLike<T>
where
    T: Add<Output = T> + Sub<Output = T> + Ord,
{
    fn new(x: T, y: T) -> Self
    where
        Self: Sized;

    fn x(&self) -> T;

    fn y(&self) -> T;

    fn neighbors(&self) -> Vec<Self>
    where
        Self: Sized;

    #[inline]
    fn add(&self, other: &dyn PointLike<T>) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }

    #[inline]
    fn sub(&self, other: &dyn PointLike<T>) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x() - other.x(), self.y() - other.y())
    }

    #[inline]
    fn distance(&self, other: &dyn PointLike<T>) -> T {
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
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
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

                if let Some(x) = self.0.checked_sub(1) {
                    result.push((x, self.1));
                }
                if let Some(x) = self.0.checked_add(1) {
                    result.push((x, self.1));
                }
                if let Some(y) = self.1.checked_sub(1) {
                    result.push((self.0, y));
                }
                if let Some(y) = self.1.checked_add(1) {
                    result.push((self.0, y));
                }

                result
            }
        }
    };
}

point_like!(usize);
point_like!(isize);
point_like!(u32);
point_like!(i32);
point_like!(u64);
point_like!(i64);
