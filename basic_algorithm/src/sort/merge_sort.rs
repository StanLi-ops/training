pub fn merge_sort(nums: &mut Vec<usize>) {

    fn _merge(nums: &mut Vec<usize>, left: usize, mid: usize, right: usize) {
        let left_part: Vec<usize> = nums[left..mid].iter().cloned().collect();
        let right_part: Vec<usize> = nums[mid..right].iter().cloned().collect();
        let (mut left_offset, mut right_offset) = (0usize, 0usize);
        while left_offset < left_part.len() || right_offset < right_part.len() {
            if right_offset == right_part.len()
            || (left_offset < left_part.len() && left_part[left_offset] <= right_part[right_offset]) {
                nums[left + left_offset + right_offset] = left_part[left_offset];
                left_offset += 1;
            } else {
                nums[left + left_offset + right_offset] = right_part[right_offset];
                right_offset += 1;
            }
        }
    }

    fn _merge_sort(nums: &mut Vec<usize>, left: usize, right: usize) {
        if left+1 < right {
            let mid = (left + right) / 2;
            _merge_sort(nums, left, mid);
            _merge_sort(nums, mid, right);
            _merge(nums, left, mid, right);
        }
    }

    _merge_sort(nums, 0, nums.len())
}