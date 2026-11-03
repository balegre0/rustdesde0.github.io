use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

// Un trabajo es cualquier closure que se pueda ejecutar una vez en otro hilo.
type Trabajo = Box<dyn FnOnce() + Send + 'static>;

struct Piscina {
    emisor: Option<Sender<Trabajo>>,
    obreros: Vec<JoinHandle<()>>,
}

impl Piscina {
    fn new(n: usize) -> Piscina {
        let (tx, rx) = mpsc::channel::<Trabajo>();
        let rx: Arc<Mutex<Receiver<Trabajo>>> = Arc::new(Mutex::new(rx));
        let obreros = (0..n)
            .map(|_| {
                let rx = Arc::clone(&rx);
                thread::spawn(move || loop {
                    // 1. Recibe un trabajo con el candado tomado el MENOR tiempo posible.
                    // 2. Si llega Ok(trabajo), ejecútalo; si llega Err, sal del bucle.
                    let _ = &rx;
                    todo!("bucle del obrero")
                })
            })
            .collect();
        Piscina { emisor: Some(tx), obreros }
    }

    fn ejecutar(&self, f: impl FnOnce() + Send + 'static) {
        self.emisor.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for Piscina {
    fn drop(&mut self) {
        // 3. Cierra el canal y espera (join) a todos los obreros.
        todo!("apagado ordenado")
    }
}

fn main() {
    let (tx_res, rx_res) = mpsc::channel();
    {
        let piscina = Piscina::new(3);
        for n in 1..=6u64 {
            let tx_res = tx_res.clone();
            piscina.ejecutar(move || {
                let factorial: u64 = (1..=n).product();
                tx_res.send((n, factorial)).unwrap();
            });
        }
    }
    drop(tx_res);

    let mut resultados: Vec<(u64, u64)> = rx_res.iter().collect();
    resultados.sort();
    for (n, f) in resultados {
        println!("{n}! = {f}");
    }
}
