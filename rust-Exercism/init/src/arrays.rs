use std::mem;

pub fn run() {
    let mut numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];

    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // get single val

    println!("Single Value: {}", numbers[0]);

    //Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice 
    let slice: &[i32] = &numbers[1..5];
    println!("Slice: {:?}", slice);
}
