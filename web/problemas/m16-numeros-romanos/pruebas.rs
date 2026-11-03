fn main() {
    let mut fallos = 0;
    let fijos = [(1, "I"), (4, "IV"), (9, "IX"), (14, "XIV"), (40, "XL"), (1994, "MCMXCIV"), (2026, "MMXXVI"), (3999, "MMMCMXCIX")];
    for (n, r) in fijos {
        if a_romano(n) == r && de_romano(r) == Some(n) {
            println!("OK: {n} <-> {r}");
        } else {
            println!("FALLO: {n} <-> {r}: a_romano dio {:?}, de_romano dio {:?}", a_romano(n), de_romano(r));
            fallos += 1;
        }
    }
    for malo in ["IIII", "IC", "VV", "", "ABC", "MMMM", "iv"] {
        if de_romano(malo).is_none() {
            println!("OK: {malo:?} rechazado");
        } else {
            println!("FALLO: {malo:?} debía ser None, dio {:?}", de_romano(malo));
            fallos += 1;
        }
    }
    let rotos: Vec<u32> = (1..=3999).filter(|&n| de_romano(&a_romano(n)) != Some(n)).collect();
    if rotos.is_empty() {
        println!("OK: propiedad de ida y vuelta para 1..=3999");
    } else {
        println!("FALLO: la ida y vuelta falla en {} números, el primero {}", rotos.len(), rotos[0]);
        fallos += 1;
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
