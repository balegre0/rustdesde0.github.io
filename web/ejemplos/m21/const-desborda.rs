const fn factorial(n: u64) -> u64 {
    let mut r = 1;
    let mut i = 2;
    while i <= n {
        r *= i;
        i += 1;
    }
    r
}

// 21! no cabe en un u64: el intérprete de MIR detecta el overflow AL COMPILAR.
const F21: u64 = factorial(21);

fn main() {
    println!("{F21}");
}
