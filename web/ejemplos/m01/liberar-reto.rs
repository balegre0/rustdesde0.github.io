// Mete UNA línea dentro de un bloque { } para que se imprima:
// adiós primero
// fin
// adiós segundo
struct Aviso(&'static str);

impl Drop for Aviso {
    fn drop(&mut self) {
        println!("adiós {}", self.0);
    }
}

fn main() {
    {
        let _a = Aviso("primero");
    } // aquí se acaba _a
    let _b = Aviso("segundo");
    println!("fin");
}
