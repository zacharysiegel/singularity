#include <cstdint>
#include <raylib.h>

#include "app.h"
#include "result.h"

namespace app {

static uint8_t const TARGET_FPS = 60;

static Font default_font;
static Font custom_font;

static void draw();

result_t init() {
    SetTraceLogLevel(LOG_DEBUG);
    SetTargetFPS(app::TARGET_FPS);

    InitWindow(800, 600, application_name.c_str());
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

result_t run() {
    while (!WindowShouldClose()) {
        if (IsKeyPressed(KEY_A)) {
            TraceLog(LOG_DEBUG, "a pressed");
        }

        BeginDrawing();
        draw();
        EndDrawing();
    }
    return OK;
}

static void draw() {
    ClearBackground(Color{.r = 30, .g = 30, .b = 30, .a = 0xFF});

    // Text drawing is not working (even copying the Raylib default font example fails)
    DrawText("test text 1", 10, 10, 20, Color{0xff, 0xff, 0x80, 0xff});
    DrawTextEx(app::default_font, "test text 2", Vector2{.x = 10, .y = 10}, 20.0f, 10.0f, Color{0xff, 0xff, 0x80, 0xff});
    DrawTextEx(app::custom_font, "test text 3", Vector2{.x = 10, .y = 10}, 20.0f, 10.0f, Color{0xff, 0xff, 0x80, 0xff});
    DrawTextPro(app::default_font, "test text 5", Vector2{10, 10}, Vector2{50, 50}, 0.0f, 20.0f, 10.0f, Color{0xff, 0xff, 0x80, 0xff});
    DrawTextPro(app::custom_font, "test text 5", Vector2{10, 10}, Vector2{50, 50}, 0.0f, 20.0f, 10.0f, Color{0xff, 0xff, 0x80, 0xff});

    // todo: draw background

    // todo: draw map
}

} // namespace app
