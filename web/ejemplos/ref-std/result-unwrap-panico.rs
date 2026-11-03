fn main() {
    let r: Result<u8, _> = "300".parse::<u8>();
    println!("{r:?}");
    let n = r.unwrap();
    println!("{n}");
}
