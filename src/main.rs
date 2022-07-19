mod slice;
fn main() {
    println!("Hello, world!");
    let s = String::from("hey how are you doing my friend");
    let first_word_run = slice::first_word(&s);
    println!("{}", first_word_run);
    let (start, end) = slice::second_word(&s);
    println!("The second word starts at {} and ends at {}", start, end);
}
