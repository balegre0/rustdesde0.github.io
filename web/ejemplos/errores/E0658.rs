fn main() {
    let r: Result<i32, std::num::ParseIntError> = try {
        "5".parse::<i32>()? + 1
    };
    println!("{r:?}");
}
