fn dividir_en_grupos(personas: u32, por_grupo: u32) -> Result<u32, u32> {
    assert!(por_grupo > 0, "por_grupo debe ser > 0");
    let sobran = personas % por_grupo;
    if sobran != 0 {
        return Err(sobran);
    }
    Ok(personas / por_grupo)
}

fn main() {
    println!("{:?}", dividir_en_grupos(12, 4));
    println!("{:?}", dividir_en_grupos(10, 4));
}
