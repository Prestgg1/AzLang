#![allow(warnings)]

fn salamla(text: String) {
    println!("{}", text);
}

fn main() {
    let mut hello: String = String::from("Salam");
    const HELLO: &str = "Salam";
    println!("{}", hello);
    salamla(hello);
}
