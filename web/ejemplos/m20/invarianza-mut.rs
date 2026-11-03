// &mut T es INVARIANTE en T. Si no lo fuera, esto crearía una referencia colgante.
fn meter<'a>(destino: &mut &'a str, valor: &'a str) {
    *destino = valor;
}

fn main() {
    let mut global: &'static str = "para siempre";
    {
        let corta = String::from("efímera");
        meter(&mut global, &corta); // ¿meter algo corto donde se espera 'static?
    }
    println!("{global}");
}
