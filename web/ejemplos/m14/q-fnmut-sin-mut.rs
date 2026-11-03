fn main() {
    let mut c = 0;
    let inc = || c += 1;
    inc();
    println!("{c}");
}
