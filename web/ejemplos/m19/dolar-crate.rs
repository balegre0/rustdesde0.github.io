mod util {
    pub fn doble(n: i32) -> i32 { n * 2 }

    // 1. $crate apunta siempre a la crate donde está definida la macro
    macro_rules! doble_seguro {
        ($e:expr) => { $crate::util::doble($e) };
    }
    pub(crate) use doble_seguro;
}

mod otro {
    // 2. Aunque aquí exista otra función `doble`, la macro llama a la buena
    #[allow(dead_code)]
    fn doble(_: i32) -> i32 { -1 }

    pub fn prueba() -> i32 { crate::util::doble_seguro!(21) }
}

fn main() {
    println!("{}", otro::prueba());
}
