///
///
/// # Examples
/// ```
/// # use chad_math::bitwise::popcnt_while;
/// assert!(popcnt_while(0b1101) == 3);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn popcnt_while(mut data: u32) -> u32 {
    let mut count = 0;
    while data > 0 {
        data &= data - 1;
        count += 1;
    }
    count
}
