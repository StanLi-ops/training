pub fn quick_sort(nums: &mut Vec<usize>) {

    fn _partition(nums: &mut Vec<usize>, begin: usize, end: usize) -> usize {
        let (mut i, v) = (begin, nums[end-1]);
        for j in begin..end-1 {
            if nums[j] <= v {
                nums.swap(i, j);
                i += 1;
            }
        }
        nums.swap(i, end-1);
        i
    }

    fn _quick_sort(nums: &mut Vec<usize>, begin: usize, end: usize) {
        if begin+1 < end {
            let mid = _partition(nums, begin, end);
            _quick_sort(nums, begin, mid);
            _quick_sort(nums, mid+1, end);
        }
    }

    _quick_sort(nums, 0, nums.len())
}