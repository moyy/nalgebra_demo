
#[test]
fn test_qr() {
    use nalgebra::{UnitComplex,Transform2, Similarity2, Matrix3, Translation2};
    use approx::assert_relative_eq;

    let s1 = Similarity2::from_parts(
        Translation2::new(100.0, 200.0),
        UnitComplex::new(std::f64::consts::FRAC_PI_6),
        3.0
    );

    let s2 = Similarity2::from_parts(
        Translation2::new(10.0, 20.0),
        UnitComplex::new(std::f64::consts::FRAC_PI_3),
        6.0
    );

    let shear = Matrix3::new(
        1.0, 6.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );

    let reflect = Matrix3::new(
        0.0, 1.0, 0.0,
        1.0, 0.0, 0.0,
        0.0, 0.0, 1.0,
    );

    // 注：Similarity 到 Transform 用 乘法
    let m = s1.to_homogeneous() * reflect * shear * s2.to_homogeneous();

    let (pre, scale, translation) = decompose_transform(m);
    
    let tr = Transform2::identity() * Translation2::new(translation[0], translation[1]);
    let result =  tr.to_homogeneous() * pre * scale;
    // 验证 分解后 的 乘积 就是 原矩阵
    assert_relative_eq!(result, m, epsilon = 0.001);
    
    // 验证 分解后 的 scale
    assert!(scale.m11 > 0.0);
    assert!(scale.m22 > 0.0);
    assert_relative_eq!(scale.m12, 0.0, epsilon = 0.001);
    assert_relative_eq!(scale.m21, 0.0, epsilon = 0.001);
    
    // 验证 分解后 的 pre 没有 缩放系数
    // 行列式 肯定是  正负 1
    assert_relative_eq!(f64::abs(pre.determinant()), 1.0, epsilon = 0.001);

}

// 返回：
// 第一个 表示 旋转 反射 错切 信息；行列式 等于 正负 1
// 第二个 表示 缩放 信息，缩放系数 全为正值
// 第三个 表示 平移 信息
fn decompose_transform(m: nalgebra::Matrix3<f64>) -> (nalgebra::Matrix3<f64>, nalgebra::Matrix3<f64>, nalgebra::Vector2<f64>) {
    use nalgebra::{Vector2, Vector3, Matrix2};
    use approx::assert_relative_eq;

    // 第一步，判断 是否 仿射变换，判断最后一行是否 等于 0, 0, 1
    let v = m.row(2);
    assert_eq!( Vector3::new(v[0], v[1], v[2]), Vector3::new(0.0, 0.0, 1.0));

    // 第二步，最后一列 抽出来 就是 平移 信息
    let translation = m.column(2);
    let translation = Vector2::new(translation[0], translation[1]);

    // 第三步，构造 左上角 2*2 矩阵 
    let m = Matrix2::new(m.m11, m.m12, m.m21, m.m22);

    // 第四步，QR 分解
    let (q, r) = m.qr().unpack();
    
    // assert_relative_eq!(m, q * r);

    // 第五步，将 R 分解为 shear + scale   
    let a = r.m11;
    let b = r.m12;
    let d = r.m22;
    
    // 下面 四行代码 保证 缩放系数 为 正数
    let shear_a = if a < 0.0 { -1.0 } else { 1.0 };
    let shear_d = if d < 0.0 { -1.0 } else { 1.0 };

    let a = if a < 0.0 { -a } else { a };
    let d = if d < 0.0 { -d } else { d };

    let shear = Matrix2::new(shear_a, b / d, 0.0, shear_d);
    let scale = Matrix2::new(a, 0.0, 0.0, d);

    // 就算 前置 变换，特点：没有 缩放系数
    let pre = q * shear;

    let pre = pre.to_homogeneous();
    let scale = scale.to_homogeneous();
    
    (pre, scale, translation)

}
