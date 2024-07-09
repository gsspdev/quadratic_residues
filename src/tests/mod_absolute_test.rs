#[cfg(test)]
use crate::pos_mod::abs_mod;

#[test]
pub fn mod_absolute_test_neg_x() {
    assert_eq!(abs_mod(-7, 5), 2);
}
