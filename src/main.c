#include "../include/other.h"
#include "../include/rust.h"
#include <stdint.h>
#include <stdio.h>
int main(int argc, char *argv[]) {
  printf("In main\n");
  int a = 5;
  int b = 6;
  int sum = sum_squares(a, b);
  printf("sum_squares(%d, %d) -> %d\n", a, b, sum);
  sum = double_sum_squares(a, b);
  printf("double_sum_squares(%d, %d) -> %d\n", a, b, sum);
  return 0;
}
