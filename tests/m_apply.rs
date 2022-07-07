/**
 * 矩阵 作用于 点
 */
#[test]
fn test_m4_apply_point() {
    use nalgebra::{Matrix4, Point3, Vector4};
    
    let mut m = Matrix4::identity();
    m.set_column(3, &Vector4::new(100.0, 200.0, 300.0, 1.0));
    
    let p = Point3::new(1.0, 2.0, 3.0);

    // 矩阵是 4*4 的话，要和 齐次坐标相乘
    let p2 = m * p.to_homogeneous();
    
    // 齐次坐标 变回 普通 坐标
    let p3 = Point3::from_homogeneous(p2).unwrap();
    println!("p3 {:?}\n", p3);
}

/**
 * 矩阵 作用于 向量
 */
#[test]
fn test_m4_apply_vector() {
    use nalgebra::{Matrix4, Vector3, Vector4};
    
    let mut m = Matrix4::identity();
    m.set_column(3, &Vector4::new(100.0, 200.0, 300.0, 1.0));
    
    let v = Vector3::new(1.0, 2.0, 3.0);

    let v2 = m * v.to_homogeneous();
    let v3 = Vector3::from_homogeneous(v2).unwrap();
    println!("v3 {:?}\n", v3);
}