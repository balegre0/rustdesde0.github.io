use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    // Duration: una cantidad de tiempo
    let d = Duration::from_millis(1500);
    println!("{:?} {} {} {}", d, d.as_secs(), d.subsec_millis(), d.as_secs_f64());
    let total = d + Duration::from_secs(2) * 3;
    println!("{:?} {:?}", total, Duration::from_secs(90).checked_sub(Duration::from_secs(100)));
    // Instant: un reloj que solo avanza, para medir
    let inicio = Instant::now();
    let mut suma = 0u64;
    for i in 0..100_000 {
        suma += i;
    }
    let tardado = inicio.elapsed();
    println!("{suma} {}", tardado < Duration::from_secs(10));
    // SystemTime: la fecha y hora del sistema (puede ir hacia atrás)
    let ahora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", ahora.as_secs() > 1_700_000_000);
}
