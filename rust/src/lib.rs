pub mod bitwise_stuff;
pub mod more_constants;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// (a * a) + (b * b)
#[unsafe(no_mangle)]
pub extern "C" fn sum_squares(a: i32, b: i32) -> i32 {
    a * a + b * b
}

#[unsafe(no_mangle)]
pub extern "C" fn sum_array(array: *const i32, len: usize) -> i32 {
    let mut acc = 0;
    for i in 0..len {
        unsafe {
            acc += *(array.add(i));
        }
    }
    acc
}

pub const WOW: u32 = 69;

const FIB_LEN: usize = {
    let mut a = 0;
    let mut b = 1;
    let mut len = 1;
    while let Some(c) = u32::checked_add(a, b) {
        a = b;
        b = c;
        len += 1;
    }
    len
};

const _FIB_TABLE: [u32; FIB_LEN] = {
    let mut arr = [0; FIB_LEN];
    arr[1] = 1;
    let mut idx = 2;
    while idx < arr.len() {
        arr[idx] = arr[idx - 2] + arr[idx - 1];
        idx += 1;
    }
    arr
};

pub const FIB_TABLE: [u32; FIB_LEN] = _FIB_TABLE;

/// Reads a lookup table for the values between 0 and 46.
/// Going over 46 returns zero.
#[unsafe(no_mangle)]
pub const extern "C" fn fib_get(n: usize) -> u32 {
    if n > FIB_TABLE.len() - 1 {
        return 0;
    }
    FIB_TABLE[n]
}

// pub const FACTORIAL_LEN: usize = {
//     let mut n = 2;
//     let mut acc = 1;
//     while let Some(product) = u32::checked_mul(n, acc) {
//         acc = product;
//         n += 1;
//     }
//     n as usize
// };
//
// pub const FACTORIAL_TABLE: [u64; FACTORIAL_LEN] = {
//     let mut arr = [1u64; FACTORIAL_LEN];
//     let mut idx = 1;
//     while idx < arr.len() {
//         arr[idx] = arr[idx - 1] * idx as u64;
//         idx += 1;
//     }
//     arr
// };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
