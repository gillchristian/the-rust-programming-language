fn main() {
    let s = String::from("hello");
    let s2 = String::from("bye");
    let s3 = String::from("hey there!");

    let (s, len) = calculate_length(s);

    println!("The length of '{}' is {}.", s, len);

    takes_ownershipt(s2);

    // println!("This wouldn't work becase owndership of s2 ({}) was give away", s2);

    borrows(&s3);

    println!("Yay s3 (\"{}\") is still here!", s3);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn takes_ownershipt(s: String) {
  println!("s (s=\"{}\") is now mineeee!", s);
}

fn borrows(s: &String) {
  println!("Hey I just borrowed it, I'll use it here \"{}\"", s);

  println!("Now take it back");
}

// more examples here:
// @link https://repl.it/@ChristianGill/mut-immut-burrows
