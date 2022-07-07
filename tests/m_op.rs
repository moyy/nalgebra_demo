/**
 * 加法
 */
#[test]
fn test_m4_add() {
    use nalgebra::{Matrix4};
    
    let m1 = Matrix4::<f32>::identity();

    let mut m2 = Matrix4::<f32>::identity();

    let m3 = m1 + m2;
    println!("add: {:?}", m3);

    m2 += m1;
    println!("add-assign: {:?}", m2);
}

/**
 * 矩阵乘法
 */
#[test]
fn test_m4_mul() {
    use nalgebra::{Matrix2x3, Matrix3x4, Vector2, Vector3};

    // 2行3列，就是 3个二维 列向量
    let m1 = Matrix2x3::from_columns(&[
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ]);

    // 3行4列，就是 4个 三维 列向量
    let m2 = Matrix3x4::from_columns(&[
        Vector3::new(1.0, 0.0, 1.0),
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(0.0, 1.0, 1.0),
        Vector3::new(0.0, 0.0, 1.0),
    ]);

    // 对 方阵的乘法，一样可以用 *= 
    // 结果 是 2*4 的 矩阵 Matrix<f64, Const<2>, Const<4>, ArrayStorage<f64, 2, 4>>
    let m3 = m1 * m2;
    println!("mul: {:?}", m3);
}

/**
 * 逐元素乘法
 */
#[test]
fn test_m4_component_mul() {
    use nalgebra::{Matrix2, Vector2};

	let a = Matrix2::from_columns(&[
        Vector2::new(0.0, 2.0),
        Vector2::new(1.0, 3.0)
    ]);
    let b = Matrix2::from_columns(&[
        Vector2::new(4.0, 6.0),
        Vector2::new(5.0, 7.0)
    ]);
    let expected = Matrix2::from_columns(&[
        Vector2::new(0.0, 12.0),
        Vector2::new(5.0, 21.0)
    ]);

    let m = a.component_mul(&b);
	assert_eq!(m, expected);
	println!("component_mul: {:?}", m);
}

/**
 * 求逆
 */
#[test]
fn test_m4_inverse() {
    use nalgebra::{Matrix2, Vector2};

    let m = Matrix2::from_columns(&[
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
    ]);
  
    match m.try_inverse() {
        Some(inv) => {
            println!("inverse: {:?}", inv)
        }
        None => {
            println!("not inversable!");
        }
    }
}

/**
 * 转置
 */
#[test]
fn test_m4_transpose() {
    use nalgebra::{Matrix2x3, Vector2};

    let m = Matrix2x3::from_columns(&[
        Vector2::new(1.0, 2.0),
        Vector2::new(3.0, 4.0),
        Vector2::new(5.0, 6.0),
    ]);
	// 转置 2行3列，得到 3行2列的矩阵 t
    let t = m.transpose();
	// 列主序 的 打印
    println!("transpose: {:?}", t);
}

/**
 * 行列式
 */
#[test]
fn test_m4_determinant() {
    use nalgebra::{Matrix2, Vector2};

    let m = Matrix2::from_columns(&[
        Vector2::new(1.0, 2.0),
        Vector2::new(3.0, 4.0),
    ]);

    let det = m.determinant();
	// 列主序 的 打印
    println!("determinant: {:?}", det);
}