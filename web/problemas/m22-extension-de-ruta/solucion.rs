use std::collections::BTreeMap;
use std::path::Path;

fn contar_por_extension(rutas: &[&str]) -> BTreeMap<String, usize> {
    let mut cuenta = BTreeMap::new();
    for r in rutas {
        let ext = Path::new(r)
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_else(|| String::from("(ninguna)"));
        *cuenta.entry(ext).or_insert(0) += 1;
    }
    cuenta
}
