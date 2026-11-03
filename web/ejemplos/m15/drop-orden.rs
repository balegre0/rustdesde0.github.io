struct Ruidoso(&'static str);

// 1. Drop es el código que se ejecuta justo antes de liberar un valor.
impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("adiós, {}", self.0);
    }
}

fn main() {
    let _a = Ruidoso("a");
    let _b = Ruidoso("b");
    {
        let _c = Ruidoso("c");
        println!("fin del bloque interior");
    }
    // 2. drop(x) adelanta la destrucción: mueve x a una función que lo suelta.
    let d = Ruidoso("d");
    drop(d);
    println!("fin de main");
    // 3. Al salir, las variables se destruyen en orden inverso: b, luego a.
}
