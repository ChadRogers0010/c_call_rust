#include "../include/rust.h"
#include <stdio.h>
int main(int argc, char *argv[]) {
  printf("In main\n");
  int idx = 7;
  int fib_num = fib_get(7);
  printf("fib_num(%d) -> %d\n", idx, fib_num);
  for (int i = 0; i < 10; i++) {
    printf("fib %d: %d\n", i, FIB_TABLE[i]);
  }
  return 0;
}
