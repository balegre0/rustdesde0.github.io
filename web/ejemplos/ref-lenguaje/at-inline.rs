// #[inline] sugiere copiar el cuerpo en cada llamada; #[inline(never)] lo prohíbe.
#[inline]
fn doble(x: u32) -> u32 {
    x * 2
}

#[inline(always)]
fn es_par(x: u32) -> bool {
    x % 2 == 0
}

#[inline(never)]
fn registrar(x: u32) {
    println!("registro: {x}");
}

fn main() {
    let mut pares = 0;
    for i in 0..10 {
        if es_par(doble(i) / 2) {
            pares += 1;
        }
    }
    registrar(pares);
}
