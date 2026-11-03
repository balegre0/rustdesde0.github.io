// Un iterador que va de `actual` hasta `fin` (sin incluirlo) dando saltos de `paso`.
struct Pasos {
    actual: u32,
    fin: u32,
    paso: u32,
}

impl Iterator for Pasos {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        todo!()
    }
}

fn main() {
    let v: Vec<u32> = Pasos { actual: 0, fin: 20, paso: 5 }.collect();
    println!("{v:?}");
    let suma: u32 = Pasos { actual: 1, fin: 10, paso: 3 }.sum();
    println!("{suma}");
    println!("{}", Pasos { actual: 7, fin: 7, paso: 1 }.count());
}
