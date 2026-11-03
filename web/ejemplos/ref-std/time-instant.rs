use std::time::{Duration, Instant};

fn main() {
    let inicio = Instant::now();
    let mut suma: u64 = 0;
    for i in 0..2_000_000u64 {
        suma = suma.wrapping_add(i * i);
    }
    let pasado = inicio.elapsed();
    println!("suma = {suma}");
    println!("¿tardó menos de un minuto? {}", pasado < Duration::from_secs(60));

    let d = Duration::from_millis(1_500);
    println!("{:?} = {} s y {} ms", d, d.as_secs(), d.subsec_millis());
    println!("{:?}", d * 3 + Duration::from_micros(250));
    println!("{:?}", Duration::from_secs(5).checked_sub(Duration::from_secs(9)));
}
