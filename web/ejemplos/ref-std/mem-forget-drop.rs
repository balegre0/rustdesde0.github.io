use std::mem;

struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop de {}", self.0);
    }
}

fn main() {
    let a = Ruidoso("a");
    let b = Ruidoso("b");
    let c = Ruidoso("c");
    drop(a); // se destruye ya
    mem::forget(b); // nunca se destruye: su drop no se ejecuta
    println!("fin de main");
    let _ = &c;
} // aquí se destruye c
