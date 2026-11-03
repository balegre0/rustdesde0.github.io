use std::collections::BTreeMap;

fn mas_frecuente(texto: &str) -> Option<(String, usize)> {
    let mut cuenta: BTreeMap<String, usize> = BTreeMap::new();
    for p in texto.split_whitespace() {
        *cuenta.entry(p.to_lowercase()).or_insert(0) += 1;
    }
    let mut mejor: Option<(String, usize)> = None;
    for (p, n) in cuenta {
        // BTreeMap recorre en orden alfabético: solo se sustituye con una cuenta ESTRICTAMENTE mayor.
        if mejor.as_ref().is_none_or(|(_, m)| n > *m) {
            mejor = Some((p, n));
        }
    }
    mejor
}
