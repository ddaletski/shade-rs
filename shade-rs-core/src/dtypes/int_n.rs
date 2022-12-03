use super::numeric::Int;
use super::vector::Vector;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::From,
    derive_more::Index,
    derive_more::IndexMut,
)]
#[from(forward)]
#[shade_rs_core_macros::impl_color_permutations(rg)]
#[shade_rs_core_macros::impl_color_permutations(xy)]
pub struct Int2(Vector<Int, 2>);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::From,
    derive_more::Index,
    derive_more::IndexMut,
)]
#[from(forward)]
#[shade_rs_core_macros::impl_color_permutations(rgb)]
#[shade_rs_core_macros::impl_color_permutations(xyz)]
pub struct Int3(Vector<Int, 3>);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::From,
    derive_more::Index,
    derive_more::IndexMut,
)]
#[from(forward)]
#[shade_rs_core_macros::impl_color_permutations(rgba)]
#[shade_rs_core_macros::impl_color_permutations(xyzw)]
pub struct Int4(Vector<Int, 4>);
