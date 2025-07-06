#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * An array of uint32_t fibonacci values
 * going from f(0) to f(46)
 */
extern const uint32_t FIB_TABLE[47];

/**
 * Prints a hello world with std::println!()
 */
void hello_rust(void);

/**
 * Reads a lookup table for the values
 * between 0 and 46. Going over 46 returns zero.
 */
uint32_t fib_get(uint8_t n);
