use std::thread;

fn main() {
    let palabras = vec![String::from("sol"), String::from("luna")];
    let para_hilo = palabras.clone();
    let h = thread::spawn(move || para_hilo.iter().map(|p| p.len()).sum::<usize>());
    let letras = h.join().unwrap();
    println!("{} palabras, {letras} letras", palabras.len());
}
