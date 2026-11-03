use std::thread;

fn main() {
    let nombre = String::from("Ana");
    // `move`: el closure se queda con la PROPIEDAD de `nombre`.
    let h = thread::spawn(move || {
        println!("hola, {nombre}");
    });
    h.join().unwrap();
}
