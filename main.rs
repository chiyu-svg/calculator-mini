fn main() {
    let number: f64 = 3.14159;
    let number = (number * 1000.0).round() / 1000.0;
    println!("{}", number);
}