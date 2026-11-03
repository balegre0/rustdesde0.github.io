fn sumar(textos: &[&str]) -> Result<i64, String> {
    let mut total = 0;
    for t in textos {
        let n: i64 = t.trim().parse().map_err(|_| format!("«{t}» no es un número"))?;
        total += n;
    }
    Ok(total)
}
