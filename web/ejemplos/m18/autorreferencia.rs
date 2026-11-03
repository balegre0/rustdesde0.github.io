// 1. Un struct que guarda la dirección de uno de sus propios campos
struct Nota {
    texto: [u8; 4],
    puntero_a_texto: usize, // dirección de `texto`, guardada como número
}

fn main() {
    let mut a = Nota { texto: *b"hola", puntero_a_texto: 0 };
    a.puntero_a_texto = &a.texto as *const _ as usize;
    let ok_antes = a.puntero_a_texto == &a.texto as *const _ as usize;
    println!("antes de mover, ¿apunta a sí mismo?   {ok_antes}");

    // 2. Mover = copiar los bytes a otro sitio. El número guardado NO cambia
    let b = a;
    let caja = Box::new(b);
    let ok_despues = caja.puntero_a_texto == &caja.texto as *const _ as usize;
    println!("después de mover, ¿apunta a sí mismo? {ok_despues}");
}
