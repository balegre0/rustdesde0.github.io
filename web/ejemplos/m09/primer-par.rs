fn buscar_par(v: &[i32]) -> Option<i32> {
    for &x in v {
        if x % 2 == 0 {
            return Some(x);
        }
    }
    None
}

fn primer_par(v: &[i32]) -> String {
    let Some(n) = buscar_par(v) else {
        return String::from("sin pares");
    };
    format!("primer par: {n}")
}

fn main() {
    println!("{}", primer_par(&[3, 7, 10, 4]));
    println!("{}", primer_par(&[1, 5]));
}
