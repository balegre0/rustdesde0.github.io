// Pruebas de propiedades: en vez de elegir tú los ejemplos, describes algo
// que debe cumplirse SIEMPRE y lo compruebas con muchos datos generados.

struct Azar(u64);

impl Azar {
    fn siguiente(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn ordenar(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}

fn main() {
    let mut azar = Azar(7);
    let mut casos = 0;
    for _ in 0..1000 {
        // 1. Generamos una lista aleatoria de 0 a 19 números entre -50 y 49.
        let largo = (azar.siguiente() % 20) as usize;
        let v: Vec<i32> = (0..largo).map(|_| (azar.siguiente() % 100) as i32 - 50).collect();
        let o = ordenar(v.clone());

        // 2. Propiedad A: no se pierde ni se inventa ningún elemento.
        assert_eq!(o.len(), v.len());
        // 3. Propiedad B: cada elemento es <= que el siguiente.
        assert!(o.windows(2).all(|par| par[0] <= par[1]), "desordenado: {o:?}");
        // 4. Propiedad C: ordenar dos veces da lo mismo que una.
        assert_eq!(ordenar(o.clone()), o);
        casos += 1;
    }
    println!("{casos} listas aleatorias: las 3 propiedades se cumplen");
}
