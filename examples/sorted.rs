use rspy::Iterable;

fn main() {
    let arr = [100, 20, -2000, 60, 0];
    let sorted_arr = arr.sorted();
    println!("arr        = {:?}", arr);
    println!("sorted_arr = {:?}", sorted_arr);
    println!();

    let vec = vec![100, 20, -2000, 60, 0];
    let sorted_vec = vec.sorted();
    println!("vec        = {:?}", arr);
    println!("sorted_vec = {:?}", sorted_vec);
    println!();

    let s = "cbdae".to_string();
    let sorted_s = s.sorted();
    println!("s        = {:?}", s);
    println!("sorted_s = {:?}", sorted_s);
    println!();

    let s = "cbdae";
    let sorted_s = s.sorted();
    println!("s        = {:?}", s);
    println!("sorted_s = {:?}", sorted_s);
}
