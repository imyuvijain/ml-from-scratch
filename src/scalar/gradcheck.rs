pub fn numeric_grad(f: impl Fn(&[f32]) -> f32, x: &[f32], h: f32) -> Vec<f32> {
    
    let mut output: Vec<f32> = vec!();

    for i in 0..x.len() {

        let mut x_plus = x.to_vec();
        let mut x_minus = x.to_vec();

        x_plus[i] += h;
        x_minus[i] -= h;

        // f(x+h) - f(x-h) / 2h
        let slope = (f(&x_plus) - f(&x_minus)) / (2.0*h);
        output.push(slope);
    }

    output
}