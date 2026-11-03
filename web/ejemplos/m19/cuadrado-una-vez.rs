macro_rules! cuadrado {
    ($x:expr) => {{ let v = $x; v * v }};
}

fn siguiente(n: &mut i32) -> i32 {
    *n += 1;
    *n
}

fn main() {
    let mut n = 0;
    let r = cuadrado!(siguiente(&mut n));
    println!("r = {}, n = {}", r, n);
}
