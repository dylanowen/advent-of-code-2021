use crate::two_d::PointLike;
use std::marker::PhantomData;
use std::ops::{Add, Range, RangeInclusive, Sub};

pub struct Grid<T, Num = usize>
where
    Num: GridIndex,
{
    x_offset: Num,
    y_offset: Num,
    width: Num,

    grid: Vec<Vec<T>>,

    default: T,
}

impl<T, Num> Grid<T, Num>
where
    Num: GridIndex,
{
    pub fn new<XRange: GridRange<Num>, YRange: GridRange<Num>>(
        x: XRange,
        y: YRange,
        default: T,
    ) -> Grid<T, Num> {
        let width = x.end() - x.start();
        let height = (y.end() - y.start()).to_index();

        let mut grid = Vec::with_capacity(height);
        for _ in 0..height {
            grid.push(Vec::with_capacity(width.to_index()))
        }

        Grid {
            x_offset: x.start(),
            y_offset: y.start(),
            width,
            grid,
            default,
        }
    }

    pub fn get<P: PointLike<Num>>(&self, point: P) -> &T {
        let raw_x = self.raw_x(point.x());
        let raw_y = self.raw_y(point.y());

        if raw_x >= Num::zero()
            && raw_y >= Num::zero()
            && raw_y < Num::from_index(self.grid.len())
            && raw_x < Num::from_index(self.grid[raw_y.to_index()].len())
        {
            &self.grid[raw_y.to_index()][raw_x.to_index()]
        } else {
            &self.default
        }
    }

    pub fn set<P: PointLike<Num>>(&mut self, point: P, value: T) -> T {
        point.x();

        todo!()
    }

    pub fn x_min(&self) -> Num {
        self.x_offset
    }

    pub fn y_min(&self) -> Num {
        self.y_offset
    }

    pub fn width(&self) -> Num {
        self.width
    }

    pub fn height(&self) -> Num {
        Num::from_index(self.grid.len())
    }

    // exclusive max
    pub fn x_max(&self) -> Num {
        self.x_min() + self.width()
    }

    // exclusive max
    pub fn y_max(&self) -> Num {
        self.y_min() + self.height()
    }

    fn raw_x(&self, x: Num) -> Num {
        x - self.x_min()
    }

    fn raw_y(&self, y: Num) -> Num {
        y - self.y_min()
    }
}

// pub struct Set;
// pub struct Unset;
//
// pub struct GridBuilder<T, Num, XRange, YRange>
// where
//     Num: Add<Output = Num> + Sub<Output = Num> + Ord,
//     XRange: GridRange<Num>,
//     YRange: GridRange<Num>,
// {
//     x_range: XRange,
//     y_range: YRange,
//
//     default: Option<Box<dyn Fn() -> T>>,
//     _num: PhantomData<Num>,
// }
//
// impl<T, Num, XRange, YRange> GridBuilder<T, Num, XRange, YRange>
// where
//     Num: Add<Output = Num> + Sub<Output = Num> + Ord,
//     XRange: GridRange<Num>,
//     YRange: GridRange<Num>,
// {
//     fn new() {}
// }

pub trait GridRange<Num> {
    fn start(&self) -> Num;
    // exclusive end
    fn end(&self) -> Num;
}

impl<Num: Copy> GridRange<Num> for Range<Num> {
    fn start(&self) -> Num {
        self.start
    }

    fn end(&self) -> Num {
        self.end
    }
}

impl<Num: GridIndex> GridRange<Num> for RangeInclusive<Num> {
    fn start(&self) -> Num {
        *self.start()
    }

    fn end(&self) -> Num {
        *self.end() + Num::one()
    }
}

pub trait GridIndex: Add<Output = Self> + Sub<Output = Self> + Ord + Copy {
    fn zero() -> Self;
    fn one() -> Self;
    fn to_index(self) -> usize;
    fn from_index(index: usize) -> Self;
}

#[macro_export]
macro_rules! grid_index {
    ($primitive:ty) => {
        impl GridIndex for $primitive {
            #[inline]
            fn zero() -> Self {
                0
            }

            #[inline]
            fn one() -> Self {
                1
            }

            #[inline]
            fn to_index(self) -> usize {
                self as usize
            }

            #[inline]
            fn from_index(index: usize) -> Self {
                index as $primitive
            }
        }
    };

    ($primitive:ty, $($other:ty),+) => {
        grid_index! { $primitive }
        grid_index! { $($other),+ }
    };
}

grid_index!(usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_creation() {
        assert_eq!(Grid::new(-10..10, -10..10, 10).width(), 20);
        assert_eq!(Grid::new(-10..=10, -10..10, 10).width(), 21);
        assert_eq!(Grid::new(-10..=10, -10..=10, 10).width(), 21);
    }
}
