#pragma once

#include <string>

#include "raylib.h"

// todo: rename to config.h
namespace app {

std::string const APPLICATION_NAME{"silicogenesis"};
uint16_t const HEX_COUNT_SQRT{64};
uint16_t const HEX_COUNT{HEX_COUNT_SQRT * HEX_COUNT_SQRT};
Color const BACKGROUND_COLOR{.r = 30, .g = 30, .b = 30, .a = 0xff};
Color const HEX_OUTLINE_COLOR{.r=0xff, .g=0xff, .b=0xff, .a=0x80};
Color const METAL_BACKGROUND_COLOR{.r = 0x60, .g = 0x50, .b = 0x70, .a = 0xff};
Color const OIL_BACKGROUND_COLOR{.r = 0x58, .g = 0x58, .b = 0x58, .a = 0xff};

} // namespace app
