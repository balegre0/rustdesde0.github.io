// El `for` tal como lo ve el compilador después del lowering a HIR.
fn main() {
    let v = [10, 20, 30];

    // 1. La versión con azúcar
    for x in v {
        print!("{x} ");
    }
    println!("(for)");

    // 2. La versión sin azúcar: IntoIterator::into_iter + loop + match sobre next()
    match IntoIterator::into_iter(v) {
        mut iter => loop {
            match Iterator::next(&mut iter) {
                None => break,
                Some(x) => {
                    print!("{x} ");
                }
            }
        },
    }
    println!("(a mano)");
}
