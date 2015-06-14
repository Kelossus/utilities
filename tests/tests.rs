extern crate util;
use util::*;


#[test]
fn read_line_with_timeout_works() {
    println!("{:?}",read_line_with_timeout(1000) );
}  


#[test]
#[should_panic]
fn KMP_shifts_works() {
    let a = KMP_shifts("aaabaaooaafsjmskjaakka a".to_string());
    let b = vec!(1, 1, 1, 1, 4, 4, 4, 7, 8, 8, 8, 11, 12, 13, 14, 15, 16, 17, 17, 17, 20, 21, 21, 23, 23);
    assert_eq!(a,b);
}
