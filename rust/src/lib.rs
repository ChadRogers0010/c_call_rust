/// For a u32 the of fibonacci elements is 47
const fn fib_len() -> usize {
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

const fn make_fib_table() -> [u32; fib_len()] {
    let mut arr = [0; fib_len()];
    arr[1] = 1;
    let mut idx = 2;
    while idx < arr.len() {
        arr[idx] = arr[idx - 2] + arr[idx - 1];
        idx += 1;
    }
    arr
}

/// An array of uint32_t fibonacci values
/// going from f(0) to f(46)
#[unsafe(no_mangle)]
pub static FIB_TABLE: [u32; 47] = make_fib_table();

/// Reads a lookup table for the values
/// between 0 and 46. Going over 46 returns zero.
#[unsafe(no_mangle)]
pub const extern "C" fn fib_get(n: u8) -> u32 {
    let n = n as usize;
    if n > FIB_TABLE.len() - 1 {
        return 0;
    }
    FIB_TABLE[n]
}

#[cfg(test)]
mod tests {
    use crate::{FIB_TABLE, fib_get, fib_len};

    #[test]
    fn fib_get_test() {
        let fib_num = fib_get(7);
        assert!(fib_num == 13);

        let fib_num = fib_get(47);
        assert!(fib_num == 0);
    }

    #[test]
    fn fib_table_test() {
        assert!(FIB_TABLE[7] == 13);

        assert!(FIB_TABLE.get(fib_len()).is_none());
    }
}
