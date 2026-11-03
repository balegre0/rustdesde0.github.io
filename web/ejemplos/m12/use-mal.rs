mod geometria {
    pub fn area(r: f64) -> f64 {
        3.14 * r * r
    }
}

use geometria::superficie;

fn main() {
    println!("{}", superficie(2.0));
}
