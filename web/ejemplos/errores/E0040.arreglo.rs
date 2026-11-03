struct Archivo;

impl Drop for Archivo {
    fn drop(&mut self) {
        println!("cerrado");
    }
}

fn main() {
    let a = Archivo;
    drop(a); // std::mem::drop se lleva la propiedad y destruye el valor ya
    println!("fin");
}
