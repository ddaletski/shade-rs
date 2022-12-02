use super::float2::Float2;
use super::float3::Float3;
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
#[shade_rs_core_macros::impl_rgba(f32, 4)]
pub struct Float4(Vector<f32, 4>);

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn f4() -> Float4 {
        [1.0, 2.0, 3.0, 4.0].into()
    }

    macro_rules! check_slice {
        ($val:expr, $slice:ident, $($value:literal),+) => {
            assert_eq!($val.$slice(), [$($value),*].into());
        };
    }

    #[rstest]
    fn single_getters(f4: Float4) {
        assert_eq!(f4.r(), 1.0);
        assert_eq!(f4.g(), 2.0);
        assert_eq!(f4.b(), 3.0);
        assert_eq!(f4.a(), 4.0);
    }

    #[rstest]
    fn double_getters(f4: Float4) {
        check_slice!(f4, rr, 1.0, 1.0);
        check_slice!(f4, rg, 1.0, 2.0);
        check_slice!(f4, rb, 1.0, 3.0);
        check_slice!(f4, ra, 1.0, 4.0);

        check_slice!(f4, gr, 2.0, 1.0);
        check_slice!(f4, gg, 2.0, 2.0);
        check_slice!(f4, gb, 2.0, 3.0);
        check_slice!(f4, ga, 2.0, 4.0);

        check_slice!(f4, br, 3.0, 1.0);
        check_slice!(f4, bg, 3.0, 2.0);
        check_slice!(f4, bb, 3.0, 3.0);
        check_slice!(f4, ba, 3.0, 4.0);

        check_slice!(f4, ar, 4.0, 1.0);
        check_slice!(f4, ag, 4.0, 2.0);
        check_slice!(f4, ab, 4.0, 3.0);
        check_slice!(f4, aa, 4.0, 4.0);
    }

    #[rstest]
    fn triple_getters(f4: Float4) {
        check_slice!(f4, rrr, 1.0, 1.0, 1.0);
        check_slice!(f4, rrg, 1.0, 1.0, 2.0);
        check_slice!(f4, rrb, 1.0, 1.0, 3.0);
        check_slice!(f4, rra, 1.0, 1.0, 4.0);

        check_slice!(f4, rgr, 1.0, 2.0, 1.0);
        check_slice!(f4, rgg, 1.0, 2.0, 2.0);
        check_slice!(f4, rgb, 1.0, 2.0, 3.0);
        check_slice!(f4, rga, 1.0, 2.0, 4.0);

        check_slice!(f4, rbr, 1.0, 3.0, 1.0);
        check_slice!(f4, rbg, 1.0, 3.0, 2.0);
        check_slice!(f4, rbb, 1.0, 3.0, 3.0);
        check_slice!(f4, rba, 1.0, 3.0, 4.0);

        check_slice!(f4, rar, 1.0, 4.0, 1.0);
        check_slice!(f4, rag, 1.0, 4.0, 2.0);
        check_slice!(f4, rab, 1.0, 4.0, 3.0);
        check_slice!(f4, raa, 1.0, 4.0, 4.0);

        ////////////////////////////////////

        check_slice!(f4, grr, 2.0, 1.0, 1.0);
        check_slice!(f4, grg, 2.0, 1.0, 2.0);
        check_slice!(f4, grb, 2.0, 1.0, 3.0);
        check_slice!(f4, gra, 2.0, 1.0, 4.0);

        check_slice!(f4, ggr, 2.0, 2.0, 1.0);
        check_slice!(f4, ggg, 2.0, 2.0, 2.0);
        check_slice!(f4, ggb, 2.0, 2.0, 3.0);
        check_slice!(f4, gga, 2.0, 2.0, 4.0);

        check_slice!(f4, gbr, 2.0, 3.0, 1.0);
        check_slice!(f4, gbg, 2.0, 3.0, 2.0);
        check_slice!(f4, gbb, 2.0, 3.0, 3.0);
        check_slice!(f4, gba, 2.0, 3.0, 4.0);

        check_slice!(f4, gar, 2.0, 4.0, 1.0);
        check_slice!(f4, gag, 2.0, 4.0, 2.0);
        check_slice!(f4, gab, 2.0, 4.0, 3.0);
        check_slice!(f4, gaa, 2.0, 4.0, 4.0);

        ////////////////////////////////////

        check_slice!(f4, brr, 3.0, 1.0, 1.0);
        check_slice!(f4, brg, 3.0, 1.0, 2.0);
        check_slice!(f4, brb, 3.0, 1.0, 3.0);
        check_slice!(f4, bra, 3.0, 1.0, 4.0);

        check_slice!(f4, bgr, 3.0, 2.0, 1.0);
        check_slice!(f4, bgg, 3.0, 2.0, 2.0);
        check_slice!(f4, bgb, 3.0, 2.0, 3.0);
        check_slice!(f4, bga, 3.0, 2.0, 4.0);

        check_slice!(f4, bbr, 3.0, 3.0, 1.0);
        check_slice!(f4, bbg, 3.0, 3.0, 2.0);
        check_slice!(f4, bbb, 3.0, 3.0, 3.0);
        check_slice!(f4, bba, 3.0, 3.0, 4.0);

        check_slice!(f4, bar, 3.0, 4.0, 1.0);
        check_slice!(f4, bag, 3.0, 4.0, 2.0);
        check_slice!(f4, bab, 3.0, 4.0, 3.0);
        check_slice!(f4, baa, 3.0, 4.0, 4.0);

        ////////////////////////////////////

        check_slice!(f4, arr, 4.0, 1.0, 1.0);
        check_slice!(f4, arg, 4.0, 1.0, 2.0);
        check_slice!(f4, arb, 4.0, 1.0, 3.0);
        check_slice!(f4, ara, 4.0, 1.0, 4.0);

        check_slice!(f4, agr, 4.0, 2.0, 1.0);
        check_slice!(f4, agg, 4.0, 2.0, 2.0);
        check_slice!(f4, agb, 4.0, 2.0, 3.0);
        check_slice!(f4, aga, 4.0, 2.0, 4.0);

        check_slice!(f4, abr, 4.0, 3.0, 1.0);
        check_slice!(f4, abg, 4.0, 3.0, 2.0);
        check_slice!(f4, abb, 4.0, 3.0, 3.0);
        check_slice!(f4, aba, 4.0, 3.0, 4.0);

        check_slice!(f4, aar, 4.0, 4.0, 1.0);
        check_slice!(f4, aag, 4.0, 4.0, 2.0);
        check_slice!(f4, aab, 4.0, 4.0, 3.0);
        check_slice!(f4, aaa, 4.0, 4.0, 4.0);
    }

    #[rstest]
    fn quadruple_getters(f4: Float4) {
        check_slice!(f4, rrrr, 1.0, 1.0, 1.0, 1.0);
        check_slice!(f4, rrrg, 1.0, 1.0, 1.0, 2.0);
        check_slice!(f4, rrrb, 1.0, 1.0, 1.0, 3.0);
        check_slice!(f4, rrra, 1.0, 1.0, 1.0, 4.0);

        check_slice!(f4, rrgr, 1.0, 1.0, 2.0, 1.0);
        check_slice!(f4, rrgg, 1.0, 1.0, 2.0, 2.0);
        check_slice!(f4, rrgb, 1.0, 1.0, 2.0, 3.0);
        check_slice!(f4, rrga, 1.0, 1.0, 2.0, 4.0);

        check_slice!(f4, rrbr, 1.0, 1.0, 3.0, 1.0);
        check_slice!(f4, rrbg, 1.0, 1.0, 3.0, 2.0);
        check_slice!(f4, rrbb, 1.0, 1.0, 3.0, 3.0);
        check_slice!(f4, rrba, 1.0, 1.0, 3.0, 4.0);

        check_slice!(f4, rrar, 1.0, 1.0, 4.0, 1.0);
        check_slice!(f4, rrag, 1.0, 1.0, 4.0, 2.0);
        check_slice!(f4, rrab, 1.0, 1.0, 4.0, 3.0);
        check_slice!(f4, rraa, 1.0, 1.0, 4.0, 4.0);

        check_slice!(f4, rgrr, 1.0, 2.0, 1.0, 1.0);
        check_slice!(f4, rgrg, 1.0, 2.0, 1.0, 2.0);
        check_slice!(f4, rgrb, 1.0, 2.0, 1.0, 3.0);
        check_slice!(f4, rgra, 1.0, 2.0, 1.0, 4.0);

        check_slice!(f4, rggr, 1.0, 2.0, 2.0, 1.0);
        check_slice!(f4, rggg, 1.0, 2.0, 2.0, 2.0);
        check_slice!(f4, rggb, 1.0, 2.0, 2.0, 3.0);
        check_slice!(f4, rgga, 1.0, 2.0, 2.0, 4.0);

        check_slice!(f4, rgbr, 1.0, 2.0, 3.0, 1.0);
        check_slice!(f4, rgbg, 1.0, 2.0, 3.0, 2.0);
        check_slice!(f4, rgbb, 1.0, 2.0, 3.0, 3.0);
        check_slice!(f4, rgba, 1.0, 2.0, 3.0, 4.0);

        check_slice!(f4, rgar, 1.0, 2.0, 4.0, 1.0);
        check_slice!(f4, rgag, 1.0, 2.0, 4.0, 2.0);
        check_slice!(f4, rgab, 1.0, 2.0, 4.0, 3.0);
        check_slice!(f4, rgaa, 1.0, 2.0, 4.0, 4.0);

        check_slice!(f4, rbrr, 1.0, 3.0, 1.0, 1.0);
        check_slice!(f4, rbrg, 1.0, 3.0, 1.0, 2.0);
        check_slice!(f4, rbrb, 1.0, 3.0, 1.0, 3.0);
        check_slice!(f4, rbra, 1.0, 3.0, 1.0, 4.0);

        check_slice!(f4, rbgr, 1.0, 3.0, 2.0, 1.0);
        check_slice!(f4, rbgg, 1.0, 3.0, 2.0, 2.0);
        check_slice!(f4, rbgb, 1.0, 3.0, 2.0, 3.0);
        check_slice!(f4, rbga, 1.0, 3.0, 2.0, 4.0);

        check_slice!(f4, rbbr, 1.0, 3.0, 3.0, 1.0);
        check_slice!(f4, rbbg, 1.0, 3.0, 3.0, 2.0);
        check_slice!(f4, rbbb, 1.0, 3.0, 3.0, 3.0);
        check_slice!(f4, rbba, 1.0, 3.0, 3.0, 4.0);

        check_slice!(f4, rbar, 1.0, 3.0, 4.0, 1.0);
        check_slice!(f4, rbag, 1.0, 3.0, 4.0, 2.0);
        check_slice!(f4, rbab, 1.0, 3.0, 4.0, 3.0);
        check_slice!(f4, rbaa, 1.0, 3.0, 4.0, 4.0);

        check_slice!(f4, rarr, 1.0, 4.0, 1.0, 1.0);
        check_slice!(f4, rarg, 1.0, 4.0, 1.0, 2.0);
        check_slice!(f4, rarb, 1.0, 4.0, 1.0, 3.0);
        check_slice!(f4, rara, 1.0, 4.0, 1.0, 4.0);

        check_slice!(f4, ragr, 1.0, 4.0, 2.0, 1.0);
        check_slice!(f4, ragg, 1.0, 4.0, 2.0, 2.0);
        check_slice!(f4, ragb, 1.0, 4.0, 2.0, 3.0);
        check_slice!(f4, raga, 1.0, 4.0, 2.0, 4.0);

        check_slice!(f4, rabr, 1.0, 4.0, 3.0, 1.0);
        check_slice!(f4, rabg, 1.0, 4.0, 3.0, 2.0);
        check_slice!(f4, rabb, 1.0, 4.0, 3.0, 3.0);
        check_slice!(f4, raba, 1.0, 4.0, 3.0, 4.0);

        check_slice!(f4, raar, 1.0, 4.0, 4.0, 1.0);
        check_slice!(f4, raag, 1.0, 4.0, 4.0, 2.0);
        check_slice!(f4, raab, 1.0, 4.0, 4.0, 3.0);
        check_slice!(f4, raaa, 1.0, 4.0, 4.0, 4.0);

        //////////////////////////////////////////

        check_slice!(f4, grrr, 2.0, 1.0, 1.0, 1.0);
        check_slice!(f4, grrg, 2.0, 1.0, 1.0, 2.0);
        check_slice!(f4, grrb, 2.0, 1.0, 1.0, 3.0);
        check_slice!(f4, grra, 2.0, 1.0, 1.0, 4.0);

        check_slice!(f4, grgr, 2.0, 1.0, 2.0, 1.0);
        check_slice!(f4, grgg, 2.0, 1.0, 2.0, 2.0);
        check_slice!(f4, grgb, 2.0, 1.0, 2.0, 3.0);
        check_slice!(f4, grga, 2.0, 1.0, 2.0, 4.0);

        check_slice!(f4, grbr, 2.0, 1.0, 3.0, 1.0);
        check_slice!(f4, grbg, 2.0, 1.0, 3.0, 2.0);
        check_slice!(f4, grbb, 2.0, 1.0, 3.0, 3.0);
        check_slice!(f4, grba, 2.0, 1.0, 3.0, 4.0);

        check_slice!(f4, grar, 2.0, 1.0, 4.0, 1.0);
        check_slice!(f4, grag, 2.0, 1.0, 4.0, 2.0);
        check_slice!(f4, grab, 2.0, 1.0, 4.0, 3.0);
        check_slice!(f4, graa, 2.0, 1.0, 4.0, 4.0);

        check_slice!(f4, ggrr, 2.0, 2.0, 1.0, 1.0);
        check_slice!(f4, ggrg, 2.0, 2.0, 1.0, 2.0);
        check_slice!(f4, ggrb, 2.0, 2.0, 1.0, 3.0);
        check_slice!(f4, ggra, 2.0, 2.0, 1.0, 4.0);

        check_slice!(f4, gggr, 2.0, 2.0, 2.0, 1.0);
        check_slice!(f4, gggg, 2.0, 2.0, 2.0, 2.0);
        check_slice!(f4, gggb, 2.0, 2.0, 2.0, 3.0);
        check_slice!(f4, ggga, 2.0, 2.0, 2.0, 4.0);

        check_slice!(f4, ggbr, 2.0, 2.0, 3.0, 1.0);
        check_slice!(f4, ggbg, 2.0, 2.0, 3.0, 2.0);
        check_slice!(f4, ggbb, 2.0, 2.0, 3.0, 3.0);
        check_slice!(f4, ggba, 2.0, 2.0, 3.0, 4.0);

        check_slice!(f4, ggar, 2.0, 2.0, 4.0, 1.0);
        check_slice!(f4, ggag, 2.0, 2.0, 4.0, 2.0);
        check_slice!(f4, ggab, 2.0, 2.0, 4.0, 3.0);
        check_slice!(f4, ggaa, 2.0, 2.0, 4.0, 4.0);

        check_slice!(f4, gbrr, 2.0, 3.0, 1.0, 1.0);
        check_slice!(f4, gbrg, 2.0, 3.0, 1.0, 2.0);
        check_slice!(f4, gbrb, 2.0, 3.0, 1.0, 3.0);
        check_slice!(f4, gbra, 2.0, 3.0, 1.0, 4.0);

        check_slice!(f4, gbgr, 2.0, 3.0, 2.0, 1.0);
        check_slice!(f4, gbgg, 2.0, 3.0, 2.0, 2.0);
        check_slice!(f4, gbgb, 2.0, 3.0, 2.0, 3.0);
        check_slice!(f4, gbga, 2.0, 3.0, 2.0, 4.0);

        check_slice!(f4, gbbr, 2.0, 3.0, 3.0, 1.0);
        check_slice!(f4, gbbg, 2.0, 3.0, 3.0, 2.0);
        check_slice!(f4, gbbb, 2.0, 3.0, 3.0, 3.0);
        check_slice!(f4, gbba, 2.0, 3.0, 3.0, 4.0);

        check_slice!(f4, gbar, 2.0, 3.0, 4.0, 1.0);
        check_slice!(f4, gbag, 2.0, 3.0, 4.0, 2.0);
        check_slice!(f4, gbab, 2.0, 3.0, 4.0, 3.0);
        check_slice!(f4, gbaa, 2.0, 3.0, 4.0, 4.0);

        check_slice!(f4, garr, 2.0, 4.0, 1.0, 1.0);
        check_slice!(f4, garg, 2.0, 4.0, 1.0, 2.0);
        check_slice!(f4, garb, 2.0, 4.0, 1.0, 3.0);
        check_slice!(f4, gara, 2.0, 4.0, 1.0, 4.0);

        check_slice!(f4, gagr, 2.0, 4.0, 2.0, 1.0);
        check_slice!(f4, gagg, 2.0, 4.0, 2.0, 2.0);
        check_slice!(f4, gagb, 2.0, 4.0, 2.0, 3.0);
        check_slice!(f4, gaga, 2.0, 4.0, 2.0, 4.0);

        check_slice!(f4, gabr, 2.0, 4.0, 3.0, 1.0);
        check_slice!(f4, gabg, 2.0, 4.0, 3.0, 2.0);
        check_slice!(f4, gabb, 2.0, 4.0, 3.0, 3.0);
        check_slice!(f4, gaba, 2.0, 4.0, 3.0, 4.0);

        check_slice!(f4, gaar, 2.0, 4.0, 4.0, 1.0);
        check_slice!(f4, gaag, 2.0, 4.0, 4.0, 2.0);
        check_slice!(f4, gaab, 2.0, 4.0, 4.0, 3.0);
        check_slice!(f4, gaaa, 2.0, 4.0, 4.0, 4.0);

        //////////////////////////////////////////

        check_slice!(f4, brrr, 3.0, 1.0, 1.0, 1.0);
        check_slice!(f4, brrg, 3.0, 1.0, 1.0, 2.0);
        check_slice!(f4, brrb, 3.0, 1.0, 1.0, 3.0);
        check_slice!(f4, brra, 3.0, 1.0, 1.0, 4.0);

        check_slice!(f4, brgr, 3.0, 1.0, 2.0, 1.0);
        check_slice!(f4, brgg, 3.0, 1.0, 2.0, 2.0);
        check_slice!(f4, brgb, 3.0, 1.0, 2.0, 3.0);
        check_slice!(f4, brga, 3.0, 1.0, 2.0, 4.0);

        check_slice!(f4, brbr, 3.0, 1.0, 3.0, 1.0);
        check_slice!(f4, brbg, 3.0, 1.0, 3.0, 2.0);
        check_slice!(f4, brbb, 3.0, 1.0, 3.0, 3.0);
        check_slice!(f4, brba, 3.0, 1.0, 3.0, 4.0);

        check_slice!(f4, brar, 3.0, 1.0, 4.0, 1.0);
        check_slice!(f4, brag, 3.0, 1.0, 4.0, 2.0);
        check_slice!(f4, brab, 3.0, 1.0, 4.0, 3.0);
        check_slice!(f4, braa, 3.0, 1.0, 4.0, 4.0);

        check_slice!(f4, bgrr, 3.0, 2.0, 1.0, 1.0);
        check_slice!(f4, bgrg, 3.0, 2.0, 1.0, 2.0);
        check_slice!(f4, bgrb, 3.0, 2.0, 1.0, 3.0);
        check_slice!(f4, bgra, 3.0, 2.0, 1.0, 4.0);

        check_slice!(f4, bggr, 3.0, 2.0, 2.0, 1.0);
        check_slice!(f4, bggg, 3.0, 2.0, 2.0, 2.0);
        check_slice!(f4, bggb, 3.0, 2.0, 2.0, 3.0);
        check_slice!(f4, bgga, 3.0, 2.0, 2.0, 4.0);

        check_slice!(f4, bgbr, 3.0, 2.0, 3.0, 1.0);
        check_slice!(f4, bgbg, 3.0, 2.0, 3.0, 2.0);
        check_slice!(f4, bgbb, 3.0, 2.0, 3.0, 3.0);
        check_slice!(f4, bgba, 3.0, 2.0, 3.0, 4.0);

        check_slice!(f4, bgar, 3.0, 2.0, 4.0, 1.0);
        check_slice!(f4, bgag, 3.0, 2.0, 4.0, 2.0);
        check_slice!(f4, bgab, 3.0, 2.0, 4.0, 3.0);
        check_slice!(f4, bgaa, 3.0, 2.0, 4.0, 4.0);

        check_slice!(f4, bbrr, 3.0, 3.0, 1.0, 1.0);
        check_slice!(f4, bbrg, 3.0, 3.0, 1.0, 2.0);
        check_slice!(f4, bbrb, 3.0, 3.0, 1.0, 3.0);
        check_slice!(f4, bbra, 3.0, 3.0, 1.0, 4.0);

        check_slice!(f4, bbgr, 3.0, 3.0, 2.0, 1.0);
        check_slice!(f4, bbgg, 3.0, 3.0, 2.0, 2.0);
        check_slice!(f4, bbgb, 3.0, 3.0, 2.0, 3.0);
        check_slice!(f4, bbga, 3.0, 3.0, 2.0, 4.0);

        check_slice!(f4, bbbr, 3.0, 3.0, 3.0, 1.0);
        check_slice!(f4, bbbg, 3.0, 3.0, 3.0, 2.0);
        check_slice!(f4, bbbb, 3.0, 3.0, 3.0, 3.0);
        check_slice!(f4, bbba, 3.0, 3.0, 3.0, 4.0);

        check_slice!(f4, bbar, 3.0, 3.0, 4.0, 1.0);
        check_slice!(f4, bbag, 3.0, 3.0, 4.0, 2.0);
        check_slice!(f4, bbab, 3.0, 3.0, 4.0, 3.0);
        check_slice!(f4, bbaa, 3.0, 3.0, 4.0, 4.0);

        check_slice!(f4, barr, 3.0, 4.0, 1.0, 1.0);
        check_slice!(f4, barg, 3.0, 4.0, 1.0, 2.0);
        check_slice!(f4, barb, 3.0, 4.0, 1.0, 3.0);
        check_slice!(f4, bara, 3.0, 4.0, 1.0, 4.0);

        check_slice!(f4, bagr, 3.0, 4.0, 2.0, 1.0);
        check_slice!(f4, bagg, 3.0, 4.0, 2.0, 2.0);
        check_slice!(f4, bagb, 3.0, 4.0, 2.0, 3.0);
        check_slice!(f4, baga, 3.0, 4.0, 2.0, 4.0);

        check_slice!(f4, babr, 3.0, 4.0, 3.0, 1.0);
        check_slice!(f4, babg, 3.0, 4.0, 3.0, 2.0);
        check_slice!(f4, babb, 3.0, 4.0, 3.0, 3.0);
        check_slice!(f4, baba, 3.0, 4.0, 3.0, 4.0);

        check_slice!(f4, baar, 3.0, 4.0, 4.0, 1.0);
        check_slice!(f4, baag, 3.0, 4.0, 4.0, 2.0);
        check_slice!(f4, baab, 3.0, 4.0, 4.0, 3.0);
        check_slice!(f4, baaa, 3.0, 4.0, 4.0, 4.0);

        //////////////////////////////////////////

        check_slice!(f4, arrr, 4.0, 1.0, 1.0, 1.0);
        check_slice!(f4, arrg, 4.0, 1.0, 1.0, 2.0);
        check_slice!(f4, arrb, 4.0, 1.0, 1.0, 3.0);
        check_slice!(f4, arra, 4.0, 1.0, 1.0, 4.0);

        check_slice!(f4, argr, 4.0, 1.0, 2.0, 1.0);
        check_slice!(f4, argg, 4.0, 1.0, 2.0, 2.0);
        check_slice!(f4, argb, 4.0, 1.0, 2.0, 3.0);
        check_slice!(f4, arga, 4.0, 1.0, 2.0, 4.0);

        check_slice!(f4, arbr, 4.0, 1.0, 3.0, 1.0);
        check_slice!(f4, arbg, 4.0, 1.0, 3.0, 2.0);
        check_slice!(f4, arbb, 4.0, 1.0, 3.0, 3.0);
        check_slice!(f4, arba, 4.0, 1.0, 3.0, 4.0);

        check_slice!(f4, arar, 4.0, 1.0, 4.0, 1.0);
        check_slice!(f4, arag, 4.0, 1.0, 4.0, 2.0);
        check_slice!(f4, arab, 4.0, 1.0, 4.0, 3.0);
        check_slice!(f4, araa, 4.0, 1.0, 4.0, 4.0);

        check_slice!(f4, agrr, 4.0, 2.0, 1.0, 1.0);
        check_slice!(f4, agrg, 4.0, 2.0, 1.0, 2.0);
        check_slice!(f4, agrb, 4.0, 2.0, 1.0, 3.0);
        check_slice!(f4, agra, 4.0, 2.0, 1.0, 4.0);

        check_slice!(f4, aggr, 4.0, 2.0, 2.0, 1.0);
        check_slice!(f4, aggg, 4.0, 2.0, 2.0, 2.0);
        check_slice!(f4, aggb, 4.0, 2.0, 2.0, 3.0);
        check_slice!(f4, agga, 4.0, 2.0, 2.0, 4.0);

        check_slice!(f4, agbr, 4.0, 2.0, 3.0, 1.0);
        check_slice!(f4, agbg, 4.0, 2.0, 3.0, 2.0);
        check_slice!(f4, agbb, 4.0, 2.0, 3.0, 3.0);
        check_slice!(f4, agba, 4.0, 2.0, 3.0, 4.0);

        check_slice!(f4, agar, 4.0, 2.0, 4.0, 1.0);
        check_slice!(f4, agag, 4.0, 2.0, 4.0, 2.0);
        check_slice!(f4, agab, 4.0, 2.0, 4.0, 3.0);
        check_slice!(f4, agaa, 4.0, 2.0, 4.0, 4.0);

        check_slice!(f4, abrr, 4.0, 3.0, 1.0, 1.0);
        check_slice!(f4, abrg, 4.0, 3.0, 1.0, 2.0);
        check_slice!(f4, abrb, 4.0, 3.0, 1.0, 3.0);
        check_slice!(f4, abra, 4.0, 3.0, 1.0, 4.0);

        check_slice!(f4, abgr, 4.0, 3.0, 2.0, 1.0);
        check_slice!(f4, abgg, 4.0, 3.0, 2.0, 2.0);
        check_slice!(f4, abgb, 4.0, 3.0, 2.0, 3.0);
        check_slice!(f4, abga, 4.0, 3.0, 2.0, 4.0);

        check_slice!(f4, abbr, 4.0, 3.0, 3.0, 1.0);
        check_slice!(f4, abbg, 4.0, 3.0, 3.0, 2.0);
        check_slice!(f4, abbb, 4.0, 3.0, 3.0, 3.0);
        check_slice!(f4, abba, 4.0, 3.0, 3.0, 4.0);

        check_slice!(f4, abar, 4.0, 3.0, 4.0, 1.0);
        check_slice!(f4, abag, 4.0, 3.0, 4.0, 2.0);
        check_slice!(f4, abab, 4.0, 3.0, 4.0, 3.0);
        check_slice!(f4, abaa, 4.0, 3.0, 4.0, 4.0);

        check_slice!(f4, aarr, 4.0, 4.0, 1.0, 1.0);
        check_slice!(f4, aarg, 4.0, 4.0, 1.0, 2.0);
        check_slice!(f4, aarb, 4.0, 4.0, 1.0, 3.0);
        check_slice!(f4, aara, 4.0, 4.0, 1.0, 4.0);

        check_slice!(f4, aagr, 4.0, 4.0, 2.0, 1.0);
        check_slice!(f4, aagg, 4.0, 4.0, 2.0, 2.0);
        check_slice!(f4, aagb, 4.0, 4.0, 2.0, 3.0);
        check_slice!(f4, aaga, 4.0, 4.0, 2.0, 4.0);

        check_slice!(f4, aabr, 4.0, 4.0, 3.0, 1.0);
        check_slice!(f4, aabg, 4.0, 4.0, 3.0, 2.0);
        check_slice!(f4, aabb, 4.0, 4.0, 3.0, 3.0);
        check_slice!(f4, aaba, 4.0, 4.0, 3.0, 4.0);

        check_slice!(f4, aaar, 4.0, 4.0, 4.0, 1.0);
        check_slice!(f4, aaag, 4.0, 4.0, 4.0, 2.0);
        check_slice!(f4, aaab, 4.0, 4.0, 4.0, 3.0);
        check_slice!(f4, aaaa, 4.0, 4.0, 4.0, 4.0);
    }
}
