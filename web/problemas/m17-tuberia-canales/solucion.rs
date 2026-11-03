use std::sync::mpsc::{self, Receiver, Sender};

fn etapa(rx1: Receiver<u32>, tx2: Sender<u32>) {
    for x in rx1 {
        if x % 2 == 0 {
            tx2.send(x * 10).unwrap();
        }
    }
} // tx2 se destruye aquí: el canal 2 queda cerrado

fn tuberia(entrada: &[u32]) -> Vec<u32> {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    for &x in entrada {
        tx1.send(x).unwrap();
    }
    drop(tx1);
    etapa(rx1, tx2);
    rx2.iter().collect()
}
