#include "../include/other.h"
#include "../include/rust.h"
#include <stdint.h>
#include <stdio.h>
int main(int argc, char *argv[]) {
  printf("Hello from the C main()!\n");
  hello_rust();

  printf("fib_get(%d) -> %d\n", 7, fib_get(7));
  for (int i = 0; i < 10; i++) {
    printf("fib %d: %d\n", i, FIB_TABLE[i]);
  }

  other_test();
  return 0;
}
