use rand::seq::SliceRandom;
use std::io;

fn main() {
    println!("Enter the numbers, separated by spaces:");

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut array: Vec<i64> = s
        .split_whitespace()
        .map(|d| d.parse::<i64>().unwrap())
        .collect();

    array.shuffle(&mut rand::rng());

    loop {
        array.shuffle(&mut rand::rng());
        println!("{:?}", array);

        //for items in 0..=array.len(){
        //if array[items] < array[items+1]{
        //println!("{:?}",array);
        //}
        //}
        if array.is_sorted() {
            println!("Final array is:");
            println!("{:?}", array);
            break;
        }
    }
}
