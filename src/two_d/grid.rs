use crate::two_d::PointLike;
use std::ops::{Add, Range, Sub};

pub struct Grid<T, Num = usize>
where
    Num: Add<Output = Num> + Sub<Output = Num> + Ord,
{
    x_offset: Num,
    y_offset: Num,

    default: dyn Fn() -> T,
}

impl<T, N> Grid<T, N>
where
    N: Add<Output = N> + Sub<Output = N> + Ord,
{
    pub fn set(&mut self, point: &dyn PointLike<N>, value: T) -> T {
        todo!()
    }
}

pub struct Set;
pub struct Unset;

pub struct GridBuilder<T, Num, DefaultFn>
where
    Num: Add<Output = Num> + Sub<Output = Num> + Ord, {}

impl<T, Num, DefaultFn> GridBuilder<T, Num, DefaultFn> {
    fn new()
}
