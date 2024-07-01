use crate::mod_absolute::mod_abs;

#[test]
fn test_mod_absolute() {
    let ma_result = mod_abs(7, 5);
    let rem_result = 7 % 5;
    assert_eq!(ma_result, rem_result);
}
