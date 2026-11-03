fn linea_ticket(producto: &str, unidades: u32, precio: f64) -> String {
    format!("{:<12} x{:>3} {:>8.2} €", producto, unidades, unidades as f64 * precio)
}

fn total(lineas: &[(&str, u32, f64)]) -> String {
    let suma: f64 = lineas.iter().map(|(_, u, p)| *u as f64 * p).sum();
    format!("{:<12}{:>6}{:>8.2} €", "TOTAL", "", suma)
}
