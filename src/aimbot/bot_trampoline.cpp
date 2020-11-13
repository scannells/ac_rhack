#include <stdio.h>
#include <stdint.h>

// this CPP function serves as a trampoline to call BotIsVisible() of AC
// IsVisible() has a weird calling convention Rust can't deal with.
// We could have used Rust inline ASM but that would require the nightly
// rust build at the time of writing, so we just use this CPP trampoline.
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