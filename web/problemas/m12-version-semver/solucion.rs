fn partes(v: &str) -> (u64, u64, u64) {
    let mut it = v.split('.').map(|p| p.parse::<u64>().unwrap_or(0));
    (it.next().unwrap_or(0), it.next().unwrap_or(0), it.next().unwrap_or(0))
}

fn compatible(requisito: &str, version: &str) -> bool {
    let r = partes(requisito);
    let v = partes(version);
    if v < r {
        return false;
    }
    if r.0 > 0 {
        v.0 == r.0
    } else if r.1 > 0 {
        v.0 == 0 && v.1 == r.1
    } else {
        v == r
    }
}
