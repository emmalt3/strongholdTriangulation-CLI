use std::f64::consts::PI;
use std::io;

fn triangulateStronghold(
    x1: f64, z1: f64, theta1: f64, 
    x2: f64, z2: f64, theta2: f64
    ) -> Result<(f64, f64), String> {

    let m1 = (theta1 * PI / 180.0).tan();
    let m2 = (theta2 * PI / 180.0).tan();

    let det = m1 - m2;
    if det.abs() < 1e-6 {
        return Err("error: no stronghold found within region".to_string());
    }

    let x = (m1 * x1 - m2 * x2 + z2 - z1) / det;

    let z = m1 * (x - x1) + z1;

   Ok((x, z))
}

fn interpretInput(prompt: &str) -> Result<(f64, f64, f64), String> {
    let mut input = String::new();

    println!(prompt);
    io:stdin().read_line(&mut input).map_err(|_| "failed to interpret input".to_string())?;

    let values: Vec<f64> = input.trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if values.len() != 3 {
        return Err("invalid input, enter three numbers seperated by spaces".to_string());
    }

    Ok((values[0], values[1], values[2]))
}

fn main() {
    match interpretInput("input x1, z1, theta1:") {
        Ok((x1, z1, theta1)) => {
            match interpretInput("input x2, z2, theta2:") {
                Ok((x2, z2, theta2)) => {
                    match triangulateStronghold(x1, z1, theta1, x2, z2, theta2) {
                        Ok((x, z)) => println!("strongold triangulated at: x = {:.2}, z = {2:.2}", x, z),
                        Err(e) => println!(e),
                    }
                }
                Err(e) => println!(e),
            }
        }
        Err(e) => println!(e),
    }
}

