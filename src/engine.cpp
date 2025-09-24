#include <cstdint>
#include <raylib.h>

#include "app.h"
#include "result.h"

namespace app {

static uint8_t const TARGET_FPS = 60;
static uint16_t const DISPLAY_WIDTH = 1600;
static uint16_t const DISPLAY_HEIGHT = 900;

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
    if (IsKeyPressed(KEY_A)) {
        TraceLog(LOG_DEBUG, "a pressed");
    }
}

static void draw() {
    ClearBackground(Color{.r = 30, .g = 30, .b = 30, .a = 0xFF});

    // Text drawing tests
    DrawText("test text 1", 10, 10, 20, Color{0xff, 0xff, 0x80, 0xff});
    DrawTextEx(app::default_font, "test text 2", Vector2{.x = 10, .y = 30}, 20.0f, 10.0f, Color{0xff, 0xff, 0x80, 0xff});
    DrawTextEx(app::custom_font, "test text 3", Vector2{.x = 10, .y = 50}, 20.0f, 10.0f, Color{0xff, 0xff, 0x80, 0xff});
    DrawTextPro(app::default_font, "test text 4", Vector2{0, 70}, Vector2{-10, 0}, 0.0f, 20.0f, 10.0f, Color{0xff, 0xff, 0x80, 0xff});
    DrawTextPro(app::custom_font, "test text 5", Vector2{0, 90}, Vector2{-10, 0}, 0.0f, 20.0f, 10.0f, Color{0xff, 0xff, 0x80, 0xff});

    // todo: draw background

    // todo: draw map
    DrawPolyLinesEx(Vector2{.x = DISPLAY_WIDTH / 2.0, .y = DISPLAY_HEIGHT / 2.0}, 6, 10.0f, 0.0f, 2.0f, RAYWHITE);
}

} // namespace app
