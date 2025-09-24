#include <cassert>
#include <cmath>
#include "raylib.h"

namespace app {

double const SIN_PI_DIV_3{std::sin(PI / 3)};
double const SIN_PI_DIV_6{std::sin(PI / 6)};

template <typename T>
T mod(T modulus, T a) {
    T result = a % modulus;
    result = result < 0 ? result + modulus : result;
    return result;
}

template <typename T>
T modularAdditionSafe(T modulus, T a, T b) {
    T a_mod = mod(modulus, a);
    T b_mod = mod(modulus, b);
    b_mod = b_mod < 0 ? b_mod + modulus : b_mod;

    return mod(modulus, a_mod + b_mod);
}

/**
 * Slightly more efficient implementation, but may sacrifice floating point precision far from zero
 * or accuracy when a + b results in an overflow, but mod(a) + mod(b) would not.
 */
template <typename T>
T modularAddition(T modulus, T a, T b) {
    return mod(modulus, a + b);
}

inline void test_mod() {
    assert(mod(1, 1) == 0);
    assert(mod(1, 0) == 0);
    assert(mod(1, -1) == 0);
    assert(mod(2, -1) == 1);
    assert(mod(2, -3) == 1);
    assert(mod(3, -1) == 2);
    assert(mod(3, 5) == 2);
}

inline void test_modularAddition() {
    assert(modularAdditionSafe(2, 0, 0) == 0);
    assert(modularAdditionSafe(2, 0, 1) == 1);
    assert(modularAdditionSafe(2, 1, 0) == 1);
    assert(modularAdditionSafe(2, 1, 1) == 0);
    assert(modularAdditionSafe(3, 0, 1) == 1);
    assert(modularAdditionSafe(3, 1, 1) == 2);
    assert(modularAdditionSafe(3, 1, 3) == 1);
    assert(modularAdditionSafe(3, 1, -2) == 2);
    assert(modularAdditionSafe(3, 1, -6) == 1);
}

inline void test_modularAdditionFast() {
    assert(modularAddition(2, 0, 0) == 0);
    assert(modularAddition(2, 0, 1) == 1);
    assert(modularAddition(2, 1, 0) == 1);
    assert(modularAddition(2, 1, 1) == 0);
    assert(modularAddition(3, 0, 1) == 1);
    assert(modularAddition(3, 1, 1) == 2);
    assert(modularAddition(3, 1, 3) == 1);
    assert(modularAddition(3, 1, -2) == 2);
    assert(modularAddition(3, 1, -6) == 1);
}

inline void test_util() {
    test_mod();
    test_modularAddition();
    test_modularAdditionFast();
}

} // namespace app
