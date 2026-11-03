struct Inventario {
    items: Vec<String>,
    revisados: usize,
}

impl Inventario {
    fn marcar(&mut self) {
        self.revisados += 1;
    }
}

fn main() {
    let mut inv = Inventario { items: vec![String::from("tornillo"), String::from("tuerca")], revisados: 0 };
    for item in &inv.items {
        println!("revisando {item}");
        inv.marcar();
    }
    println!("{} revisados", inv.revisados);
}
