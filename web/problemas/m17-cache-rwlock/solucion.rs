use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

struct Cache {
    datos: RwLock<HashMap<u64, u64>>,
    calculos: AtomicUsize,
}

impl Cache {
    fn new() -> Cache {
        Cache { datos: RwLock::new(HashMap::new()), calculos: AtomicUsize::new(0) }
    }

    fn obtener(&self, n: u64, calcular: impl Fn(u64) -> u64) -> u64 {
        if let Some(v) = self.datos.read().unwrap().get(&n) {
            return *v;
        } // el guard de lectura se suelta aquí
        let mut escritura = self.datos.write().unwrap();
        *escritura.entry(n).or_insert_with(|| {
            self.calculos.fetch_add(1, Ordering::Relaxed);
            calcular(n)
        })
    }

    fn calculos(&self) -> usize {
        self.calculos.load(Ordering::Relaxed)
    }
}
