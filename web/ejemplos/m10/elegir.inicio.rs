// `dividir_en_grupos(personas, por_grupo)` reparte personas en grupos.
// Decide bien qué es un pánico y qué es un error:
//   - por_grupo == 0 es un FALLO DE QUIEN LLAMA (pánico con assert!: "por_grupo debe ser > 0").
//   - si personas no es múltiplo de por_grupo, es un caso NORMAL: Err(sobran) con las que sobran.
//   - si no, Ok(número de grupos).

fn dividir_en_grupos(personas: u32, por_grupo: u32) -> Result<u32, u32> {
    todo!()
}

fn main() {
    println!("{:?}", dividir_en_grupos(12, 4));
    println!("{:?}", dividir_en_grupos(10, 4));
}
