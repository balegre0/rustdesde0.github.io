// Covarianza: donde se pide una vida corta, sirve una larga.
fn mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let fija: &'static str = "constelación"; // vive todo el programa
    let local = String::from("sol");
    // 1. &'static str se «acorta» a la vida de `local`: &'static str <: &'a str
    let r = mas_largo(fija, &local);
    println!("{r}");
    // 2. Lo mismo con Vec<&'static str> → Vec<&'a str> (Vec<T> es covariante en T)
    let v: Vec<&'static str> = vec!["a", "b"];
    let w: Vec<&str> = v;
    println!("{w:?}");
}
