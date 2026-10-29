#pragma once

#define sqrtf(f) __builtin_sqrtf(f)
#define fabsf(f) __builtin_fabsf(f)
#define floorf(f) __builtin_floorf(f)
#define ceilf(f) __builtin_ceilf(f)

// No wasm instruction for these: the Rust side links them in.
#ifdef __cplusplus
extern "C" {
#endif
float log2f(float x);
float frexpf(float x, int* exp);
float ldexpf(float x, int exp);
float cosf(float x);
#ifdef __cplusplus
}
#endif
