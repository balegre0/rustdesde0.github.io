fn leer() -> i32 {
    let n: i32 = "7".parse()?;
    n
}

fn main() {
    println!("{}", leer());
}
