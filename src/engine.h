#pragma once

#include "result.h"

namespace app {

uint8_t const TARGET_FPS = 60;
uint16_t const DISPLAY_WIDTH = 1600;
uint16_t const DISPLAY_HEIGHT = 900;

result_t init();
result_t run();
result_t destroy();

} // namespace app
