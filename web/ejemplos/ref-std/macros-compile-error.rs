#[cfg(not(any(feature = "rapido", feature = "seguro")))]
compile_error!("activa la feature «rapido» o la feature «seguro»");

fn main() {
    println!("modo elegido");
}
