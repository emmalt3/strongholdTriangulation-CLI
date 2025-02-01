#![allow(non_snake_case)]

use std::io;

fn triangulateStronghold(
    x1: f64, z1: f64, theta1: f64, 
    x2: f64, z2: f64, theta2: f64
) -> Result<(f64, f64), String> {

    let rad1 = (90.0 - theta1).to_radians();
    let rad2 = (90.0 - theta2).to_radians();

    let dx1 = rad1.sin();
    let dz1 = rad1.cos();
    let dx2 = rad2.sin();
    let dz2 = rad2.cos();

    let determinant = dx1 * dz2 - dx2 * dz1;

    if determinant.abs() < 1e-6 {
        return Err("error: no stronghold found within region".to_string());
    }

    let x = (dz2 * (dx1 * x1 + dz1 * z1) - dz1 * (dx2 * x2 + dz2 * z2)) / determinant;
    let z = (dx1 * (dx2 * x2 + dz2 * z2) - dx2 * (dx1 * x1 + dz1 * z1)) / determinant;

    Ok((x, z))
}

fn interpretInput(prompt: &str) -> Result<(f64, f64, f64), String> {
    let mut input = String::new();
    println!("{}", prompt);

    io::stdin().read_line(&mut input).map_err(|_| "failed to interpret input".to_string())?;

    let values: Vec<f64> = input.trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if values.len() != 3 {
        return Err("invalid input. please input three numerical values seperated by spaces".to_string());
    }

    Ok((values[0], values[1], values[2]))
}

fn main() {
    let (x1, z1, theta1) = match interpretInput("input x1, z1, theta1:") {
        Ok(values) => values,
        Err(e) => return println!("{}", e),
    };

    let (x2, z2, theta2) = match interpretInput("input x2, z2, theta2:") {
        Ok(values) => values,
        Err(e) => return println!("{}", e),
    };

    match triangulateStronghold(x1, z1, theta1, x2, z2, theta2) {
        Ok((x, z)) => println!("stronghold triangulated at: x = {:.0}, z = {:.0}", x, z),
        Err(e) => println!("{}", e),
    }
}

