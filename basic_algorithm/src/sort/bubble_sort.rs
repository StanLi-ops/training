pub fn bubble_sort(nums: &mut Vec<usize>) {
    for _i in 1..nums.len() {
        for i in 1..nums.len() {
            if nums[i-1] > nums[i] {
                nums.swap(i-1, i);
            }
        }
    }
}