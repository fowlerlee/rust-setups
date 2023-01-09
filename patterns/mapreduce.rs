use std::thread;

fn map_reduce(data: &[i32], map_func: fn(i32) -> i32, reduce_func: fn(i32, i32) -> i32) -> i32 {
    // Split the data into chunks and apply the map function to each chunk in parallel
    let mapped_data: Vec<i32> = data.chunks(8).into_par_iter().map(|chunk| {
        chunk.iter().map(|x| map_func(*x)).sum()
    }).collect();

    // Apply the reduce function to the mapped data
    mapped_data.into_iter().fold(0, |acc, x| reduce_func(acc, x))
}

// This code defines a map_reduce function that takes a slice of data, a map function, 
// and a reduce function as arguments. It uses the chunks method to split the data into 
// chunks and the into_par_iter method to apply the map function to each chunk in parallel using 
// multiple threads. It then collects the mapped data into a vector and uses the fold method to apply the 
// reduce function to the data. In the main function, the map and reduce functions are defined and the map_reduce 
// function is called with the data and the map and reduce functions as arguments.

fn main() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];

    // Define the map and reduce functions
    let map_func = |x: i32| -> i32 { x * 2 };
    let reduce_func = |x: i32, y: i32| -> i32 { x + y };

    let result = map_reduce(&data, map_func, reduce_func);
    println!("Result: {}", result);
}
