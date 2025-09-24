#include "raylib.h"

#include "map.h"
#include "engine.h"

namespace app {

void drawMap() {
    DrawPolyLinesEx(Vector2{.x = DISPLAY_WIDTH / 2.0, .y = DISPLAY_HEIGHT / 2.0}, 6, 10.0f, 0.0f, 2.0f, RAYWHITE);
}

}

