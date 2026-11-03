use std::thread;

fn main() {
    let palabras = vec![String::from("sol"), String::from("luna")];
    // Este programa no compila: arréglalo para que el hilo cuente las letras
    // y main pueda seguir usando `palabras` después.
    let h = thread::spawn(|| palabras.iter().map(|p| p.len()).sum::<usize>());
    let letras = h.join().unwrap();
    println!("{} palabras, {letras} letras", palabras.len());
}
