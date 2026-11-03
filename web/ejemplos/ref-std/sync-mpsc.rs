use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // varios productores
    tx.send(String::from("hola")).unwrap();
    tx2.send(String::from("adiós")).unwrap();
    println!("{:?}", rx.recv());
    println!("{:?}", rx.try_recv());
    println!("{:?}", rx.try_recv());
    drop(tx);
    drop(tx2);
    // sin emisores vivos, recv devuelve Err
    println!("{:?}", rx.recv());
}
