use std::io;
//convert temperature from fehrenheit to celcius
pub fn fehrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

//Convert temperature from celcius to fehrenheit
pub fn celcius_to_fehrenheit(c: f64) -> f64 {
    ((c * 9.0) / 5.0) + 32.0
}

pub fn api() {
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
