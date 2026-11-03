use std::sync::mpsc::{self, Receiver, Sender};

fn etapa(rx1: Receiver<u32>, tx2: Sender<u32>) {
    todo!()
}

fn tuberia(entrada: &[u32]) -> Vec<u32> {
    let (_tx1, _rx1) = mpsc::channel::<u32>();
    todo!()
}
