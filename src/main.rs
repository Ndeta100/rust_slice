mod exercise;
mod slice;
fn main() {
    println!("Hello, world!");
    let s = String::from("hey how are you doing my friend");
    let first_word_run = slice::first_word(&s);
    println!("{}", first_word_run);
    let (start, end) = slice::second_word(&s);
    println!("The second word starts at {} and ends at {}", start, end);
    let word_is = slice::second_word_is(&s);
    println!("The second word is {}", word_is);
    let f1 = exercise::celcius_to_fehrenheit(46.4);
    let f2 = exercise::fehrenheit_to_celcius(76.7);
    //println!("{}     {}", f1, f2);
    //exercise::api();
    exercise::api_fib();
}
