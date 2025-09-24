#include <cstdio>
#include <raylib.h>

#include "engine.h"
#include "app.h"
#include "result.h"
#include "map.h"

namespace app {

static char fps_str[5];
static Font default_font;
static Font custom_font;

result_t init() {
    SetTraceLogLevel(LOG_DEBUG);
    SetTargetFPS(app::TARGET_FPS);

    SetConfigFlags(FLAG_WINDOW_HIGHDPI); // FLAG_WINDOW_HIGHDPI needed for MacOS resolution adjustment
    InitWindow(DISPLAY_WIDTH, DISPLAY_HEIGHT, application_name.c_str());

    // todo: SetWindowIcon

    default_font = GetFontDefault();
    custom_font = LoadFont("/Users/zacharysiegel/Downloads/Google_Sans_Code/static/GoogleSansCode-Regular.ttf");

    if (!IsWindowReady()) {
        return ERROR;
    }

    return OK;
}

result_t destroy() {
    CloseWindow();
    return OK;
}

static void draw();
static void update();

result_t run() {
    while (!WindowShouldClose()) {
        update();

        BeginDrawing();
        draw();
        EndDrawing();
    }
    return OK;
}

static void update() {
    int fps = GetFPS();
    std::snprintf(fps_str, 5, "%d", fps);

    if (IsKeyPressed(KEY_A)) {
        TraceLog(LOG_DEBUG, "a pressed");
    }
}

static void draw() {
    ClearBackground(Color{.r = 30, .g = 30, .b = 30, .a = 0xFF});

    // Debug
    DrawText(fps_str, 10, 10, 20.0f, YELLOW);

    // todo: draw background

    drawMap();
}

} // namespace app
