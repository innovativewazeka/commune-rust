use rayon::prelude::*;

fn main() {    
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _nums_squared: Vec<i32> = nums
        .par_iter()
        .map(|&x| x * x)
        .collect();
}