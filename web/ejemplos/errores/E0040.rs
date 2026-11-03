struct Archivo;

impl Drop for Archivo {
    fn drop(&mut self) {
        println!("cerrado");
    }
}

fn main() {
    let a = Archivo;
    a.drop();
    println!("fin");
}
