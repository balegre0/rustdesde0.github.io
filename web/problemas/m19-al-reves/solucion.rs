macro_rules! al_reves {
    (@ [] [$($hecho:expr),*]) => { [$($hecho),*] };
    (@ [$primero:expr $(, $resto:expr)*] [$($hecho:expr),*]) => {
        al_reves!(@ [$($resto),*] [$primero $(, $hecho)*])
    };
    ($($x:expr),+ $(,)?) => { al_reves!(@ [$($x),+] []) };
}
