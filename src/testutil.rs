pub fn assert_close(a: f32, b: f32, tol: f32) {
    if (a - b).abs() > tol {
        panic!("not close: {a} vs {b} (diff {}, tol {tol})", (a - b).abs());
    }
}
