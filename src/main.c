#include "../include/rust.h"
#include <stdio.h>
int main(int argc, char *argv[]) {
  printf("In main\n");
  int idx = 7;
  int fib_num = fib_get(7);
  printf("fib_num(%d) -> %d", idx, fib_num);
  return 0;
}
