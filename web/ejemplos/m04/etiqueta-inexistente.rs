fn main() {
    'fuera: for a in 1..4 {
        for b in 1..4 {
            if a * b == 4 {
                break 'afuera;
            }
        }
    }
}
