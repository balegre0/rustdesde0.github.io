pub trait Area {
    fn area(&self) -> f64 { 0.0 }
}

macro_rules! impl_area {
    ($($tipo:ty, $s:ident => $formula:expr);* $(;)?) => {
        // cámbialo: el método por defecto devuelve 0.0
        $( impl Area for $tipo {} )*
    };
}
