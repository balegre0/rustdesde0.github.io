use std::mem::size_of_val;

async fn paso() {}

async fn hoja() -> u8 {
    let buf = [1u8; 64];
    paso().await;
    buf[0]
}

// Espera a `hoja`: el future de hoja viaja DENTRO del de rama
async fn rama() -> u8 {
    hoja().await + 1
}

// Lo mismo, pero el future de hoja va en una caja en el heap
async fn rama_en_caja() -> u8 {
    Box::pin(hoja()).await + 1
}

fn main() {
    println!("hoja: {}", size_of_val(&hoja()));
    println!("rama: {}", size_of_val(&rama()));
    println!("rama_en_caja: {}", size_of_val(&rama_en_caja()));
}
