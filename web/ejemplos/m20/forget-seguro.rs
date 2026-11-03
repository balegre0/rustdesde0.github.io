// Filtrar memoria es SEGURO: ninguna API sound puede depender de que Drop se ejecute.
struct Aviso(&'static str);

impl Drop for Aviso {
    fn drop(&mut self) {
        println!("drop de {}", self.0);
    }
}

fn main() {
    let a = Aviso("a");
    let b = Aviso("b");
    std::mem::forget(a); // 1. sin unsafe: el destructor de `a` no se ejecuta nunca

    let c: &'static mut i32 = Box::leak(Box::new(41)); // 2. también seguro
    *c += 1;
    println!("c = {c}");

    drop(b); // 3. este sí
    println!("fin");
}
