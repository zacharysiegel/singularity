#pragma once

#include <cstdint>
#include <string>

#include "raylib.h"

#include "util.h"

namespace app {

std::string const APPLICATION_NAME{"silicogenesis"};
Color const BACKGROUND_COLOR{.r = 30, .g = 30, .b = 30, .a = 0xff};
Color const HEX_OUTLINE_COLOR{.r = 0xff, .g = 0xff, .b = 0xff, .a = 0x80};
Color const METAL_BACKGROUND_COLOR{.r = 0x60, .g = 0x50, .b = 0x70, .a = 0xff};
Color const OIL_BACKGROUND_COLOR{.r = 0x58, .g = 0x58, .b = 0x58, .a = 0xff};
uint16_t const HEX_COUNT_SQRT{64};
uint16_t const HEX_COUNT{HEX_COUNT_SQRT * HEX_COUNT_SQRT};
uint8_t const HEX_SIDES{6};
uint8_t const HEX_RADIUS{32};
double const HEX_SIDE_LENGTH{2 * SIN_PI_DIV_6 * HEX_RADIUS};
double const HEX_HEIGHT{SIN_PI_DIV_3 * HEX_RADIUS * 2};
float const HEX_ROTATION{30.0f};

inline uint16_t constexpr getHexCountWidth(float pixels) {
    return static_cast<uint16_t>(std::ceilf(pixels / HEX_HEIGHT));
}

inline uint16_t constexpr getHexCountHeight(float pixels) {
    return static_cast<uint16_t>(std::ceilf(pixels / (HEX_RADIUS * 2)));
}

inline float constexpr getHexWidthPixels(uint16_t hex_count) {
    return hex_count * (HEX_HEIGHT);
}

inline float constexpr getHexHeightPixels(uint16_t hex_count) {
    return hex_count * (HEX_RADIUS * 2);
}

inline float constexpr getMapWidthPixels() {
    return getHexWidthPixels(HEX_COUNT_SQRT);
}

inline float constexpr getMapHeightPixels() {
    return getHexHeightPixels(HEX_COUNT_SQRT);
}

} // namespace app
