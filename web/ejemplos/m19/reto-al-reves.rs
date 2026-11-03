macro_rules! al_reves {
    (@ [] [$($hecho:expr),*]) => { [$($hecho),*] };
    (@ [$primero:expr $(, $resto:expr)*] [$($hecho:expr),*]) => {
        al_reves!(@ [$($resto),*] [$primero $(, $hecho)*])
    };
    ($($x:expr),*) => { al_reves!(@ [$($x),*] []) };
}

fn main() {
    println!("{:?}", al_reves![1, 2, 3]);
    println!("{:?}", al_reves!['a', 'b']);
}
