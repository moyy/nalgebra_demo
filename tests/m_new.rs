/**
 * 从 平移向量 构造 矩阵
 * 验证：矩阵 是 列主序 的
 */
#[test]
fn test_m4_translation() {
    use nalgebra::{Matrix4, Vector3};
    
    // m1 打印出来是 二维数组 [v1, v2, v3, v4]
    // 分别对应 第1列 第2列 第3列 第4列
    let m = Matrix4::new_translation(&Vector3::new(100.0, 200.0, 300.0));
    
    // 打印 new_translation: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [100.0, 200.0, 300.0, 1.0]]
    println!("new_translation: {:?}", m);

    // 取 第4列（注意 索引 从0开始）
    // 打印 new_translation column(3) = (100, 200, 300, 1)
    let v = m.column(3);
    println!("new_translation column(3) = ({}, {}, {}, {})\n", v[0], v[1], v[2], v[3]);
}

/**
 * 验证 matrix4 是 列主序 的
 */
#[test]
fn test_m4_new() {
    use nalgebra::{Matrix4};

    // Matrix4::new(m11, m12, m13, m14, m21...m44)
    // m24 代表 第2行第1列
    let m = Matrix4::new(
        1.0, 0.0, 0.0, 100.0,
        0.0, 1.0, 0.0, 200.0,
        0.0, 0.0, 1.0, 300.0,
        0.0, 0.0, 0.0,   1.0,
    );

    // 打印 Matrix4::new [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [100.0, 200.0, 300.0, 1.0]]
    println!("Matrix4::new {:?}", m);

    // 取 第4列（索引 从0开始）
    // 打印 Matrix4::new column(3) = (100, 200, 300, 1)
    let v = m.column(3);
    println!("Matrix4::new column(3) = ({}, {}, {}, {})\n", v[0], v[1], v[2], v[3]);
}

/**
 * 验证 行向量 构造
 */
#[test]
fn test_m4_from_rows() {
    use nalgebra::{Matrix4, RowVector4};

    // 行向量 构造 矩阵
    // 行向量 必须用 RowVector
    // 可以 更精确的看到  100 200 300 作为 平移分量 是怎么 放置的
    let m = Matrix4::from_rows(&[
        RowVector4::new(1.0, 0.0, 0.0, 100.0),
        RowVector4::new(0.0, 1.0, 0.0, 200.0),
        RowVector4::new(0.0, 0.0, 1.0, 300.0),
        RowVector4::new(0.0, 0.0, 0.0, 1.0),
    ]);

    // 打印 Matrix4::from_rows [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [100.0, 200.0, 300.0, 1.0]]
    println!("Matrix4::from_rows {:?}\n", m);
}

/**
 * 验证 列向量 构造
 */
#[test]
fn test_m4_from_columns() {
    use nalgebra::{Matrix4, Vector4};
    // 列向量 构造 矩阵
    // 列向量 必须用 Vector
    // 可以 更精确的看到  100 200 300 作为 平移分量 是怎么 放置的
    let m = Matrix4::from_columns(&[
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(100.0, 200.0, 300.0, 1.0),
    ]);
    
    // 打印 Matrix4::from_columns [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [100.0, 200.0, 300.0, 1.0]]
    println!("Matrix4::from_columns {:?}\n", m);
}

/**
 * 设置 行 的 方式
 */
#[test]
fn test_m4_set_rows() {
    use nalgebra::{Matrix4, RowVector4};
    
    // m 构造为 单位阵
    let mut m = Matrix4::identity();
    // 设置 第三行 为 平移平移分量，这个打印和 上面的 不 同
    m.set_row(3, &RowVector4::new(100.0, 200.0, 300.0, 1.0));
    // 打印 set_row(3) [[1.0, 0.0, 0.0, 100.0], [0.0, 1.0, 0.0, 200.0], [0.0, 0.0, 1.0, 300.0], [0.0, 0.0, 0.0, 1.0]]
    println!("set_row(3) {:?}\n", m);
}

/**
 * 设置 列 的 方式
 */
#[test]
fn test_m4_set_columns() {
    use nalgebra::{Matrix4, Vector4};
    
    // m 构造为 单位阵
    let mut m = Matrix4::identity();
    // 设置 第三列 为 平移平移分量，这个打印和上面的 一 致；
    m.set_column(3, &Vector4::new(100.0, 200.0, 300.0, 1.0));
    // 打印 set_column(3) [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [100.0, 200.0, 300.0, 1.0]]
    println!("set_column(3) {:?}\n", m);
}