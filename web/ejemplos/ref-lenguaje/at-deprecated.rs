#[deprecated(since = "2.0.0", note = "usa `area_rect` en su lugar")]
fn area(a: f64, b: f64) -> f64 {
    a * b
}

fn area_rect(ancho: f64, alto: f64) -> f64 {
    ancho * alto
}

fn main() {
    #[allow(deprecated)]
    let vieja = area(2.0, 3.0);
    println!("{vieja} {}", area_rect(2.0, 3.0));
}
