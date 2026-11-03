use std::marker::PhantomData;

struct Metros;
struct Pies;

#[derive(Debug, Clone, Copy)]
struct Longitud<U> {
    valor: f64,
    _unidad: PhantomData<U>,
}

impl<U> Longitud<U> {
    fn new(valor: f64) -> Self {
        Longitud { valor, _unidad: PhantomData }
    }
    fn sumar(self, otra: Longitud<U>) -> Longitud<U> {
        Longitud::new(self.valor + otra.valor)
    }
}

fn main() {
    let a: Longitud<Metros> = Longitud::new(3.0);
    let b: Longitud<Metros> = Longitud::new(4.5);
    println!("{}", a.sumar(b).valor);
    let _p: Longitud<Pies> = Longitud::new(1.0);
    // a.sumar(_p) no compila: metros y pies son tipos distintos
    println!("{} bytes", size_of::<Longitud<Metros>>());
    println!("{} bytes", size_of::<PhantomData<String>>());
}
