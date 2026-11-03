async fn cuenta() -> u32 {
    3
}

fn main() {
    // Una función async no devuelve u32, sino un futuro que lo producirá.
    let n: u32 = cuenta();
    println!("{n}");
}
