use rand::{seq::SliceRandom, Rng};

fn main() {
    let mut array: Vec<i64> = vec![1,4,34,35,62];
    array.shuffle(&mut rand::rng());
        
    loop{
        array.shuffle(&mut rand::rng());
        println!("{:?}",array);

        //for items in 0..=array.len(){
            //if array[items] < array[items+1]{
                //println!("{:?}",array);
            //}
        //}
        if array.is_sorted() == true{
            println!("Final array is");
            println!("{:?}",array);
            break;
        }
    }
}
