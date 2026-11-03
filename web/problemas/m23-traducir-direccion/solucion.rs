#[derive(Debug, PartialEq)]
enum Fallo {
    Nivel1,
    Nivel2,
}

fn traducir(virt: u16, tabla1: &[Option<usize>; 16], tablas2: &[[Option<u16>; 16]]) -> Result<u32, Fallo> {
    let i1 = (virt >> 12) as usize & 0xf;
    let i2 = (virt >> 8) as usize & 0xf;
    let desplazamiento = (virt & 0xff) as u32;
    let t2 = tabla1[i1].and_then(|i| tablas2.get(i)).ok_or(Fallo::Nivel1)?;
    let marco = t2[i2].ok_or(Fallo::Nivel2)?;
    Ok(marco as u32 * 256 + desplazamiento)
}
