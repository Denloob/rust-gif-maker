use std::io;
use std::str::FromStr;

pub fn input<T: FromStr>() -> Result<T, T::Err> {
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    string.parse()
}
