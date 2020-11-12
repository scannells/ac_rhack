#include <stdio.h>
#include <stdint.h>

// call into the function, correctly.
// this is the struct vec definition as in the AssaultCube code
struct vec
{
    union
    {
        struct { float x, y, z; };
        float v[3];
        int i[3];
    };
};

typedef uint8_t (*IsVisible_t)(vec from, vec to, void *tracer, bool skipTags);

extern "C" char bot_isvisible(unsigned long isVisibleAddr, vec *from, vec *to) {
    vec a = *from;
    vec b = *to;
    IsVisible_t hook = (IsVisible_t)isVisibleAddr;
    return hook(a, b, NULL, false);
}