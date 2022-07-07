
/**
 * 单位
 */
#[test]
fn test_t3_identity() {
    use nalgebra::{Point3, Translation3};

    let t = Translation3::identity();

    let p = Point3::new(1.0, 2.0, 3.0);
    
    assert_eq!(t * p, p);
}

/**
 * 平移
 */
#[test]
fn test_t3_new() {
    use nalgebra::{Matrix4, Translation3};

    let expected = Matrix4::new(
        1.0, 0.0, 0.0, 10.0, 
        0.0, 1.0, 0.0, 20.0, 
        0.0, 0.0, 1.0, 30.0, 
        0.0, 0.0, 0.0, 1.0,
    );

    let t = Translation3::new(10.0, 20.0, 30.0);

    assert_eq!(t.to_homogeneous(), expected);
}


/**
 * 构造
 */
#[test]
fn test_t3_from_vec() {
    use nalgebra::{Translation3};

    let t = Translation3::from([1.0, 2.0, 3.0]);

    let p = Translation3::new(1.0, 2.0, 3.0);
    
    assert_eq!(t, p);
}

/**
 * 求逆
 */
#[test]
fn test_t3_inverse() {
    use nalgebra::{Translation3};

    let t = Translation3::new(1.0, 2.0, 3.0);

    let inv = t.inverse();
    assert_eq!(inv, Translation3::new(-1.0, -2.0, -3.0));

    assert_eq!(t * inv, Translation3::identity());
    
    assert_eq!(inv * t, Translation3::identity());
}

// 原地 转 逆
#[test]
fn test_t3_inverse_mut() {
    use nalgebra::{Translation3};

    let mut t = Translation3::new(1.0, 2.0, 3.0);

    t.inverse_mut();
    assert_eq!(t, Translation3::new(-1.0, -2.0, -3.0));
}

/**
 * 作用于 点
 */
#[test]
fn test_t3_apply_point() {
    use nalgebra::{Translation3, Point3};

    let t = Translation3::new(1.0, 2.0, 3.0);

    let transformed_point = t.transform_point(&Point3::new(4.0, 5.0, 6.0));

    assert_eq!(transformed_point, Point3::new(5.0, 7.0, 9.0));
}

/**
 * 逆 作用于 点
 */
#[test]
fn test_t3_inverse_apply_point() {
    use nalgebra::{Translation3, Point3};

    let t = Translation3::new(1.0, 2.0, 3.0);
    
    let transformed_point = t.inverse_transform_point(&Point3::new(4.0, 5.0, 6.0));

    assert_eq!(transformed_point, Point3::new(3.0, 3.0, 3.0));
}
