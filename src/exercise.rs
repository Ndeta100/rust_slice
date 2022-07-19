use std::io;
//convert temperature from fehrenheit to celcius
pub fn fehrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

//Convert temperature from celcius to fehrenheit
pub fn celcius_to_fehrenheit(c: f64) -> f64 {
    ((c * 9.0) / 5.0) + 32.0
}

pub fn api_temp() {
    println!("Please input a temperature in fehrenheit!");
    let mut fehrenheit = String::new();
    io::stdin()
        .read_line(&mut fehrenheit)
        .expect("Failed to read line");
    let fehrenheit: f64 = match fehrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please input a number not a char"),
    };
    let celcius = fehrenheit_to_celcius(fehrenheit);
    println!("{} F is equal to {} C", fehrenheit, celcius);
}

//Generate the nth fibonacci number
pub fn api_fib() {
    println!("What fibonacci number do you like!");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please input a number not a char"),
    };
    let nth = nth_fib(n);
    println!("The number of {}   {} ", n, nth);
}

pub fn nth_fib(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else if n == 0 {
        return 0;
    }
    nth_fib(n - 1) + nth_fib(n - 2)
}

pub fn determine_temp_from_string(temp_string: &str) -> &str {
    let temp_string_bytes = temp_string.as_bytes();
    for (i, &item) in temp_string_bytes.iter().enumerate() {
        if item == b'F' || item == b'f' {
            return &temp_string[0..i];
        } else if item == b'C' || item == b'c' {
            return &temp_string[0..i];
        }
    }
    &temp_string[..]
}
