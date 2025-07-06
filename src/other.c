#include "../include/rust.h"
#include <stdio.h>

int32_t func(int32_t a, int32_t b) { return a * b; }

void other_test(void) {
  int lhs[5] = {0, 1, 2, 3, 4};
  int rhs[5] = {4, 3, 2, 1, 0};
  int output[5] = {0};
  add_arrays(lhs, rhs, output, sizeof(output) / sizeof(int32_t));
  for (int i = 0; i < 5; i++) {
    printf("%d ", output[i]);
  }
  printf("\n");

  int foo[5] = {2, 4, 6, 8, 10};
  int bar[5] = {3, 6, 9, 12, 15};
  int baz[5] = {0};
  map_arrays(foo, bar, baz, sizeof(foo) / sizeof(int32_t), func);

  for (int i = 0; i < 5; i++) {
    printf("%d ", baz[i]);
  }
  printf("\n");
}
