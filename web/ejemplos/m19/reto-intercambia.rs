macro_rules! intercambia {
    ($a:ident, $b:ident) => {
        let tmp = $a;
        $a = $b;
        $b = tmp;
    };
}

fn main() {
    let mut tmp = 1;
    let mut otro = 2;
    intercambia!(tmp, otro);
    println!("tmp = {}, otro = {}", tmp, otro);
}
