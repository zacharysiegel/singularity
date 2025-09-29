#include <format>
#include <string>

#include "raylib.h"
#include "raymath.h"

#include "config.h"
#include "engine.h"
#include "map.h"
#include "result.h"

namespace app {

static MapCoord map_origin{0, 0};

static MapCoord overflowAdjustedMapCoord(MapCoord map_coord) {
    float mapWidthPixels = getMapWidthPixels();
    float mapHeightPixels = getMapHeightPixels();

    while (map_coord.x < 0) {
        map_coord.x += mapWidthPixels;
    }
    while (map_coord.y < 0) {
        map_coord.y += mapHeightPixels;
    }

    while (map_coord.x >= mapWidthPixels) {
        map_coord.x -= mapWidthPixels;
    }
    while (map_coord.y >= mapHeightPixels) {
        map_coord.y -= mapHeightPixels;
    }

    return map_coord;
}

static MapCoord computeScrolledMapOrigin(MapCoord map_origin) {
    MapCoord scroll = Vector2Multiply(GetMouseWheelMoveV(), {.x = -1, .y = -1});
    MapCoord raw_updated_origin = Vector2Add(map_origin, scroll);
    return overflowAdjustedMapCoord(raw_updated_origin);
}

static void update() {
    if (IsKeyPressed(KEY_A)) {
        TraceLog(LOG_DEBUG, "a pressed"); // todo: delete
    }

    MapCoord old{map_origin};
    map_origin = computeScrolledMapOrigin(map_origin);
    if (old.x != map_origin.x || old.y != map_origin.y) {
        TraceLog(LOG_DEBUG, std::format("({}, {})", map_origin.x, map_origin.y).c_str());
    }
}

static void draw() {
    ClearBackground(BACKGROUND_COLOR);
    // todo: draw background

    drawMap(map_origin);

    { // Debug
        DrawFPS(10, 10);
    }
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

    BeginDrawing();
    ClearBackground(BACKGROUND_COLOR);
    DrawText("Loading...", 16, GetScreenHeight() - 30, 20, RAYWHITE);
    EndDrawing();

    initMap();

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
