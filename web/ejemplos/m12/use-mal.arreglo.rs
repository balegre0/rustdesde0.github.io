mod geometria {
    pub fn area(r: f64) -> f64 {
        3.14 * r * r
    }
}

use geometria::area;

fn main() {
    println!("{}", area(2.0));
}
