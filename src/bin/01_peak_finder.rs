
fn peak_finder(start: usize, end: usize ,vector: Vec<i32>) -> i32 {
    let middle = (end - start) / 2;
    if vector[middle] > vector[middle -1] && vector[middle] > vector[middle +1] {
        return vector[middle]
    }
    return 0
}


fn main() {
    println!("----- PEAK FINDER ALGO MIT LECTURE 01 -----");
    let stack =  vec![1,2,3,10,5,6,7];
    let start = 0;
    let end = stack.len() -1 ;
    let result = peak_finder(start, end, stack);
    println!("PEAK VALUE IS: {}", result);
}