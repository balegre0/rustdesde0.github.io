mod banco {
    fn saldo_secreto() -> u32 { 1000 }
}

fn main() {
    // Todo es privado salvo que se marque con `pub`.
    println!("{}", banco::saldo_secreto());
}
