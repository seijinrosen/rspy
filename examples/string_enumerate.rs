use rspy::Enumerator;

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

    for (i, ch) in rspy::Enumerator::enumerate("abcde", -3) {
        println!("{}: {}", i, ch);
    }
}
