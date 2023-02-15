pub fn count_sort(nums: &mut Vec<usize>) {
    let n = nums.iter().max().unwrap();
    let origin_nums = nums.clone();
    let mut count: Vec<usize> = Vec::new();
    for _i in 0..n+1 {
        count.push(0)
    }
    for &v in nums.iter() {
        count[v] += 1;
    }
    for i in 1..count.len() {
        count[i] += count[i-1];
    }
    for &v in origin_nums.iter() {
        nums[count[v]-1] = v;
        count[v] -= 1;
    }
}