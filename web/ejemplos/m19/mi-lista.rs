// 1. Una versión casera de vec!
macro_rules! lista {
    () => { Vec::new() };
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $( v.push($x); )+
        v
    }};
}

fn main() {
    let vacia: Vec<i32> = lista![];
    let nums = lista![1, 2, 3];
    let palabras = lista!["sí", "no",];
    println!("{:?} {:?} {:?}", vacia, nums, palabras);
}
