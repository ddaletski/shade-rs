use super::numeric::Float;
use super::float2::Float2;
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
#[shade_rs_core_macros::impl_color_permutations(rgb)]
pub struct Float3(Vector<f32, 3>);
