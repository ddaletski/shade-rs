use std::ops::{Add, AddAssign, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where
    T: Sized,
    T: Clone,
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self { data: [value; N] }
    }

    pub fn arr(&self) -> &[T; N] {
        &self.data
    }

    pub fn slice<const K: usize>(&self) -> Vector<T, K> {
        let mut data = [self[0]; K];

        data.copy_from_slice(&self.data[..K]);

        Vector { data }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(arr: [T; N]) -> Self {
        Self { data: arr }
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: AddAssign,
    T: Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
        self.data[3] += rhs.data[3];
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: AddAssign,
    T: Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    type FourInts = Vector<i32, 4>;

    #[fixture]
    fn zeros() -> FourInts {
        Vector::new(0)
    }

    #[fixture]
    fn ones() -> FourInts {
        Vector::new(1)
    }

    #[fixture]
    fn twos() -> FourInts {
        Vector::new(2)
    }

    #[rstest]
    fn new_fills_with_value(ones: FourInts) {
        assert_eq!(ones.data, [1; 4]);
    }

    #[rstest]
    fn add_assign_works(mut ones: FourInts, twos: FourInts) {
        ones += twos;

        assert_eq!(ones.data, [3; 4]);
    }

    #[rstest]
    fn add_works(ones: FourInts, twos: FourInts) {
        let threes = ones + twos;

        assert_eq!(threes.data, [3; 4]);
    }

    #[rstest]
    fn valid_size_slice_works(ones: FourInts) {
        let ones_4 = ones.slice::<4>();
        assert_eq!(ones_4.data, [1; 4]);

        let ones_3 = ones.slice::<3>();
        assert_eq!(ones_3.data, [1; 3]);

        let ones_2 = ones.slice::<2>();
        assert_eq!(ones_2.data, [1; 2]);

        let ones_1 = ones.slice::<1>();
        assert_eq!(ones_1.data, [1; 1]);

        let empty = ones.slice::<0>();
        assert_eq!(empty.data, []);
    }

    #[rstest]
    #[should_panic]
    fn larger_slice_panics(ones: FourInts) {
        ones.slice::<5>();
    }
}
