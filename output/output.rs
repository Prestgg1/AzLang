use std::io::Write;

fn main() {
    const text: &str = "Salam";
    fn salamla(texts: &str) {
println!("{:?}", texts);
}
    const ədədlər: &[usize] = &[1, 2, 3];
    println!("{:?}", ədədlər);
    println!("{:?}", ədədlər.len());
    let mut ad: String = {
                print!("{}", "Ad gir: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input.trim().to_string()
            };
    println!("{:?}", ad);
}
