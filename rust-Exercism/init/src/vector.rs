use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last value 
    numbers.pop();
    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);
    // get single val

    println!("Single Value: {}", numbers[0]);

    //Get vector length
    println!("vector Length: {}", numbers.len());

    // vectors are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..5];
    println!("Slice: {:?}", slice);
    //Loop shtough vector values
    for x in numbers.iter(){
        println!("Number: {}",x);
    }
    //Loop & mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }
    println!("Number Vec: {:?}",numbers);
}
