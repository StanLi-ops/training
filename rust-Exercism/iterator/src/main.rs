fn main() {
    let vl = vec![1,2,3];
    let mut vl_iter = vl.iter();
    assert_eq!(vl_iter.next(), Some(&1));
    assert_eq!(vl_iter.next(), Some(&2));
    assert_eq!(vl_iter.next(), Some(&3));
    assert_eq!(vl_iter.next(), None);
}
