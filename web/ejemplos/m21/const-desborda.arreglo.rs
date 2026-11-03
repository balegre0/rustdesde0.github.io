const fn factorial(n: u128) -> u128 {
    let mut r = 1;
    let mut i = 2;
    while i <= n {
        r *= i;
        i += 1;
    }
    r
}

// Con u128 sí cabe 21!
const F21: u128 = factorial(21);

fn main() {
    println!("{F21}");
}
