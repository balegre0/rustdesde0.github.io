async fn dato() -> u32 {
    7
}

fn main() {
    let n = dato().await;
    println!("{n}");
}
