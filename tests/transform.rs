/**
 * 变换
 */
 #[test]
 fn test_tr3_new() {
     use nalgebra::{Matrix4, Transform3};
 
     let m = Matrix4::new(
        1.0, 0.0, 0.0, 10.0, 
        0.0, 1.0, 0.0,20.0, 
        0.0, 0.0, 1.0, 30.0,
        0.0, 0.0, 0.0, 1.0);

    let t = Transform3::from_matrix_unchecked(m);

    assert_eq!(t.into_inner(), m);
 }
