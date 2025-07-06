/// Prints a hello world with std::println!()
#[unsafe(no_mangle)]
pub extern "C" fn hello_rust() {
    println!("Hello from the Rust lib!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn tests_are_automated_0() {
        assert!(true)
    }
    #[test]
    fn tests_are_automated_1() {
        assert!(true)
    }
    #[test]
    fn tests_are_automated_2() {
        assert!(true)
    }
}
