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

    pub fn data(&self) -> &[T; N] {
        &self.data
    }

    pub fn slice<const START_IDX: usize, const LEN: usize>(&self) -> Vector<T, LEN> {
        let mut data = [self[0]; LEN];

        data.copy_from_slice(&self.data[START_IDX..START_IDX + LEN]);

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
    fn valid_size_slice_works() {
        let vector: Vector<i32, 3> = [0, 1, 2].into();

        let ones_2 = vector.slice::<0, 2>();
        assert_eq!(ones_2.data, [0, 1]);
        let ones_2 = vector.slice::<1, 2>();
        assert_eq!(ones_2.data, [1, 2]);

        let ones_1 = vector.slice::<0, 1>();
        assert_eq!(ones_1.data, [0]);
        let ones_1 = vector.slice::<1, 1>();
        assert_eq!(ones_1.data, [1]);
        let ones_1 = vector.slice::<2, 1>();
        assert_eq!(ones_1.data, [2]);

        let empty = vector.slice::<0, 0>();
        assert_eq!(empty.data, []);
        let empty = vector.slice::<1, 0>();
        assert_eq!(empty.data, []);
        let empty = vector.slice::<2, 0>();
        assert_eq!(empty.data, []);
    }

    #[rstest]
    #[should_panic]
    fn larger_slice_panics(ones: FourInts) {
        ones.slice::<0, 5>();
    }

    #[rstest]
    #[should_panic]
    fn out_of_bounds_slice_panics(ones: FourInts) {
        ones.slice::<3, 2>();
    }
}
