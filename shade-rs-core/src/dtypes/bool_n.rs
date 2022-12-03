use super::numeric::Bool;
use super::vector::Vector;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    derive_more::From,
    derive_more::Index,
    derive_more::IndexMut,
)]
#[from(forward)]
#[shade_rs_core_macros::impl_color_permutations(rg)]
#[shade_rs_core_macros::impl_color_permutations(xy)]
pub struct Bool2(Vector<Bool, 2>);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    derive_more::From,
    derive_more::Index,
    derive_more::IndexMut,
)]
#[from(forward)]
#[shade_rs_core_macros::impl_color_permutations(rgb)]
#[shade_rs_core_macros::impl_color_permutations(xyz)]
pub struct Bool3(Vector<Bool, 3>);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    derive_more::From,
    derive_more::Index,
    derive_more::IndexMut,
)]
#[from(forward)]
#[shade_rs_core_macros::impl_color_permutations(rgba)]
#[shade_rs_core_macros::impl_color_permutations(xyzw)]
pub struct Bool4(Vector<Bool, 4>);
