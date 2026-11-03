macro_rules! unidad {
    (metros) => { 1.0 };
    (km) => { 1000.0 };
    ($otra:ident) => {
        compile_error!(concat!("unidad desconocida: ", stringify!($otra)))
    };
}

fn main() {
    let d = 3.0 * unidad!(km);
    println!("{}", d);
}
