use std::io;
use std::str::FromStr;

/// Reads a line from stdin and returns the parsing of it into T.
///
/// Before parsing, trims the input string.
///
/// # Examples
///
/// ```no_run
/// println!("Please enter a number: ");
/// let num: i32 = input().unwrap_or_default();
/// println!("Your number is: {}", num);
/// ```
pub fn input<T: FromStr>() -> Result<T, T::Err> {
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    string.trim().parse()
}

/// [input]s T from stdin until the input is valid.
///
/// Each time [input] fails, prints so to the console.
///
/// # Examples
///
/// ```no_run
/// println!("Please enter a number: ");
/// let num: i32 = input_valid();
/// println!("Your number is: {}", num);
/// ```
///
/// Example for what the user could see:
/// ```text
/// Please enter a number:
/// abc
/// Invalid input. Please try again:
/// 3
/// Your number is: 3
/// ```
pub fn input_valid<T: FromStr>() -> T {
    loop {
        match input::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please try again:"),
        }
    }
}

/// Uses [input_valid] to input T and does so until the func_condition
/// returns true.
///
/// When the condition returns false, notifies the user about it.
///
/// # Examples
///
/// ```
/// let nums = vec![1, 2, 3];
/// let index = input_valid_and_also(|x| *x < nums.len());
/// println!("nums[index]={}", nums[index]);
/// ```
pub fn input_valid_and_also<F, T: FromStr>(func_condition: F) -> T
where
    F: Fn(&T) -> bool,
{
    loop {
        let value = input_valid::<T>();
        if func_condition(&value) {
            return value;
        } else {
            println!("Invalid input. Please try again:");
        }
    }
}
