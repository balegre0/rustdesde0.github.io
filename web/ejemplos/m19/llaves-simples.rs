macro_rules! lista {
    ($($x:expr),+) => {
        let mut v = Vec::new();
        $( v.push($x); )+
        v
    };
}
fn main() { let nums = lista![1, 2]; println!("{:?}", nums); }
