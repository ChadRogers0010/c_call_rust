#include "../include/hello_c.h"
#include "../include/hello_cpp.h"
#include "../include/rust.h"
#include <stdint.h>
int main(int argc, char *argv[]) {
  hello_c();
  hello_cpp();
  hello_rust();

  return 0;
}
