// La forma SIMPLE del agujero #25860 hoy se rechaza. (Solo para estudiarla.)
// Idea: una función cuya firma «presume» que 'b vive más que 'a (porque
// recibe un &'a &'b ()), usada a través de un puntero a función que olvida
// esa presunción.
static UNIDAD: &'static &'static () = &&();

fn traductor<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T {
    v // válido: la firma implica 'b: 'a
}

fn alargar<'a, T>(x: &'a T) -> &'static T {
    let f: fn(&'static &'a (), &'a T) -> &'static T = traductor;
    f(UNIDAD, x)
}

fn main() {
    let r;
    {
        let s = String::from("hola");
        r = alargar(&s);
    }
    let _ = r;
}
