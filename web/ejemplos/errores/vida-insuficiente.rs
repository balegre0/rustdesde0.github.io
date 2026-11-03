fn para_siempre(x: &i32) -> Box<dyn Fn() + 'static> {
    Box::new(move || println!("{x}"))
}

fn main() {
    let n = 5;
    let f = para_siempre(&n);
    f();
}
