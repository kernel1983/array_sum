// use std::panic;
// use num;

fn array_sum(array: &[u32]) -> Option<u32> {
    // let result = panic::catch_unwind(|| {
    //     panic!("oh no!");
    //     // return None
    // });

    let mut sum:u32 = 0;
    for i in array{
        match sum.checked_add(*i) {
            None => {return None}
            Some(t) => {
                sum += t;
                // println!("{}", sum);
            }
        }
    }
    Some(sum)
}

fn main() {
    match array_sum(&[4294967295u32;2]){
        Some(t) => {println!("{}", t);}
        None => {println!("None");}
    }
    match array_sum(&[94967295u32;2]){
        Some(t) => {println!("{}", t);}
        None => {println!("None");}
    }
}
