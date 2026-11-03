// Curry–Howard «light»: un tipo es una proposición y un valor de ese tipo, una prueba.
// Una función de A en B es una prueba de «si A, entonces B».

// Modus ponens: si tengo A, y tengo «A → B», tengo B.
fn modus_ponens<A, B>(a: A, f: impl FnOnce(A) -> B) -> B {
    f(a)
}

// «A y B» implica «A»: una tupla es una conjunción.
fn primero<A, B>(par: (A, B)) -> A {
    par.0
}

// «A o B» implica «B o A»: un Result es una disyunción.
fn conmutar<A, B>(r: Result<A, B>) -> Result<B, A> {
    match r {
        Ok(a) => Err(a),
        Err(b) => Ok(b),
    }
}

fn main() {
    let longitud = modus_ponens(String::from("hola"), |s| s.len());
    println!("{longitud}");
    println!("{}", primero(("prueba de A", 3.14)));
    println!("{:?}", conmutar::<i32, &str>(Ok(7)));
}
