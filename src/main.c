#include "../include/rust.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
  printf("Running main.c\n");

  int foo = 3, bar = 4;
  int baz = sum_squares(foo, bar);
  printf("sum_squares(%d, %d) == %d\n", foo, bar, baz);

  int arr[] = {1, 2, 3, 4};
  int arr_sum = sum_array(arr, sizeof(arr) / sizeof(uint32_t));
  printf("[1, 2, 3, 4] sum == %d\n", arr_sum);

  printf("Funny number %d\n", WOW);

  for (int i = 0; i < 11; i++) {
    printf("%d\n", fib_get(i));
  }

  int quox = 0b110101;
  int pop = popcnt_while(quox);
  printf("quox: %d\n", pop);

  return 0;
}
