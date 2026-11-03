mod banco {
    fn saldo_secreto() -> u32 {
        1000
    }
}

fn main() {
    println!("{}", banco::saldo_secreto());
}
