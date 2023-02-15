pub fn selection_sort(nums: &mut Vec<usize>) {
    for i in 0..nums.len()-1 {
        let mut p = i;
        for j in i+1..nums.len() {
            if nums[j] < nums[p] {
                p = j;
            }
        }
        nums.swap(i, p);
    }
}