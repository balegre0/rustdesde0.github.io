fn main() {
    let mut cuenta = 0;
    'a: for i in 0..4 {
        for j in 0..4 {
            if j == 2 {
                continue 'a;
            }
            cuenta += 1;
            let _ = i;
        }
    }
    println!("{cuenta}");
}
