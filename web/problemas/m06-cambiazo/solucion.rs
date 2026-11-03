fn retirar_largos(v: &mut Vec<String>, max: usize) -> Vec<String> {
    let mut sacados = Vec::new();
    for i in 0..v.len() {
        if v[i].chars().count() > max {
            sacados.push(std::mem::take(&mut v[i]));
        }
    }
    sacados
}
