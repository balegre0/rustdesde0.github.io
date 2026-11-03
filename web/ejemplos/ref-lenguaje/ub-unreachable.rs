// `unreachable_unchecked` promete que esa línea nunca se alcanza.
// Alcanzarla es UB (un «intrínseco» usado mal).
const fn mitad(x: u32) -> u32 {
    if x % 2 == 0 { x / 2 } else { unsafe { std::hint::unreachable_unchecked() } }
}
const MALO: u32 = mitad(3);

fn main() {
    println!("{MALO}");
}
