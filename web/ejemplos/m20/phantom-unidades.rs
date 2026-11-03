// PhantomData: «uso un tipo» sin guardar ningún valor de él (tamaño 0).
use std::marker::PhantomData;
use std::ops::Add;

struct Metros;
struct Segundos;

#[derive(Clone, Copy, Debug)]
struct Cantidad<U> {
    valor: f64,
    _unidad: PhantomData<U>, // solo existe para el compilador
}

impl<U> Cantidad<U> {
    fn new(valor: f64) -> Self {
        Cantidad { valor, _unidad: PhantomData }
    }
}

impl<U> Add for Cantidad<U> {
    type Output = Cantidad<U>;
    fn add(self, otra: Self) -> Self {
        Cantidad::new(self.valor + otra.valor)
    }
}

fn main() {
    let a: Cantidad<Metros> = Cantidad::new(3.0);
    let b: Cantidad<Metros> = Cantidad::new(4.5);
    let t: Cantidad<Segundos> = Cantidad::new(2.0);
    println!("{} m", (a + b).valor);
    // a + t  // ← no compila: Metros y Segundos son tipos distintos
    let _ = t;
    println!("size_of Cantidad<Metros> = {} (igual que f64: {})", size_of::<Cantidad<Metros>>(), size_of::<f64>());
    println!("size_of PhantomData<String> = {}", size_of::<PhantomData<String>>());
}
