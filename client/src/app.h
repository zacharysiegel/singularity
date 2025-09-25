#pragma once

#include <string>

#include "raylib.h"

namespace app {

std::string const APPLICATION_NAME{"silicogenesis"};
uint16_t const HEX_COUNT_SQRT{64};
uint16_t const HEX_COUNT{HEX_COUNT_SQRT * HEX_COUNT_SQRT};
Color const BACKGROUND_COLOR{.r = 30, .g = 30, .b = 30, .a = 0xFF};

} // namespace app
