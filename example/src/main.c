#include <stdio.h>
#include <test.h>
#include <seastar_pkg/seastar_lib.h>

int main() {
    printf("Hello world!\n");
    test();
    test_cpp();
    seastar_library_test();

    return 0;
}