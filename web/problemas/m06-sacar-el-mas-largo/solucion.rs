fn sacar_mas_largo(v: &mut Vec<String>) -> Option<String> {
    if v.is_empty() {
        return None;
    }
    let mut mejor = 0;
    for i in 1..v.len() {
        if v[i].len() > v[mejor].len() {
            mejor = i;
        }
    }
    Some(v.remove(mejor))
}
