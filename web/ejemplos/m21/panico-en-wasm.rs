struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop de {}", self.0);
    }
}

fn main() {
    // 1. Al salir normalmente de un ámbito, Drop se ejecuta
    {
        let _x = Ruidoso("x");
    }
    // 2. Ahora un pánico dentro de catch_unwind
    let r = std::panic::catch_unwind(|| {
        let _a = Ruidoso("a");
        panic!("¡algo salió mal!");
    });
    // 3. ¿Llegamos aquí? Depende de la estrategia de pánico del target
    println!("catch_unwind devolvió Err: {}", r.is_err());
}
