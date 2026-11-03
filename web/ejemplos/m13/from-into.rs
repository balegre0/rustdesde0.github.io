struct Metros(f64);

impl From<f64> for Metros {
    fn from(x: f64) -> Self {
        Metros(x)
    }
}

fn main() {
    let a = Metros::from(2.5);
    let b: Metros = 4.0.into();
    println!("{}", a.0 + b.0);
}
