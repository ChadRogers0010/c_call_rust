#include "../include/rust.h"
#include <stdint.h>
#include <stdio.h>
int main(int argc, char *argv[]) {
  printf("Hello from the C main()!\n");
  hello_rust();

  return 0;
}
