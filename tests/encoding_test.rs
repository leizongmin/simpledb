use simpledb::codec::{compare_score_bytes, get_next_upper_bound};

#[test]
fn test_compare_score_bytes() {
    assert_eq!(1, compare_score_bytes("aab".as_bytes(), "aaa".as_bytes()));
    assert_eq!(-1, compare_score_bytes("aaa".as_bytes(), "aab".as_bytes()));
    assert_eq!(0, compare_score_bytes("aaa".as_bytes(), "aaa".as_bytes()));
    assert_eq!(-1, compare_score_bytes("aaa".as_bytes(), "aaab".as_bytes()));
}

#[test]
fn test_get_next_upper_bound() {
    assert_eq!(
        vec![0, 0, 0, 1],
        get_next_upper_bound(vec![0, 0, 0, 0].as_slice())
    );
    assert_eq!(
        vec![1, 2, 3, 5],
        get_next_upper_bound(vec![1, 2, 3, 4].as_slice())
    );
    assert_eq!(
        vec![1, 2, 4, 0],
        get_next_upper_bound(vec![1, 2, 3, 255].as_slice())
    );
    assert_eq!(
        vec![2, 0, 0, 0],
        get_next_upper_bound(vec![1, 255, 255, 255].as_slice())
    );
    assert_eq!(
        vec![255, 255, 255, 255, 0],
        get_next_upper_bound(vec![255, 255, 255, 255].as_slice())
    );
}
