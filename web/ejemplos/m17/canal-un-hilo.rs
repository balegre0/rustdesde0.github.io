use std::sync::mpsc;

fn main() {
    // 1. Un canal tiene dos extremos: el que envía (tx) y el que recibe (rx).
    let (tx, rx) = mpsc::channel();

    // 2. send MUEVE el valor al canal: después ya no es nuestro.
    tx.send(String::from("uno")).unwrap();
    tx.send(String::from("dos")).unwrap();
    tx.send(String::from("tres")).unwrap();

    // 3. Al soltar el último emisor, el canal sabe que no llegará nada más.
    drop(tx);

    // 4. Iterar sobre rx recibe en orden (FIFO) hasta que el canal se cierra.
    for msg in rx.iter() {
        println!("recibido: {msg}");
    }
    println!("canal cerrado");
}
