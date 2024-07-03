#[cfg(test)]
use crate::pos_mod::abs_mod;

#[test]
pub fn mod_absolute_test_neg_x() {
    assert_eq!(abs_mod(-7, 5), 2);
}

// pub fn mod_absolute_tests() {
//     let ma_result = abs_mod(7, 5);
//     let rem_result = 7 % 5;
//     assert_eq!(ma_result, rem_result);
// }
