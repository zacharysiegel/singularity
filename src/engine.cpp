#include <cstdint>
#include <raylib.h>

#include "result.h"

static uint8_t const TARGET_FPS = 60;

static void draw();

result_t init() {
    InitWindow(800, 600, "__untitled__"); // todo: title
    SetTargetFPS(TARGET_FPS);

    if (!IsWindowReady()) {
        return ERROR;
    }

    return OK;
}

result_t destroy() {
    CloseWindow();
    return OK;
}

result_t run() {
    while (!WindowShouldClose()) {
        BeginDrawing();
        draw();
        EndDrawing();
    }
    return OK;
}

static void draw() {
    ClearBackground(Color{.r = 30, .g = 30, .b = 30, .a = 0xFF});
}
