fn main() {
    let slogan = "hello world";
    let re_slogn: String = slogan.chars().rev().collect();
    println!("{}", re_slogn);
}