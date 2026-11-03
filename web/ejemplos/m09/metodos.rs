struct Rectangulo {
    ancho: u32,
    alto: u32,
}

// 1. Un bloque `impl` pega funciones (métodos) al tipo.
impl Rectangulo {
    // 2. `&self`: préstamo compartido. Solo lee.
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    // 3. `&mut self`: préstamo exclusivo. Puede cambiar campos.
    fn duplicar(&mut self) {
        self.ancho *= 2;
        self.alto *= 2;
    }

    // 4. `self`: se queda con el valor (lo mueve). Después no se puede usar.
    fn describir(self) -> String {
        format!("rectángulo de {}×{}", self.ancho, self.alto)
    }

    // 5. Un método puede recibir otros argumentos.
    fn cabe_dentro(&self, otro: &Rectangulo) -> bool {
        self.ancho <= otro.ancho && self.alto <= otro.alto
    }
}

fn main() {
    let mut r = Rectangulo { ancho: 3, alto: 4 };
    println!("área = {}", r.area());
    r.duplicar();
    println!("área tras duplicar = {}", r.area());

    let caja = Rectangulo { ancho: 10, alto: 10 };
    println!("¿cabe en la caja? {}", r.cabe_dentro(&caja));

    // 6. `r.area()` es azúcar de `Rectangulo::area(&r)`.
    println!("lo mismo, sin azúcar: {}", Rectangulo::area(&r));

    let texto = r.describir();
    println!("{}", texto);
}
