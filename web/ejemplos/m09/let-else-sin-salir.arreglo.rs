fn main() {
    let entrada = "abc";
    let Ok(n) = entrada.parse::<i32>() else {
        println!("no es un número");
        return;
    };
    println!("{n}");
}
