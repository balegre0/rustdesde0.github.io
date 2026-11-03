fn movimientos_hanoi(n: u32) -> u64 {
    if n == 0 {
        0
    } else {
        2 * movimientos_hanoi(n - 1) + 1
    }
}
