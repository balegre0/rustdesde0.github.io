#[derive(Debug, PartialEq)]
enum Fallo {
    Nivel1,
    Nivel2,
}

fn traducir(virt: u16, tabla1: &[Option<usize>; 16], tablas2: &[[Option<u16>; 16]]) -> Result<u32, Fallo> {
    todo!()
}
