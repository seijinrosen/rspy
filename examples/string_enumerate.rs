use rspy::PyString;

fn main() {
    for (i, ch) in "abcde".chars().enumerate() {
        println!("{}: {}", i, ch);
    }

    println!();

    for (i, ch) in "abcde".enumerate(-3) {
        println!("{}: {}", i, ch);
    }
}
