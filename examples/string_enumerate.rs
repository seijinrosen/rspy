use rspy::enumerate;
use rspy::Iterable;

fn main() {
    for (i, ch) in "abcde".chars().enumerate() {
        println!("{}: {}", i, ch);
    }

    println!();

    for (i, ch) in "abcde".enumerate(-3) {
        println!("{}: {}", i, ch);
    }

    println!();

    let s = "abcde".to_string();
    for (i, ch) in s.enumerate(0) {
        println!("{}: {}", i, ch);
    }

    println!();

    for (i, ch) in rspy::Iterable::enumerate("abcde", -3) {
        println!("{}: {}", i, ch);
    }

    println!();

    let s = "abcde".to_string();
    for (i, ch) in Iterable::enumerate(&s, -100) {
        println!("{}: {}", i, ch);
    }

    println!();

    for (i, ch) in enumerate(&s, 10000) {
        println!("{}: {}", i, ch);
    }
}
