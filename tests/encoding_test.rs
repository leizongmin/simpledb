use cedar::encoding::compare_score_bytes;

#[test]
fn test_compare_score_bytes() {
    assert_eq!(1, compare_score_bytes("aab".as_bytes(), "aaa".as_bytes()));
    assert_eq!(-1, compare_score_bytes("aaa".as_bytes(), "aab".as_bytes()));
    assert_eq!(0, compare_score_bytes("aaa".as_bytes(), "aaa".as_bytes()));
    assert_eq!(-1, compare_score_bytes("aaa".as_bytes(), "aaab".as_bytes()));
}
