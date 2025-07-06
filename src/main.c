#include "../include/rust.h"
#include <stdio.h>
int main(int argc, char *argv[]) {
  printf("In main\n");
  int a = 5;
  int b = 6;
  int sum = sum_squares(a, b);
  printf("sum_squares(%d, %d) -> %d", a, b, sum);
  // printf("Fib_Table[5] = %d", FIB_TABLE[5]);
  return 0;
}
