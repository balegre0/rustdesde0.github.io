fn main() {
    'fuera: for a in 1..4 {
        for b in 1..4 {
            if a * b == 4 {
                println!("{a} x {b} = 4");
                break 'fuera; // el nombre coincide con la etiqueta
            }
        }
    }
}
