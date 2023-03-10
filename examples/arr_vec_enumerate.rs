use rspy::enumerate;
use rspy::Iterable;

fn main() {
    let arr = [100, 30, -100, 2000, 50];
    let vec = vec![100, 30, -100, 2000, 50];

    for (i, v) in arr.iter().enumerate() {
        println!("{}: {}", i, v);
    }

    println!();

    for (i, v) in vec.iter().enumerate() {
        println!("{}: {}", i, v);
    }

    println!();

    for (i, v) in arr.enumerate(-3) {
        println!("{}: {}", i, v);
    }

    println!();

    for (i, v) in vec.enumerate(-3) {
        println!("{}: {}", i, v);
    }

    println!();

    for (i, v) in Iterable::enumerate(&arr, -100) {
        println!("{}: {}", i, v);
    }

    println!();

    for (i, v) in arr.enumerate(3) {
        println!("{}: {}", i, v);
    }

    println!();

    for (i, v) in enumerate(&vec, 3) {
        println!("{}: {}", i, v);
    }
}
