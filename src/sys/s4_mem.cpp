#include <cstdio>

struct S1 {
    u_int8_t a;
    u_int16_t b;
    u_int8_t c;
};

struct S2 {
    u_int8_t a;
    u_int8_t c;
    u_int16_t b;
};

int main() {
    printf("size of S1: %lu, S2: %lu", sizeof(struct S1), sizeof(struct S2));
    return 0;
}
