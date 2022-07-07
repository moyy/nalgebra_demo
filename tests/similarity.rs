
/**
 * Similarity 相似变换
 * 
 * 均匀缩放 --> 旋转 --> 平移
 */

#[test]
fn test_sim_new() {
    use nalgebra::{UnitComplex,Transform2, Similarity2, Translation2};

    let s = Similarity2::from_parts(
        Translation2::new(100.0, 200.0),
        UnitComplex::new(std::f64::consts::FRAC_PI_6),
        3.0
    );

    assert_eq!(3.0, s.scaling());
    
    // 取 相似变换 的 各种信息
    let i = &s.isometry;
    assert_eq!(Translation2::new(100.0, 200.0), i.translation);
    assert_eq!(UnitComplex::new(std::f64::consts::FRAC_PI_6), i.rotation);

    // 注：Similarity 到 Transform 用 乘法
    let tr = Transform2::identity() * s;
}


#[test]
fn test_convert() {
    use nalgebra::{Isometry2, Similarity2, Vector2};

    // Isometry -> Similarity conversion always succeeds.
    let iso = Isometry2::new(Vector2::new(1.0, 2.0), nalgebra::zero());
    let _: Similarity2<f64> = nalgebra::convert(iso);

    // Similarity -> Isometry conversion fails if the scaling factor is not 1.0.
    let sim_without_scaling = Similarity2::new(Vector2::new(1.0, 2.0), std::f64::consts::PI, 1.0);
    let sim_with_scaling    = Similarity2::new(Vector2::new(1.0, 2.0), std::f64::consts::PI, 2.0);

    let iso_success: Option<Isometry2<f64>> = nalgebra::try_convert(sim_without_scaling);
    let iso_fail:    Option<Isometry2<f64>> = nalgebra::try_convert(sim_with_scaling);

    assert!(iso_success.is_some());
    assert!(iso_fail.is_none());

    // Similarity -> Isometry conversion can be forced at your own risks.
    let iso_forced: Isometry2<f64> = nalgebra::convert_unchecked(sim_with_scaling);
    assert_eq!(iso_success.unwrap(), iso_forced);
}