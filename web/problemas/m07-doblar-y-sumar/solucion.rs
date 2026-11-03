fn doblar(v: &mut [i32]) {
    for x in v {
        *x *= 2;
    }
}

fn suma(v: &[i32]) -> i32 {
    let mut total = 0;
    for x in v {
        total += x;
    }
    total
}
