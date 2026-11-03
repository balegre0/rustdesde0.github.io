fn main() {
    'fuera: for a in 0..3 {
        for b in 0..3 {
            if a + b == 3 {
                break fuera;
            }
        }
    }
}
