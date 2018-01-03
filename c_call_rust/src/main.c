#include <stdint.h>
#include <stdio.h>

extern int32_t add_one(int32_t x);

int main() {
    int x = 12;
    int y = add_one(x);
    printf("x = %d, y = %d\n", x, y);
    return 0;
}
