#include <raylib.h>

#include "app.h"
#include "engine.h"
#include "map.h"
#include "result.h"

namespace app {

static void update() {
    if (IsKeyPressed(KEY_A)) {
        TraceLog(LOG_DEBUG, "a pressed");
    }
}

static void draw() {
    ClearBackground(Color{.r = 30, .g = 30, .b = 30, .a = 0xFF});
    // todo: draw background

    drawMap(Vector2{1000, 500});

    // Debug
    DrawFPS(10, 10);
}

result_t init() {
    SetTraceLogLevel(LOG_DEBUG);
    SetTargetFPS(app::TARGET_FPS);

    SetConfigFlags(FLAG_WINDOW_HIGHDPI | FLAG_WINDOW_RESIZABLE); // FLAG_WINDOW_HIGHDPI needed for MacOS resolution adjustment
    InitWindow(DISPLAY_WIDTH, DISPLAY_HEIGHT, APPLICATION_NAME.c_str());

    // todo: SetWindowIcon

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
        update();

        BeginDrawing();
        draw();
        EndDrawing();
    }
    return OK;
}

} // namespace app
