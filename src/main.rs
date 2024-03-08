use std::io;

fn solve_quadratic(a: f64, b: f64, c: f64) -> Result<(f64, f64), io::Error> {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "discriminant is negative",
        )
     .into());
    }
    let sqrt_discriminant = discriminant.sqrt();
    Ok(((-b - sqrt_discriminant) / 2.0 / a, (-b + sqrt_discriminant) / 2.0 / a))
}

fn main() {
    println!("Quadratic equations solver");
    println!("Input a, b, c of ax^2 + bx + c = 0 equation:");
    let mut coefficients = String::new();

    io::stdin()
        .read_line(&mut coefficients)
        .expect("Failed to read line");

    if let [Ok(a), Ok(b), Ok(c)] = coefficients.trim().split(" ")
                                        .map(|coefficients| coefficients.parse::<f64>())
                                        .collect::<Vec<_>>()[..] {
        println!("Coefficients: a = {}, b = {}, c = {}", a, b, c);

        if let Ok((x1, x2)) = solve_quadratic(a, b, c) {
            println!("Solutions: x1 = {}, x2 = {}", x1, x2);
        } else {
            println!("No solutions");
        }
    }
    else {
        println!("Failed to parse coefficients");
    }
}