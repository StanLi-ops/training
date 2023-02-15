use std::collections::VecDeque;

pub fn shell_sort(nums: &mut Vec<usize>) {

    fn _insert_sort(nums: &mut Vec<usize>, g: usize) {
        for i in g..nums.len() {
            let (mut p, v) = (i, nums[i]);
            while p >= g && nums[p-g] > v {
                nums[p] = nums[p-g];
                p -= g;
            }
            nums[p] = v;
        }
    }

    let mut a: VecDeque<usize> = VecDeque::new();
    a.push_front(1);
    while *a.front().unwrap() <= nums.len() {
        a.push_front(3*a.front().unwrap()+1);
    }
    for &g in a.iter() {
        _insert_sort(nums, g);
    }
}