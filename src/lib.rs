//! Una libreria que resuelve sistemas 2x2 y 3x3 de ecuaciones lineales usando el método de [Reducción de Cramer.]
//!
//! [Reducción de Cramer.]: https://es.wikipedia.org/wiki/Regla_de_Cramer

/// Resuelve una ecuación 3x3
pub fn solve_3_determinants(
    x1: f64,
    x2: f64,
    x3: f64,
    y1: f64,
    y2: f64,
    y3: f64,
    z1: f64,
    z2: f64,
    z3: f64,
    res_1: f64,
    res_2: f64,
    res_3: f64,
) -> (f64, f64, f64) {
    let dp = (x1 * y2 * z3) + (y1 * z2 * x3) + (z1 * x2 * y3);
    let ds = (x3 * y2 * z1) + (y3 * z2 * x1) + (z3 * x2 * y1);
    let determinant = dp - ds;
    let dpx = (res_1 * y2 * z3) + (y1 * z2 * res_3) + (z1 * res_2 * y3);
    let dsx = (res_3 * y2 * z1) + (y3 * z2 * res_1) + (z3 * res_2 * y1);
    let determinant_x = dpx - dsx;
    let x = determinant_x / determinant;
    let dpy = (x1 * res_2 * z3) + (res_1 * z2 * x3) + (z1 * x2 * res_3);
    let dsy = (x3 * res_2 * z1) + (res_3 * z2 * x1) + (z3 * x2 * res_1);
    let determinant_y = dpy - dsy;
    let y = determinant_y / determinant;
    let dpz = (x1 * y2 * res_3) + (y1 * res_2 * x3) + (res_1 * x2 * y3);
    let dsz = (x3 * y2 * res_1) + (y3 * res_2 * x1) + (res_3 * x2 * y1);
    let determinant_z = dpz - dsz;
    let z = determinant_z / determinant;

    (x, y, z)
}
/// Resuelve una ecuación 2x2
pub fn solve_2_determinants(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    res_1: f64,
    res_2: f64,
) -> (f64, f64) {
    let dp = x1 * y2;
    let ds = y1 * x2;
    let determinant = dp - ds;
    let dpx = res_1 * y2;
    let dsx = res_2 * y1;
    let determinant_x = dpx - dsx;
    let x = determinant_x / determinant;
    let dpy = x1 * res_2;
    let dsy = x2 * res_1;
    let determinant_y = dpy - dsy;
    let y = determinant_y / determinant;

    (x, y)
}

pub fn print_3_determinants_result(result: (f64, f64, f64)) {
    println!("x = {}", result.0);
    println!("y = {}", result.1);
    println!("z = {}", result.2);
}
pub fn print_2_determinants_result(result: (f64, f64)) {
    println!("x = {}", result.0);
    println!("y = {}", result.1);
}
#[cfg(test)]
mod tests {
    use super::{solve_2_determinants, solve_3_determinants};
    #[test]
    fn test_2_determinants() {
        let result = solve_2_determinants(3.0, 4.0, 8.0, -9.0, 8.0, -77.0);
        assert_eq!(result, (-4.0, 5.0));
    }
    #[test]
    fn test_3_determinants_2() {
        let result = solve_3_determinants(
            9.0, -5.0, 7.0, 4.0, -3.0, 1.0, -6.0, 6.0, -3.0, -6.0, -5.0, -5.0,
        );
        assert_eq!(result, (-2.0, -3.0, -4.0));
    }
    #[test]
    fn test_3_determinants() {
        let result = solve_3_determinants(
            5.0, 2.0, 1.0, -2.0, 5.0, -4.0, 1.0, -2.0, 3.0, 24.0, -14.0, 26.0,
        );
        assert_eq!(result, (3.0, -2.0, 5.0));
    }
}
