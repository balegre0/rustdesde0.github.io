// Ownership = tipos afines: un valor se puede usar COMO MUCHO una vez.
// «Usar» aquí significa consumir (mover). Descartarlo sin usarlo está permitido.
struct Billete {
    valor: u32,
}

fn gastar(b: Billete) -> u32 {
    b.valor
}

fn main() {
    let b1 = Billete { valor: 20 };
    let b2 = Billete { valor: 50 };
    let total = gastar(b1); // b1 se usa una vez: correcto
    // gastar(b1);          // una segunda vez: E0382
    drop(b2);               // b2 se descarta sin usar: también correcto (afín, no lineal)
    println!("gastado: {total}");

    // std::mem::forget es seguro: «olvidar» un valor (no ejecutar su drop) no rompe
    // ninguna garantía de memoria. Por eso Rust es afín y no lineal.
    let b3 = Billete { valor: 5 };
    std::mem::forget(b3);
    println!("olvidado sin problema");
}
