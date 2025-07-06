/// Prints a hello world with std::println!()
#[unsafe(no_mangle)]
pub extern "C" fn hello_rust() {
    println!("Hello from the Rust lib!");
}

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
/// for i in 0..len {
///     output[i] = lhs[i] + rhs[i];
/// }
#[unsafe(no_mangle)]
pub const extern "C" fn add_arrays(lhs: *const i32, rhs: *const i32, output: *mut i32, len: usize) {
    let mut i = 0;
    while i < len {
        unsafe { *output.add(i) = *lhs.add(i) + *rhs.add(i) }
        i += 1;
    }
}

/// for i in 0..len {
///     output[i] = func(lhs[i], rhs[i]);
/// }
#[unsafe(no_mangle)]
pub extern "C" fn map_arrays(
    lhs: *const i32,
    rhs: *const i32,
    output: *mut i32,
    len: usize,
    func: extern "C" fn(i32, i32) -> i32,
) {
    let mut i = 0;
    while i < len {
        unsafe { *output.add(i) = func(*lhs.add(i), *rhs.add(i)) }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{FIB_TABLE, add_arrays, fib_get, fib_len, map_arrays};

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

    #[test]
    fn add_arrays_test() {
        let lhs = [0, 1, 2, 3, 4];
        let rhs = [4, 3, 2, 1, 0];
        let mut output = [0i32; 5];
        add_arrays(lhs.as_ptr(), rhs.as_ptr(), output.as_mut_ptr(), 5);
        assert!(output == [4; 5]);
    }
    #[test]
    fn map_arrays_test() {
        let lhs = [2, 4, 6, 8, 10];
        let rhs = [3, 6, 9, 12, 15];
        let mut output = [0; 5];

        extern "C" fn mul_nums(a: i32, b: i32) -> i32 {
            a * b
        }

        map_arrays(lhs.as_ptr(), rhs.as_ptr(), output.as_mut_ptr(), 5, mul_nums);
        let _: () = assert!(output == [6, 24, 54, 96, 150]);
    }
}
