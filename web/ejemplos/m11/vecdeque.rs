use std::collections::VecDeque;
use std::mem::size_of;

fn main() {
    // 1. Una cola: se entra por detrás y se sale por delante.
    let mut turnos: VecDeque<u32> = VecDeque::new();
    for n in 1..=4 {
        turnos.push_back(n);
    }
    while let Some(t) = turnos.pop_front() {
        print!("atiendo {t}; ");
    }
    println!();

    // 2. Por los dos extremos: push_front / push_back / pop_front / pop_back.
    let mut d: VecDeque<char> = VecDeque::from(['b', 'c']);
    d.push_front('a');
    d.push_back('d');
    println!("{:?} primero={:?} último={:?}", d, d.front(), d.back());

    // 3. Una ventana de las últimas 3 medidas: lo viejo sale por delante.
    let mut ultimas: VecDeque<f64> = VecDeque::with_capacity(3);
    for medida in [20.0, 21.5, 19.0, 23.0, 22.5] {
        if ultimas.len() == 3 {
            ultimas.pop_front();
        }
        ultimas.push_back(medida);
        let media: f64 = ultimas.iter().sum::<f64>() / ultimas.len() as f64;
        println!("últimas {:?} → media {:.2}", ultimas, media);
    }

    // 4. Por dentro es un búfer circular: puede quedar partido en dos trozos.
    let mut r: VecDeque<u8> = VecDeque::with_capacity(4);
    r.push_back(1);
    r.push_back(2);
    r.push_back(3);
    r.pop_front();
    r.push_back(4);
    r.push_back(5);
    println!("{:?} se guarda en dos trozos: {:?}", r, r.as_slices());
    println!("size_of::<VecDeque<u8>>() = {} bytes", size_of::<VecDeque<u8>>());
}
