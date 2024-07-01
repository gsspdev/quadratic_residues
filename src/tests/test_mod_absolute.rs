use crate::pos_mod::abs_mod;

#[test]
fn test_mod_absolute() {
    let ma_result = abs_mod(7, 5);
    let rem_result = 7 % 5;
    assert_eq!(ma_result, rem_result);
}
