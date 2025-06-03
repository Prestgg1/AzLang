fn salamla() {
    println!("Salam");
}

fn QiymetVer(a: usize, b: usize) {
    println!("Qiymət: {}", a);
}

fn main() {
    salamla();
    QiymetVer(1, 2);
    let mut e: usize = 1;
    const F: usize = 2;
    println!("Qiymət: {} {}  {}", e+F, e, F);
}
