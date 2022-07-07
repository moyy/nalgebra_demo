
#[test]
fn test_r3_new() {
    use nalgebra::{Vector3, Matrix4, Rotation3};

    let expected = Matrix4::new(
        0.8660254, -0.5,      0.0, 0.0,
        0.5,       0.8660254, 0.0, 0.0,
        0.0,       0.0,       1.0, 0.0,
        0.0,       0.0,       0.0, 1.0
    );
    
    // 绕 z轴 旋转 PI/6 弧度
    let rot = Rotation3::from_axis_angle(&Vector3::z_axis(), std::f32::consts::FRAC_PI_6);
    assert_eq!(rot.to_homogeneous(), expected);
}

/**
 * 旋转矩阵是 正交阵，其逆 就是 转置
 */
#[test]
fn test_r3_inverse() {
    use approx::assert_relative_eq;
    use nalgebra::{Vector3, Rotation3};
    
    let rot = Rotation3::new(Vector3::new(1.0, 2.0, 3.0));
    
    let inv = rot.inverse();
    let tr = rot.transpose();

    assert_eq!(tr, inv);
 
    assert_relative_eq!(inv * rot, Rotation3::identity(), epsilon = 1.0e-6);
}

/**
 * 两个向量 的 旋转 变换
 */
#[test]
fn test_r3_between() {
    use approx::assert_relative_eq;
    use nalgebra::{Vector2, Rotation2};
    
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(2.0, 1.0);
    
    let rot = Rotation2::rotation_between(&a, &b);
    
    assert_relative_eq!(rot * a, b);
    assert_relative_eq!(rot.inverse() * b, a);
}

/**
 * 提取变换中的 旋转矩阵
 */
#[test]
fn test_r3_from_matrix() {
    use approx::assert_relative_eq;
    use nalgebra::{Matrix3, Transform2, Translation2, Scale2, Rotation2};
      
    let t = Translation2::new(10.0, 20.0);
    let r = Rotation2::new(std::f32::consts::FRAC_PI_6);
    let s = Scale2::new(5.0, 6.0);

    let homogeneous = t.to_homogeneous() * r.to_homogeneous() * s.to_homogeneous();

    // TODO
//    let s1 = Rotation2::from_matrix(homogeneous);
}