use basic_algorithm::*;

fn main() {
    let nums: &mut Vec<usize> =&mut vec![1, 4, 6767, 5456, 413, 4679, 11135, 4635, 45387, 198];
    println!("{:?}", nums);

    // quick_sort(nums);
    // bubble_sort(nums);
    // insert_sort(nums);
    // shell_sort(nums);
    // selection_sort(nums);
    insert_sort(nums);
    // merge_sort(nums);

    println!("{:?}", nums);
}
