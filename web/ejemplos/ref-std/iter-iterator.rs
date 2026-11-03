// Un iterador propio: solo hay que escribir `next`
struct Cuenta {
    actual: u32,
    fin: u32,
}

impl Iterator for Cuenta {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.actual < self.fin {
            self.actual += 1;
            Some(self.actual)
        } else {
            None
        }
    }
}

fn main() {
    let mut c = Cuenta { actual: 0, fin: 3 };
    // 1. next a mano
    println!("{:?} {:?} {:?} {:?}", c.next(), c.next(), c.next(), c.next());
    // 2. un for llama a next hasta que devuelve None
    for n in (Cuenta { actual: 0, fin: 3 }) {
        print!("{n} ");
    }
    println!();
    // 3. todos los métodos provistos funcionan gratis
    let suma: u32 = Cuenta { actual: 0, fin: 5 }.filter(|n| n % 2 == 1).sum();
    println!("{suma}");
    // 4. iter, iter_mut, into_iter
    let mut v = vec![1, 2, 3];
    for x in v.iter() { print!("{x} "); }
    for x in v.iter_mut() { *x += 10; }
    let dueno: Vec<i32> = v.into_iter().collect();
    println!("{dueno:?}");
}
