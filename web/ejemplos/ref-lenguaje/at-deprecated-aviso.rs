#![deny(deprecated)]

#[deprecated(since = "2.0.0", note = "usa `area_rect` en su lugar")]
fn area(a: f64, b: f64) -> f64 {
    a * b
}

fn main() {
    println!("{}", area(2.0, 3.0));
}
