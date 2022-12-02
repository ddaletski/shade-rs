
                #[test]
                fn test_Float4__r() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_r(0.0);
                    assert_eq!(vector.r(), 0.0);
                }
            

                #[test]
                fn test_Float4__g() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_g(1.0);
                    assert_eq!(vector.g(), 1.0);
                }
            

                #[test]
                fn test_Float4__b() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_b(2.0);
                    assert_eq!(vector.b(), 2.0);
                }
            

                #[test]
                fn test_Float4__a() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_a(3.0);
                    assert_eq!(vector.a(), 3.0);
                }
            

                #[test]
                fn test_Float4__rr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rr([0.0, 0.0].into());
                    assert_eq!(vector.rr(), [0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rg([0.0, 1.0].into());
                    assert_eq!(vector.rg(), [0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rb([0.0, 2.0].into());
                    assert_eq!(vector.rb(), [0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__ra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ra([0.0, 3.0].into());
                    assert_eq!(vector.ra(), [0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gr([1.0, 0.0].into());
                    assert_eq!(vector.gr(), [1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gg([1.0, 1.0].into());
                    assert_eq!(vector.gg(), [1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gb([1.0, 2.0].into());
                    assert_eq!(vector.gb(), [1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__ga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ga([1.0, 3.0].into());
                    assert_eq!(vector.ga(), [1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__br() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_br([2.0, 0.0].into());
                    assert_eq!(vector.br(), [2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bg([2.0, 1.0].into());
                    assert_eq!(vector.bg(), [2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bb([2.0, 2.0].into());
                    assert_eq!(vector.bb(), [2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__ba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ba([2.0, 3.0].into());
                    assert_eq!(vector.ba(), [2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__ar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ar([3.0, 0.0].into());
                    assert_eq!(vector.ar(), [3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__ag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ag([3.0, 1.0].into());
                    assert_eq!(vector.ag(), [3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__ab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ab([3.0, 2.0].into());
                    assert_eq!(vector.ab(), [3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__aa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aa([3.0, 3.0].into());
                    assert_eq!(vector.aa(), [3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrr([0.0, 0.0, 0.0].into());
                    assert_eq!(vector.rrr(), [0.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrg([0.0, 0.0, 1.0].into());
                    assert_eq!(vector.rrg(), [0.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrb([0.0, 0.0, 2.0].into());
                    assert_eq!(vector.rrb(), [0.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rra([0.0, 0.0, 3.0].into());
                    assert_eq!(vector.rra(), [0.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgr([0.0, 1.0, 0.0].into());
                    assert_eq!(vector.rgr(), [0.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgg([0.0, 1.0, 1.0].into());
                    assert_eq!(vector.rgg(), [0.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgb([0.0, 1.0, 2.0].into());
                    assert_eq!(vector.rgb(), [0.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rga([0.0, 1.0, 3.0].into());
                    assert_eq!(vector.rga(), [0.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbr([0.0, 2.0, 0.0].into());
                    assert_eq!(vector.rbr(), [0.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbg([0.0, 2.0, 1.0].into());
                    assert_eq!(vector.rbg(), [0.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbb([0.0, 2.0, 2.0].into());
                    assert_eq!(vector.rbb(), [0.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rba([0.0, 2.0, 3.0].into());
                    assert_eq!(vector.rba(), [0.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rar([0.0, 3.0, 0.0].into());
                    assert_eq!(vector.rar(), [0.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rag([0.0, 3.0, 1.0].into());
                    assert_eq!(vector.rag(), [0.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rab([0.0, 3.0, 2.0].into());
                    assert_eq!(vector.rab(), [0.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__raa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_raa([0.0, 3.0, 3.0].into());
                    assert_eq!(vector.raa(), [0.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__grr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grr([1.0, 0.0, 0.0].into());
                    assert_eq!(vector.grr(), [1.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__grg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grg([1.0, 0.0, 1.0].into());
                    assert_eq!(vector.grg(), [1.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__grb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grb([1.0, 0.0, 2.0].into());
                    assert_eq!(vector.grb(), [1.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gra([1.0, 0.0, 3.0].into());
                    assert_eq!(vector.gra(), [1.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__ggr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggr([1.0, 1.0, 0.0].into());
                    assert_eq!(vector.ggr(), [1.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__ggg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggg([1.0, 1.0, 1.0].into());
                    assert_eq!(vector.ggg(), [1.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__ggb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggb([1.0, 1.0, 2.0].into());
                    assert_eq!(vector.ggb(), [1.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gga([1.0, 1.0, 3.0].into());
                    assert_eq!(vector.gga(), [1.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbr([1.0, 2.0, 0.0].into());
                    assert_eq!(vector.gbr(), [1.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbg([1.0, 2.0, 1.0].into());
                    assert_eq!(vector.gbg(), [1.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbb([1.0, 2.0, 2.0].into());
                    assert_eq!(vector.gbb(), [1.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gba([1.0, 2.0, 3.0].into());
                    assert_eq!(vector.gba(), [1.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gar([1.0, 3.0, 0.0].into());
                    assert_eq!(vector.gar(), [1.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gag([1.0, 3.0, 1.0].into());
                    assert_eq!(vector.gag(), [1.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gab([1.0, 3.0, 2.0].into());
                    assert_eq!(vector.gab(), [1.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gaa([1.0, 3.0, 3.0].into());
                    assert_eq!(vector.gaa(), [1.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__brr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brr([2.0, 0.0, 0.0].into());
                    assert_eq!(vector.brr(), [2.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__brg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brg([2.0, 0.0, 1.0].into());
                    assert_eq!(vector.brg(), [2.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__brb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brb([2.0, 0.0, 2.0].into());
                    assert_eq!(vector.brb(), [2.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bra([2.0, 0.0, 3.0].into());
                    assert_eq!(vector.bra(), [2.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgr([2.0, 1.0, 0.0].into());
                    assert_eq!(vector.bgr(), [2.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgg([2.0, 1.0, 1.0].into());
                    assert_eq!(vector.bgg(), [2.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgb([2.0, 1.0, 2.0].into());
                    assert_eq!(vector.bgb(), [2.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bga([2.0, 1.0, 3.0].into());
                    assert_eq!(vector.bga(), [2.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbr([2.0, 2.0, 0.0].into());
                    assert_eq!(vector.bbr(), [2.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbg([2.0, 2.0, 1.0].into());
                    assert_eq!(vector.bbg(), [2.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbb([2.0, 2.0, 2.0].into());
                    assert_eq!(vector.bbb(), [2.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bba([2.0, 2.0, 3.0].into());
                    assert_eq!(vector.bba(), [2.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bar([2.0, 3.0, 0.0].into());
                    assert_eq!(vector.bar(), [2.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bag([2.0, 3.0, 1.0].into());
                    assert_eq!(vector.bag(), [2.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bab([2.0, 3.0, 2.0].into());
                    assert_eq!(vector.bab(), [2.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__baa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_baa([2.0, 3.0, 3.0].into());
                    assert_eq!(vector.baa(), [2.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__arr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arr([3.0, 0.0, 0.0].into());
                    assert_eq!(vector.arr(), [3.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__arg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arg([3.0, 0.0, 1.0].into());
                    assert_eq!(vector.arg(), [3.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__arb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arb([3.0, 0.0, 2.0].into());
                    assert_eq!(vector.arb(), [3.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__ara() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ara([3.0, 0.0, 3.0].into());
                    assert_eq!(vector.ara(), [3.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__agr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agr([3.0, 1.0, 0.0].into());
                    assert_eq!(vector.agr(), [3.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__agg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agg([3.0, 1.0, 1.0].into());
                    assert_eq!(vector.agg(), [3.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__agb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agb([3.0, 1.0, 2.0].into());
                    assert_eq!(vector.agb(), [3.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__aga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aga([3.0, 1.0, 3.0].into());
                    assert_eq!(vector.aga(), [3.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__abr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abr([3.0, 2.0, 0.0].into());
                    assert_eq!(vector.abr(), [3.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__abg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abg([3.0, 2.0, 1.0].into());
                    assert_eq!(vector.abg(), [3.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__abb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abb([3.0, 2.0, 2.0].into());
                    assert_eq!(vector.abb(), [3.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__aba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aba([3.0, 2.0, 3.0].into());
                    assert_eq!(vector.aba(), [3.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__aar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aar([3.0, 3.0, 0.0].into());
                    assert_eq!(vector.aar(), [3.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__aag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aag([3.0, 3.0, 1.0].into());
                    assert_eq!(vector.aag(), [3.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__aab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aab([3.0, 3.0, 2.0].into());
                    assert_eq!(vector.aab(), [3.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__aaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aaa([3.0, 3.0, 3.0].into());
                    assert_eq!(vector.aaa(), [3.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rrrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrrr([0.0, 0.0, 0.0, 0.0].into());
                    assert_eq!(vector.rrrr(), [0.0, 0.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rrrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrrg([0.0, 0.0, 0.0, 1.0].into());
                    assert_eq!(vector.rrrg(), [0.0, 0.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rrrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrrb([0.0, 0.0, 0.0, 2.0].into());
                    assert_eq!(vector.rrrb(), [0.0, 0.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rrra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrra([0.0, 0.0, 0.0, 3.0].into());
                    assert_eq!(vector.rrra(), [0.0, 0.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rrgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrgr([0.0, 0.0, 1.0, 0.0].into());
                    assert_eq!(vector.rrgr(), [0.0, 0.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rrgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrgg([0.0, 0.0, 1.0, 1.0].into());
                    assert_eq!(vector.rrgg(), [0.0, 0.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rrgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrgb([0.0, 0.0, 1.0, 2.0].into());
                    assert_eq!(vector.rrgb(), [0.0, 0.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rrga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrga([0.0, 0.0, 1.0, 3.0].into());
                    assert_eq!(vector.rrga(), [0.0, 0.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rrbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrbr([0.0, 0.0, 2.0, 0.0].into());
                    assert_eq!(vector.rrbr(), [0.0, 0.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rrbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrbg([0.0, 0.0, 2.0, 1.0].into());
                    assert_eq!(vector.rrbg(), [0.0, 0.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rrbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrbb([0.0, 0.0, 2.0, 2.0].into());
                    assert_eq!(vector.rrbb(), [0.0, 0.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rrba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrba([0.0, 0.0, 2.0, 3.0].into());
                    assert_eq!(vector.rrba(), [0.0, 0.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rrar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrar([0.0, 0.0, 3.0, 0.0].into());
                    assert_eq!(vector.rrar(), [0.0, 0.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rrag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrag([0.0, 0.0, 3.0, 1.0].into());
                    assert_eq!(vector.rrag(), [0.0, 0.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rrab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rrab([0.0, 0.0, 3.0, 2.0].into());
                    assert_eq!(vector.rrab(), [0.0, 0.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rraa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rraa([0.0, 0.0, 3.0, 3.0].into());
                    assert_eq!(vector.rraa(), [0.0, 0.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rgrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgrr([0.0, 1.0, 0.0, 0.0].into());
                    assert_eq!(vector.rgrr(), [0.0, 1.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rgrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgrg([0.0, 1.0, 0.0, 1.0].into());
                    assert_eq!(vector.rgrg(), [0.0, 1.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rgrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgrb([0.0, 1.0, 0.0, 2.0].into());
                    assert_eq!(vector.rgrb(), [0.0, 1.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rgra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgra([0.0, 1.0, 0.0, 3.0].into());
                    assert_eq!(vector.rgra(), [0.0, 1.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rggr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rggr([0.0, 1.0, 1.0, 0.0].into());
                    assert_eq!(vector.rggr(), [0.0, 1.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rggg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rggg([0.0, 1.0, 1.0, 1.0].into());
                    assert_eq!(vector.rggg(), [0.0, 1.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rggb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rggb([0.0, 1.0, 1.0, 2.0].into());
                    assert_eq!(vector.rggb(), [0.0, 1.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rgga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgga([0.0, 1.0, 1.0, 3.0].into());
                    assert_eq!(vector.rgga(), [0.0, 1.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rgbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgbr([0.0, 1.0, 2.0, 0.0].into());
                    assert_eq!(vector.rgbr(), [0.0, 1.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rgbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgbg([0.0, 1.0, 2.0, 1.0].into());
                    assert_eq!(vector.rgbg(), [0.0, 1.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rgbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgbb([0.0, 1.0, 2.0, 2.0].into());
                    assert_eq!(vector.rgbb(), [0.0, 1.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rgba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgba([0.0, 1.0, 2.0, 3.0].into());
                    assert_eq!(vector.rgba(), [0.0, 1.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rgar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgar([0.0, 1.0, 3.0, 0.0].into());
                    assert_eq!(vector.rgar(), [0.0, 1.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rgag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgag([0.0, 1.0, 3.0, 1.0].into());
                    assert_eq!(vector.rgag(), [0.0, 1.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rgab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgab([0.0, 1.0, 3.0, 2.0].into());
                    assert_eq!(vector.rgab(), [0.0, 1.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rgaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rgaa([0.0, 1.0, 3.0, 3.0].into());
                    assert_eq!(vector.rgaa(), [0.0, 1.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rbrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbrr([0.0, 2.0, 0.0, 0.0].into());
                    assert_eq!(vector.rbrr(), [0.0, 2.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rbrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbrg([0.0, 2.0, 0.0, 1.0].into());
                    assert_eq!(vector.rbrg(), [0.0, 2.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rbrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbrb([0.0, 2.0, 0.0, 2.0].into());
                    assert_eq!(vector.rbrb(), [0.0, 2.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rbra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbra([0.0, 2.0, 0.0, 3.0].into());
                    assert_eq!(vector.rbra(), [0.0, 2.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rbgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbgr([0.0, 2.0, 1.0, 0.0].into());
                    assert_eq!(vector.rbgr(), [0.0, 2.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rbgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbgg([0.0, 2.0, 1.0, 1.0].into());
                    assert_eq!(vector.rbgg(), [0.0, 2.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rbgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbgb([0.0, 2.0, 1.0, 2.0].into());
                    assert_eq!(vector.rbgb(), [0.0, 2.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rbga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbga([0.0, 2.0, 1.0, 3.0].into());
                    assert_eq!(vector.rbga(), [0.0, 2.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rbbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbbr([0.0, 2.0, 2.0, 0.0].into());
                    assert_eq!(vector.rbbr(), [0.0, 2.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rbbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbbg([0.0, 2.0, 2.0, 1.0].into());
                    assert_eq!(vector.rbbg(), [0.0, 2.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rbbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbbb([0.0, 2.0, 2.0, 2.0].into());
                    assert_eq!(vector.rbbb(), [0.0, 2.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rbba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbba([0.0, 2.0, 2.0, 3.0].into());
                    assert_eq!(vector.rbba(), [0.0, 2.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rbar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbar([0.0, 2.0, 3.0, 0.0].into());
                    assert_eq!(vector.rbar(), [0.0, 2.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rbag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbag([0.0, 2.0, 3.0, 1.0].into());
                    assert_eq!(vector.rbag(), [0.0, 2.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rbab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbab([0.0, 2.0, 3.0, 2.0].into());
                    assert_eq!(vector.rbab(), [0.0, 2.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rbaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rbaa([0.0, 2.0, 3.0, 3.0].into());
                    assert_eq!(vector.rbaa(), [0.0, 2.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rarr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rarr([0.0, 3.0, 0.0, 0.0].into());
                    assert_eq!(vector.rarr(), [0.0, 3.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rarg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rarg([0.0, 3.0, 0.0, 1.0].into());
                    assert_eq!(vector.rarg(), [0.0, 3.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rarb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rarb([0.0, 3.0, 0.0, 2.0].into());
                    assert_eq!(vector.rarb(), [0.0, 3.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__rara() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rara([0.0, 3.0, 0.0, 3.0].into());
                    assert_eq!(vector.rara(), [0.0, 3.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__ragr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ragr([0.0, 3.0, 1.0, 0.0].into());
                    assert_eq!(vector.ragr(), [0.0, 3.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__ragg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ragg([0.0, 3.0, 1.0, 1.0].into());
                    assert_eq!(vector.ragg(), [0.0, 3.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__ragb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ragb([0.0, 3.0, 1.0, 2.0].into());
                    assert_eq!(vector.ragb(), [0.0, 3.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__raga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_raga([0.0, 3.0, 1.0, 3.0].into());
                    assert_eq!(vector.raga(), [0.0, 3.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__rabr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rabr([0.0, 3.0, 2.0, 0.0].into());
                    assert_eq!(vector.rabr(), [0.0, 3.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__rabg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rabg([0.0, 3.0, 2.0, 1.0].into());
                    assert_eq!(vector.rabg(), [0.0, 3.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__rabb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_rabb([0.0, 3.0, 2.0, 2.0].into());
                    assert_eq!(vector.rabb(), [0.0, 3.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__raba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_raba([0.0, 3.0, 2.0, 3.0].into());
                    assert_eq!(vector.raba(), [0.0, 3.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__raar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_raar([0.0, 3.0, 3.0, 0.0].into());
                    assert_eq!(vector.raar(), [0.0, 3.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__raag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_raag([0.0, 3.0, 3.0, 1.0].into());
                    assert_eq!(vector.raag(), [0.0, 3.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__raab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_raab([0.0, 3.0, 3.0, 2.0].into());
                    assert_eq!(vector.raab(), [0.0, 3.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__raaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_raaa([0.0, 3.0, 3.0, 3.0].into());
                    assert_eq!(vector.raaa(), [0.0, 3.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__grrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grrr([1.0, 0.0, 0.0, 0.0].into());
                    assert_eq!(vector.grrr(), [1.0, 0.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__grrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grrg([1.0, 0.0, 0.0, 1.0].into());
                    assert_eq!(vector.grrg(), [1.0, 0.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__grrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grrb([1.0, 0.0, 0.0, 2.0].into());
                    assert_eq!(vector.grrb(), [1.0, 0.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__grra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grra([1.0, 0.0, 0.0, 3.0].into());
                    assert_eq!(vector.grra(), [1.0, 0.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__grgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grgr([1.0, 0.0, 1.0, 0.0].into());
                    assert_eq!(vector.grgr(), [1.0, 0.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__grgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grgg([1.0, 0.0, 1.0, 1.0].into());
                    assert_eq!(vector.grgg(), [1.0, 0.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__grgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grgb([1.0, 0.0, 1.0, 2.0].into());
                    assert_eq!(vector.grgb(), [1.0, 0.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__grga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grga([1.0, 0.0, 1.0, 3.0].into());
                    assert_eq!(vector.grga(), [1.0, 0.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__grbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grbr([1.0, 0.0, 2.0, 0.0].into());
                    assert_eq!(vector.grbr(), [1.0, 0.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__grbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grbg([1.0, 0.0, 2.0, 1.0].into());
                    assert_eq!(vector.grbg(), [1.0, 0.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__grbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grbb([1.0, 0.0, 2.0, 2.0].into());
                    assert_eq!(vector.grbb(), [1.0, 0.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__grba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grba([1.0, 0.0, 2.0, 3.0].into());
                    assert_eq!(vector.grba(), [1.0, 0.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__grar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grar([1.0, 0.0, 3.0, 0.0].into());
                    assert_eq!(vector.grar(), [1.0, 0.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__grag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grag([1.0, 0.0, 3.0, 1.0].into());
                    assert_eq!(vector.grag(), [1.0, 0.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__grab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_grab([1.0, 0.0, 3.0, 2.0].into());
                    assert_eq!(vector.grab(), [1.0, 0.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__graa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_graa([1.0, 0.0, 3.0, 3.0].into());
                    assert_eq!(vector.graa(), [1.0, 0.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__ggrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggrr([1.0, 1.0, 0.0, 0.0].into());
                    assert_eq!(vector.ggrr(), [1.0, 1.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__ggrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggrg([1.0, 1.0, 0.0, 1.0].into());
                    assert_eq!(vector.ggrg(), [1.0, 1.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__ggrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggrb([1.0, 1.0, 0.0, 2.0].into());
                    assert_eq!(vector.ggrb(), [1.0, 1.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__ggra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggra([1.0, 1.0, 0.0, 3.0].into());
                    assert_eq!(vector.ggra(), [1.0, 1.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gggr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gggr([1.0, 1.0, 1.0, 0.0].into());
                    assert_eq!(vector.gggr(), [1.0, 1.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gggg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gggg([1.0, 1.0, 1.0, 1.0].into());
                    assert_eq!(vector.gggg(), [1.0, 1.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gggb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gggb([1.0, 1.0, 1.0, 2.0].into());
                    assert_eq!(vector.gggb(), [1.0, 1.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__ggga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggga([1.0, 1.0, 1.0, 3.0].into());
                    assert_eq!(vector.ggga(), [1.0, 1.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__ggbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggbr([1.0, 1.0, 2.0, 0.0].into());
                    assert_eq!(vector.ggbr(), [1.0, 1.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__ggbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggbg([1.0, 1.0, 2.0, 1.0].into());
                    assert_eq!(vector.ggbg(), [1.0, 1.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__ggbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggbb([1.0, 1.0, 2.0, 2.0].into());
                    assert_eq!(vector.ggbb(), [1.0, 1.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__ggba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggba([1.0, 1.0, 2.0, 3.0].into());
                    assert_eq!(vector.ggba(), [1.0, 1.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__ggar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggar([1.0, 1.0, 3.0, 0.0].into());
                    assert_eq!(vector.ggar(), [1.0, 1.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__ggag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggag([1.0, 1.0, 3.0, 1.0].into());
                    assert_eq!(vector.ggag(), [1.0, 1.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__ggab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggab([1.0, 1.0, 3.0, 2.0].into());
                    assert_eq!(vector.ggab(), [1.0, 1.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__ggaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_ggaa([1.0, 1.0, 3.0, 3.0].into());
                    assert_eq!(vector.ggaa(), [1.0, 1.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gbrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbrr([1.0, 2.0, 0.0, 0.0].into());
                    assert_eq!(vector.gbrr(), [1.0, 2.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gbrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbrg([1.0, 2.0, 0.0, 1.0].into());
                    assert_eq!(vector.gbrg(), [1.0, 2.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gbrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbrb([1.0, 2.0, 0.0, 2.0].into());
                    assert_eq!(vector.gbrb(), [1.0, 2.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gbra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbra([1.0, 2.0, 0.0, 3.0].into());
                    assert_eq!(vector.gbra(), [1.0, 2.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gbgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbgr([1.0, 2.0, 1.0, 0.0].into());
                    assert_eq!(vector.gbgr(), [1.0, 2.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gbgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbgg([1.0, 2.0, 1.0, 1.0].into());
                    assert_eq!(vector.gbgg(), [1.0, 2.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gbgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbgb([1.0, 2.0, 1.0, 2.0].into());
                    assert_eq!(vector.gbgb(), [1.0, 2.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gbga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbga([1.0, 2.0, 1.0, 3.0].into());
                    assert_eq!(vector.gbga(), [1.0, 2.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gbbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbbr([1.0, 2.0, 2.0, 0.0].into());
                    assert_eq!(vector.gbbr(), [1.0, 2.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gbbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbbg([1.0, 2.0, 2.0, 1.0].into());
                    assert_eq!(vector.gbbg(), [1.0, 2.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gbbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbbb([1.0, 2.0, 2.0, 2.0].into());
                    assert_eq!(vector.gbbb(), [1.0, 2.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gbba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbba([1.0, 2.0, 2.0, 3.0].into());
                    assert_eq!(vector.gbba(), [1.0, 2.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gbar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbar([1.0, 2.0, 3.0, 0.0].into());
                    assert_eq!(vector.gbar(), [1.0, 2.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gbag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbag([1.0, 2.0, 3.0, 1.0].into());
                    assert_eq!(vector.gbag(), [1.0, 2.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gbab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbab([1.0, 2.0, 3.0, 2.0].into());
                    assert_eq!(vector.gbab(), [1.0, 2.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gbaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gbaa([1.0, 2.0, 3.0, 3.0].into());
                    assert_eq!(vector.gbaa(), [1.0, 2.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__garr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_garr([1.0, 3.0, 0.0, 0.0].into());
                    assert_eq!(vector.garr(), [1.0, 3.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__garg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_garg([1.0, 3.0, 0.0, 1.0].into());
                    assert_eq!(vector.garg(), [1.0, 3.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__garb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_garb([1.0, 3.0, 0.0, 2.0].into());
                    assert_eq!(vector.garb(), [1.0, 3.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gara() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gara([1.0, 3.0, 0.0, 3.0].into());
                    assert_eq!(vector.gara(), [1.0, 3.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gagr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gagr([1.0, 3.0, 1.0, 0.0].into());
                    assert_eq!(vector.gagr(), [1.0, 3.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gagg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gagg([1.0, 3.0, 1.0, 1.0].into());
                    assert_eq!(vector.gagg(), [1.0, 3.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gagb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gagb([1.0, 3.0, 1.0, 2.0].into());
                    assert_eq!(vector.gagb(), [1.0, 3.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gaga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gaga([1.0, 3.0, 1.0, 3.0].into());
                    assert_eq!(vector.gaga(), [1.0, 3.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gabr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gabr([1.0, 3.0, 2.0, 0.0].into());
                    assert_eq!(vector.gabr(), [1.0, 3.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gabg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gabg([1.0, 3.0, 2.0, 1.0].into());
                    assert_eq!(vector.gabg(), [1.0, 3.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gabb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gabb([1.0, 3.0, 2.0, 2.0].into());
                    assert_eq!(vector.gabb(), [1.0, 3.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gaba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gaba([1.0, 3.0, 2.0, 3.0].into());
                    assert_eq!(vector.gaba(), [1.0, 3.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__gaar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gaar([1.0, 3.0, 3.0, 0.0].into());
                    assert_eq!(vector.gaar(), [1.0, 3.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__gaag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gaag([1.0, 3.0, 3.0, 1.0].into());
                    assert_eq!(vector.gaag(), [1.0, 3.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__gaab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gaab([1.0, 3.0, 3.0, 2.0].into());
                    assert_eq!(vector.gaab(), [1.0, 3.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__gaaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_gaaa([1.0, 3.0, 3.0, 3.0].into());
                    assert_eq!(vector.gaaa(), [1.0, 3.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__brrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brrr([2.0, 0.0, 0.0, 0.0].into());
                    assert_eq!(vector.brrr(), [2.0, 0.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__brrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brrg([2.0, 0.0, 0.0, 1.0].into());
                    assert_eq!(vector.brrg(), [2.0, 0.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__brrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brrb([2.0, 0.0, 0.0, 2.0].into());
                    assert_eq!(vector.brrb(), [2.0, 0.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__brra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brra([2.0, 0.0, 0.0, 3.0].into());
                    assert_eq!(vector.brra(), [2.0, 0.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__brgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brgr([2.0, 0.0, 1.0, 0.0].into());
                    assert_eq!(vector.brgr(), [2.0, 0.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__brgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brgg([2.0, 0.0, 1.0, 1.0].into());
                    assert_eq!(vector.brgg(), [2.0, 0.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__brgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brgb([2.0, 0.0, 1.0, 2.0].into());
                    assert_eq!(vector.brgb(), [2.0, 0.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__brga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brga([2.0, 0.0, 1.0, 3.0].into());
                    assert_eq!(vector.brga(), [2.0, 0.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__brbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brbr([2.0, 0.0, 2.0, 0.0].into());
                    assert_eq!(vector.brbr(), [2.0, 0.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__brbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brbg([2.0, 0.0, 2.0, 1.0].into());
                    assert_eq!(vector.brbg(), [2.0, 0.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__brbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brbb([2.0, 0.0, 2.0, 2.0].into());
                    assert_eq!(vector.brbb(), [2.0, 0.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__brba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brba([2.0, 0.0, 2.0, 3.0].into());
                    assert_eq!(vector.brba(), [2.0, 0.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__brar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brar([2.0, 0.0, 3.0, 0.0].into());
                    assert_eq!(vector.brar(), [2.0, 0.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__brag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brag([2.0, 0.0, 3.0, 1.0].into());
                    assert_eq!(vector.brag(), [2.0, 0.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__brab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_brab([2.0, 0.0, 3.0, 2.0].into());
                    assert_eq!(vector.brab(), [2.0, 0.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__braa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_braa([2.0, 0.0, 3.0, 3.0].into());
                    assert_eq!(vector.braa(), [2.0, 0.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bgrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgrr([2.0, 1.0, 0.0, 0.0].into());
                    assert_eq!(vector.bgrr(), [2.0, 1.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bgrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgrg([2.0, 1.0, 0.0, 1.0].into());
                    assert_eq!(vector.bgrg(), [2.0, 1.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bgrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgrb([2.0, 1.0, 0.0, 2.0].into());
                    assert_eq!(vector.bgrb(), [2.0, 1.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bgra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgra([2.0, 1.0, 0.0, 3.0].into());
                    assert_eq!(vector.bgra(), [2.0, 1.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bggr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bggr([2.0, 1.0, 1.0, 0.0].into());
                    assert_eq!(vector.bggr(), [2.0, 1.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bggg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bggg([2.0, 1.0, 1.0, 1.0].into());
                    assert_eq!(vector.bggg(), [2.0, 1.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bggb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bggb([2.0, 1.0, 1.0, 2.0].into());
                    assert_eq!(vector.bggb(), [2.0, 1.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bgga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgga([2.0, 1.0, 1.0, 3.0].into());
                    assert_eq!(vector.bgga(), [2.0, 1.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bgbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgbr([2.0, 1.0, 2.0, 0.0].into());
                    assert_eq!(vector.bgbr(), [2.0, 1.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bgbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgbg([2.0, 1.0, 2.0, 1.0].into());
                    assert_eq!(vector.bgbg(), [2.0, 1.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bgbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgbb([2.0, 1.0, 2.0, 2.0].into());
                    assert_eq!(vector.bgbb(), [2.0, 1.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bgba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgba([2.0, 1.0, 2.0, 3.0].into());
                    assert_eq!(vector.bgba(), [2.0, 1.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bgar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgar([2.0, 1.0, 3.0, 0.0].into());
                    assert_eq!(vector.bgar(), [2.0, 1.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bgag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgag([2.0, 1.0, 3.0, 1.0].into());
                    assert_eq!(vector.bgag(), [2.0, 1.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bgab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgab([2.0, 1.0, 3.0, 2.0].into());
                    assert_eq!(vector.bgab(), [2.0, 1.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bgaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bgaa([2.0, 1.0, 3.0, 3.0].into());
                    assert_eq!(vector.bgaa(), [2.0, 1.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bbrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbrr([2.0, 2.0, 0.0, 0.0].into());
                    assert_eq!(vector.bbrr(), [2.0, 2.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bbrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbrg([2.0, 2.0, 0.0, 1.0].into());
                    assert_eq!(vector.bbrg(), [2.0, 2.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bbrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbrb([2.0, 2.0, 0.0, 2.0].into());
                    assert_eq!(vector.bbrb(), [2.0, 2.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bbra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbra([2.0, 2.0, 0.0, 3.0].into());
                    assert_eq!(vector.bbra(), [2.0, 2.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bbgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbgr([2.0, 2.0, 1.0, 0.0].into());
                    assert_eq!(vector.bbgr(), [2.0, 2.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bbgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbgg([2.0, 2.0, 1.0, 1.0].into());
                    assert_eq!(vector.bbgg(), [2.0, 2.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bbgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbgb([2.0, 2.0, 1.0, 2.0].into());
                    assert_eq!(vector.bbgb(), [2.0, 2.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bbga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbga([2.0, 2.0, 1.0, 3.0].into());
                    assert_eq!(vector.bbga(), [2.0, 2.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bbbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbbr([2.0, 2.0, 2.0, 0.0].into());
                    assert_eq!(vector.bbbr(), [2.0, 2.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bbbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbbg([2.0, 2.0, 2.0, 1.0].into());
                    assert_eq!(vector.bbbg(), [2.0, 2.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bbbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbbb([2.0, 2.0, 2.0, 2.0].into());
                    assert_eq!(vector.bbbb(), [2.0, 2.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bbba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbba([2.0, 2.0, 2.0, 3.0].into());
                    assert_eq!(vector.bbba(), [2.0, 2.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bbar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbar([2.0, 2.0, 3.0, 0.0].into());
                    assert_eq!(vector.bbar(), [2.0, 2.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bbag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbag([2.0, 2.0, 3.0, 1.0].into());
                    assert_eq!(vector.bbag(), [2.0, 2.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bbab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbab([2.0, 2.0, 3.0, 2.0].into());
                    assert_eq!(vector.bbab(), [2.0, 2.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bbaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bbaa([2.0, 2.0, 3.0, 3.0].into());
                    assert_eq!(vector.bbaa(), [2.0, 2.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__barr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_barr([2.0, 3.0, 0.0, 0.0].into());
                    assert_eq!(vector.barr(), [2.0, 3.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__barg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_barg([2.0, 3.0, 0.0, 1.0].into());
                    assert_eq!(vector.barg(), [2.0, 3.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__barb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_barb([2.0, 3.0, 0.0, 2.0].into());
                    assert_eq!(vector.barb(), [2.0, 3.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__bara() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bara([2.0, 3.0, 0.0, 3.0].into());
                    assert_eq!(vector.bara(), [2.0, 3.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__bagr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bagr([2.0, 3.0, 1.0, 0.0].into());
                    assert_eq!(vector.bagr(), [2.0, 3.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__bagg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bagg([2.0, 3.0, 1.0, 1.0].into());
                    assert_eq!(vector.bagg(), [2.0, 3.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__bagb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_bagb([2.0, 3.0, 1.0, 2.0].into());
                    assert_eq!(vector.bagb(), [2.0, 3.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__baga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_baga([2.0, 3.0, 1.0, 3.0].into());
                    assert_eq!(vector.baga(), [2.0, 3.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__babr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_babr([2.0, 3.0, 2.0, 0.0].into());
                    assert_eq!(vector.babr(), [2.0, 3.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__babg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_babg([2.0, 3.0, 2.0, 1.0].into());
                    assert_eq!(vector.babg(), [2.0, 3.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__babb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_babb([2.0, 3.0, 2.0, 2.0].into());
                    assert_eq!(vector.babb(), [2.0, 3.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__baba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_baba([2.0, 3.0, 2.0, 3.0].into());
                    assert_eq!(vector.baba(), [2.0, 3.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__baar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_baar([2.0, 3.0, 3.0, 0.0].into());
                    assert_eq!(vector.baar(), [2.0, 3.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__baag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_baag([2.0, 3.0, 3.0, 1.0].into());
                    assert_eq!(vector.baag(), [2.0, 3.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__baab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_baab([2.0, 3.0, 3.0, 2.0].into());
                    assert_eq!(vector.baab(), [2.0, 3.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__baaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_baaa([2.0, 3.0, 3.0, 3.0].into());
                    assert_eq!(vector.baaa(), [2.0, 3.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__arrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arrr([3.0, 0.0, 0.0, 0.0].into());
                    assert_eq!(vector.arrr(), [3.0, 0.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__arrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arrg([3.0, 0.0, 0.0, 1.0].into());
                    assert_eq!(vector.arrg(), [3.0, 0.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__arrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arrb([3.0, 0.0, 0.0, 2.0].into());
                    assert_eq!(vector.arrb(), [3.0, 0.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__arra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arra([3.0, 0.0, 0.0, 3.0].into());
                    assert_eq!(vector.arra(), [3.0, 0.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__argr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_argr([3.0, 0.0, 1.0, 0.0].into());
                    assert_eq!(vector.argr(), [3.0, 0.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__argg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_argg([3.0, 0.0, 1.0, 1.0].into());
                    assert_eq!(vector.argg(), [3.0, 0.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__argb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_argb([3.0, 0.0, 1.0, 2.0].into());
                    assert_eq!(vector.argb(), [3.0, 0.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__arga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arga([3.0, 0.0, 1.0, 3.0].into());
                    assert_eq!(vector.arga(), [3.0, 0.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__arbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arbr([3.0, 0.0, 2.0, 0.0].into());
                    assert_eq!(vector.arbr(), [3.0, 0.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__arbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arbg([3.0, 0.0, 2.0, 1.0].into());
                    assert_eq!(vector.arbg(), [3.0, 0.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__arbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arbb([3.0, 0.0, 2.0, 2.0].into());
                    assert_eq!(vector.arbb(), [3.0, 0.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__arba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arba([3.0, 0.0, 2.0, 3.0].into());
                    assert_eq!(vector.arba(), [3.0, 0.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__arar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arar([3.0, 0.0, 3.0, 0.0].into());
                    assert_eq!(vector.arar(), [3.0, 0.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__arag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arag([3.0, 0.0, 3.0, 1.0].into());
                    assert_eq!(vector.arag(), [3.0, 0.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__arab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_arab([3.0, 0.0, 3.0, 2.0].into());
                    assert_eq!(vector.arab(), [3.0, 0.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__araa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_araa([3.0, 0.0, 3.0, 3.0].into());
                    assert_eq!(vector.araa(), [3.0, 0.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__agrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agrr([3.0, 1.0, 0.0, 0.0].into());
                    assert_eq!(vector.agrr(), [3.0, 1.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__agrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agrg([3.0, 1.0, 0.0, 1.0].into());
                    assert_eq!(vector.agrg(), [3.0, 1.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__agrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agrb([3.0, 1.0, 0.0, 2.0].into());
                    assert_eq!(vector.agrb(), [3.0, 1.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__agra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agra([3.0, 1.0, 0.0, 3.0].into());
                    assert_eq!(vector.agra(), [3.0, 1.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__aggr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aggr([3.0, 1.0, 1.0, 0.0].into());
                    assert_eq!(vector.aggr(), [3.0, 1.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__aggg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aggg([3.0, 1.0, 1.0, 1.0].into());
                    assert_eq!(vector.aggg(), [3.0, 1.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__aggb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aggb([3.0, 1.0, 1.0, 2.0].into());
                    assert_eq!(vector.aggb(), [3.0, 1.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__agga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agga([3.0, 1.0, 1.0, 3.0].into());
                    assert_eq!(vector.agga(), [3.0, 1.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__agbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agbr([3.0, 1.0, 2.0, 0.0].into());
                    assert_eq!(vector.agbr(), [3.0, 1.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__agbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agbg([3.0, 1.0, 2.0, 1.0].into());
                    assert_eq!(vector.agbg(), [3.0, 1.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__agbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agbb([3.0, 1.0, 2.0, 2.0].into());
                    assert_eq!(vector.agbb(), [3.0, 1.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__agba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agba([3.0, 1.0, 2.0, 3.0].into());
                    assert_eq!(vector.agba(), [3.0, 1.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__agar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agar([3.0, 1.0, 3.0, 0.0].into());
                    assert_eq!(vector.agar(), [3.0, 1.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__agag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agag([3.0, 1.0, 3.0, 1.0].into());
                    assert_eq!(vector.agag(), [3.0, 1.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__agab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agab([3.0, 1.0, 3.0, 2.0].into());
                    assert_eq!(vector.agab(), [3.0, 1.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__agaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_agaa([3.0, 1.0, 3.0, 3.0].into());
                    assert_eq!(vector.agaa(), [3.0, 1.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__abrr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abrr([3.0, 2.0, 0.0, 0.0].into());
                    assert_eq!(vector.abrr(), [3.0, 2.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__abrg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abrg([3.0, 2.0, 0.0, 1.0].into());
                    assert_eq!(vector.abrg(), [3.0, 2.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__abrb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abrb([3.0, 2.0, 0.0, 2.0].into());
                    assert_eq!(vector.abrb(), [3.0, 2.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__abra() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abra([3.0, 2.0, 0.0, 3.0].into());
                    assert_eq!(vector.abra(), [3.0, 2.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__abgr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abgr([3.0, 2.0, 1.0, 0.0].into());
                    assert_eq!(vector.abgr(), [3.0, 2.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__abgg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abgg([3.0, 2.0, 1.0, 1.0].into());
                    assert_eq!(vector.abgg(), [3.0, 2.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__abgb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abgb([3.0, 2.0, 1.0, 2.0].into());
                    assert_eq!(vector.abgb(), [3.0, 2.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__abga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abga([3.0, 2.0, 1.0, 3.0].into());
                    assert_eq!(vector.abga(), [3.0, 2.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__abbr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abbr([3.0, 2.0, 2.0, 0.0].into());
                    assert_eq!(vector.abbr(), [3.0, 2.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__abbg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abbg([3.0, 2.0, 2.0, 1.0].into());
                    assert_eq!(vector.abbg(), [3.0, 2.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__abbb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abbb([3.0, 2.0, 2.0, 2.0].into());
                    assert_eq!(vector.abbb(), [3.0, 2.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__abba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abba([3.0, 2.0, 2.0, 3.0].into());
                    assert_eq!(vector.abba(), [3.0, 2.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__abar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abar([3.0, 2.0, 3.0, 0.0].into());
                    assert_eq!(vector.abar(), [3.0, 2.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__abag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abag([3.0, 2.0, 3.0, 1.0].into());
                    assert_eq!(vector.abag(), [3.0, 2.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__abab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abab([3.0, 2.0, 3.0, 2.0].into());
                    assert_eq!(vector.abab(), [3.0, 2.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__abaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_abaa([3.0, 2.0, 3.0, 3.0].into());
                    assert_eq!(vector.abaa(), [3.0, 2.0, 3.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__aarr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aarr([3.0, 3.0, 0.0, 0.0].into());
                    assert_eq!(vector.aarr(), [3.0, 3.0, 0.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__aarg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aarg([3.0, 3.0, 0.0, 1.0].into());
                    assert_eq!(vector.aarg(), [3.0, 3.0, 0.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__aarb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aarb([3.0, 3.0, 0.0, 2.0].into());
                    assert_eq!(vector.aarb(), [3.0, 3.0, 0.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__aara() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aara([3.0, 3.0, 0.0, 3.0].into());
                    assert_eq!(vector.aara(), [3.0, 3.0, 0.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__aagr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aagr([3.0, 3.0, 1.0, 0.0].into());
                    assert_eq!(vector.aagr(), [3.0, 3.0, 1.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__aagg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aagg([3.0, 3.0, 1.0, 1.0].into());
                    assert_eq!(vector.aagg(), [3.0, 3.0, 1.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__aagb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aagb([3.0, 3.0, 1.0, 2.0].into());
                    assert_eq!(vector.aagb(), [3.0, 3.0, 1.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__aaga() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aaga([3.0, 3.0, 1.0, 3.0].into());
                    assert_eq!(vector.aaga(), [3.0, 3.0, 1.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__aabr() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aabr([3.0, 3.0, 2.0, 0.0].into());
                    assert_eq!(vector.aabr(), [3.0, 3.0, 2.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__aabg() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aabg([3.0, 3.0, 2.0, 1.0].into());
                    assert_eq!(vector.aabg(), [3.0, 3.0, 2.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__aabb() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aabb([3.0, 3.0, 2.0, 2.0].into());
                    assert_eq!(vector.aabb(), [3.0, 3.0, 2.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__aaba() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aaba([3.0, 3.0, 2.0, 3.0].into());
                    assert_eq!(vector.aaba(), [3.0, 3.0, 2.0, 3.0].into());
                }
            

                #[test]
                fn test_Float4__aaar() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aaar([3.0, 3.0, 3.0, 0.0].into());
                    assert_eq!(vector.aaar(), [3.0, 3.0, 3.0, 0.0].into());
                }
            

                #[test]
                fn test_Float4__aaag() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aaag([3.0, 3.0, 3.0, 1.0].into());
                    assert_eq!(vector.aaag(), [3.0, 3.0, 3.0, 1.0].into());
                }
            

                #[test]
                fn test_Float4__aaab() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aaab([3.0, 3.0, 3.0, 2.0].into());
                    assert_eq!(vector.aaab(), [3.0, 3.0, 3.0, 2.0].into());
                }
            

                #[test]
                fn test_Float4__aaaa() {
                    let mut vector: Float4 = [0.0, 0.0, 0.0, 0.0].into();
                    vector.set_aaaa([3.0, 3.0, 3.0, 3.0].into());
                    assert_eq!(vector.aaaa(), [3.0, 3.0, 3.0, 3.0].into());
                }
            
