use std::sync::mpsc::{self, TrySendError};

fn main() {
    // Canal con hueco para 2 mensajes: si está lleno, el emisor tendría que esperar.
    let (tx, rx) = mpsc::sync_channel(2);
    for i in 1..=3 {
        match tx.try_send(i) {
            Ok(()) => println!("enviado {i}"),
            Err(TrySendError::Full(v)) => println!("lleno: no cabe {v}"),
            Err(TrySendError::Disconnected(_)) => println!("nadie escucha"),
        }
    }
    println!("recibo {}", rx.recv().unwrap());
    println!("¿ahora cabe el 3? {}", tx.try_send(3).is_ok());

    drop(rx);
    println!("¿y sin receptor? {:?}", tx.send(4));
}
