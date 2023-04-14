fn main() {
    let temp = farenheit_to_celcius(42.0);
    println!("{}", temp);
}

fn farenheit_to_celcius(faren: f32) -> f32 {
    let celcius = (faren - 32.0) * 5.0 / 9.0;
    return celcius;
}
