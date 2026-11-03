fn espejo(n: u32) {
    if n == 0 {
        return;
    }
    print!("{n} ");
    espejo(n - 1);
    print!("{n} ");
}

fn main() {
    espejo(3);
    println!();
}
