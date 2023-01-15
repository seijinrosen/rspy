use rspy::{reversed, Reversed};

fn main() {
    println!();

    let s = "abcde";
    let reversed_s = s.reversed();
    println!("str          = {:?}", s);
    println!("reversed_str = {:?}", reversed_s);

    println!();

    let s = "abcde".to_string();
    let reversed_s = s.reversed();
    println!("String          = {:?}", s);
    println!("reversed_String = {:?}", reversed_s);

    println!();

    let vec = vec![1, 2, 3, 4, 5];
    let reversed_vec = vec.reversed();
    println!("Vec          = {:?}", vec);
    println!("reversed_Vec = {:?}", reversed_vec);

    println!();

    let array = [1, 2, 3, 4, 5];
    let reversed_array = array.reversed();
    println!("array          = {:?}", array);
    println!("reversed_array = {:?}", reversed_array);

    println!();

    let s = "abcde";
    let reversed_s = reversed(s);
    println!("str          = {:?}", s);
    println!("reversed_str = {:?}", reversed_s);

    println!();

    let s = "abcde".to_string();
    let reversed_s = reversed(&s);
    println!("String          = {:?}", s);
    println!("reversed_String = {:?}", reversed_s);

    println!();

    let vec = vec![1, 2, 3, 4, 5];
    let reversed_vec = reversed(&vec);
    println!("Vec          = {:?}", vec);
    println!("reversed_Vec = {:?}", reversed_vec);

    println!();

    let array = [1, 2, 3, 4, 5];
    let reversed_array = reversed(array);
    println!("array          = {:?}", array);
    println!("reversed_array = {:?}", reversed_array);
}
