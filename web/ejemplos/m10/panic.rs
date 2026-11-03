fn dividir_en_partes(total: u32, partes: u32) -> u32 {
    // 1. Comprobar una condición que, si falla, es un FALLO DEL PROGRAMA.
    assert!(partes > 0, "nadie divide entre 0 partes (total = {total})");
    total / partes
}

struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  (se libera {})", self.0);
    }
}

fn main() {
    let _a = Ruidoso("a");
    println!("10 en 2 partes: {}", dividir_en_partes(10, 2));

    // 2. Pánicos que lanza la propia biblioteca estándar:
    //    índice fuera de rango, unwrap de None, división entera entre 0, desbordamiento (en debug)…
    let v = vec![1, 2, 3];
    println!("v[1] = {}", v[1]);

    // 3. Y el tuyo propio, con panic!:
    println!("ahora sí, a propósito:");
    dividir_en_partes(10, 0);

    println!("esto ya no se ejecuta");
}
