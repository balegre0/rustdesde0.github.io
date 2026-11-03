// x86_64: una dirección virtual de 48 bits se parte en 4 índices de 9 bits
// (uno por nivel de tabla de páginas) y un desplazamiento de 12 bits dentro de la página.
fn partir(virt: u64) -> ([u64; 4], u64) {
    let desplazamiento = virt & 0xfff;
    let mut indices = [0u64; 4];
    for nivel in 0..4 {
        // nivel 0 = P4 (el más alto) … nivel 3 = P1
        let desp = 12 + 9 * (3 - nivel);
        indices[nivel] = (virt >> desp) & 0x1ff;
    }
    (indices, desplazamiento)
}

fn main() {
    for virt in [0x0000_0000_0000_1234u64, 0x0000_7fff_ffff_f000, 0x0000_0080_4020_1abc] {
        let ([p4, p3, p2, p1], d) = partir(virt);
        println!("{virt:#018x} → P4={p4:3} P3={p3:3} P2={p2:3} P1={p1:3} desplazamiento={d:#05x}");
    }
    println!("tamaño de página: {} bytes · entradas por tabla: {}", 1u64 << 12, 1u64 << 9);
}
