use std::cell::OnceCell;

fn main() {
    let celda: OnceCell<String> = OnceCell::new();
    println!("{:?}", celda.get());
    // get_or_init: la closure solo se ejecuta la primera vez
    let v = celda.get_or_init(|| {
        println!("calculando...");
        String::from("valor")
    });
    println!("{v}");
    let v2 = celda.get_or_init(|| String::from("otro"));
    println!("{v2}");
    // set falla si ya tiene valor
    println!("{:?}", celda.set(String::from("nuevo")));
}
