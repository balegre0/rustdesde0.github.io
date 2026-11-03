fn pasos_collatz(n: u64) -> u32 {
    let mut x = n;
    let mut pasos = 0;
    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        pasos += 1;
    }
    pasos
}
