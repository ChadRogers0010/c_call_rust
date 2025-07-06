const fn get_fib_len() -> usize {
    let mut a = 0;
    let mut b = 1;
    let mut len = 1;
    while let Some(c) = u32::checked_add(a, b) {
        a = b;
        b = c;
        len += 1;
    }
    len
}
pub const FIB_LEN: usize = get_fib_len();

const fn make_fib_table() -> [u32; FIB_LEN] {
    let mut arr = [0; FIB_LEN];
    arr[1] = 1;
    let mut idx = 2;
    while idx < arr.len() {
        arr[idx] = arr[idx - 2] + arr[idx - 1];
        idx += 1;
    }
    arr
}
pub const FIB_TABLE: [u32; FIB_LEN] = make_fib_table();

/// Reads a lookup table for the values between 0 and 46.
/// Going over 46 returns zero.
#[unsafe(no_mangle)]
pub const extern "C" fn fib_get(n: u8) -> u32 {
    let n = n as usize;
    if n > FIB_TABLE.len() - 1 {
        return 0;
    }
    FIB_TABLE[n]
}

#[unsafe(no_mangle)]
pub const extern "C" fn double_num(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use crate::fib_get;

    #[test]
    fn fib_get_test() {
        let fib_num = fib_get(7);
        assert!(fib_num == 13);
    }
}
