const C: u32 = 7;
static S: u32 = 7;

fn main() {
    // Un static tiene una única dirección: siempre la misma
    let a = &S as *const u32 as usize;
    let b = &S as *const u32 as usize;
    println!("static: misma dirección = {}", a == b);
    // Una const se copia allí donde se usa: es un valor, no un sitio
    let x = C;
    let y = C;
    println!("const: {x} {y}");
}
