#include <iostream>
extern "C" {
void hello_cpp(void) { std::cout << "Hello from C++!" << std::endl; }
}
