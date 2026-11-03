fn main() {
    let nombre = "ana";
    match nombre {
        "a"..="m" => println!("primera mitad"),
        _ => println!("segunda mitad"),
    }
}
