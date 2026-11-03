// La función cuyo MIR leemos en esta lección.
fn doble(x: Option<u32>) -> u32 {
    match x {
        Some(n) => n * 2,
        None => 0,
    }
}

fn main() {
    // 1. Rama Some: bb0 -> bb3 -> bb4 -> bb5
    println!("doble(Some(21)) = {}", doble(Some(21)));
    // 2. Rama None: bb0 -> bb2 -> bb5
    println!("doble(None)     = {}", doble(None));
    // 3. Rama Some con overflow: el `assert` de bb3 falla y entra en pánico
    println!("doble(Some(3_000_000_000)) = {}", doble(Some(3_000_000_000)));
}
