fn main() {
    let o: Option<Option<i32>> = Some(None);
    match o {
        Some(Some(v)) => println!("v={v}"),
        Some(None) => println!("vacío dentro"),
        None => println!("nada"),
    }
    let c = 'Ñ';
    let k = match c {
        'a'..='z' => "minúscula ASCII",
        'A'..='Z' => "mayúscula ASCII",
        _ => "otra",
    };
    println!("{k}");
}
