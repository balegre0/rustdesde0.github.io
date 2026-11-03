struct Inventario {
    items: Vec<String>,
    revisados: usize,
}

fn main() {
    let mut inv = Inventario { items: vec![String::from("tornillo"), String::from("tuerca")], revisados: 0 };
    for item in &inv.items {
        println!("revisando {item}");
        inv.revisados += 1; // campo distinto de `items`: préstamos disjuntos
    }
    println!("{} revisados", inv.revisados);
}
