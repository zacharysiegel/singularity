#include <format>
#include <string>
#include <chrono>

#include "raylib.h"
#include "raymath.h"

#include "config.h"
#include "engine.h"
#include "map.h"
#include "result.h"
#include "player.h"
#include "state.h"

namespace app {

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

static MapCoord computeScrolledMapOrigin(MapCoord const map_origin) {
    MapCoord scroll = Vector2Multiply(GetMouseWheelMoveV(), {.x = -1, .y = -1});
    MapCoord raw_updated_origin = Vector2Add(map_origin, scroll);
    return overflowAdjustedMapCoord(raw_updated_origin);
}

static void update() {
    if (IsKeyPressed(KEY_A)) {
        TraceLog(LOG_DEBUG, "a pressed"); // todo: delete
    }

    MapCoord old{state.map_origin};
    state.map_origin = computeScrolledMapOrigin(state.map_origin);
    if (old.x != state.map_origin.x || old.y != state.map_origin.y) {
        TraceLog(LOG_DEBUG, std::format("({}, {})", state.map_origin.x, state.map_origin.y).c_str());
    }
}

static void draw() {
    ClearBackground(BACKGROUND_COLOR);
    // todo: draw background

    drawMap(state.map_origin);
    drawPlayers(state.map_origin);

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
    initPlayers(4);

    return OK;
}

result_t destroy() {
    CloseWindow();
    return OK;
}

result_t run() {
    while (!WindowShouldClose()) {
        std::chrono::high_resolution_clock clock = std::chrono::high_resolution_clock{};
        std::chrono::time_point frame_start = clock.now();

        update();
        std::chrono::time_point update_end = clock.now();

        BeginDrawing();
        draw();
        std::chrono::time_point draw_end = clock.now();
        EndDrawing();

        state.frame_counter += 1;
        if (state.frame_counter % 1000 == 0) {
            TraceLog(LOG_DEBUG, std::format("Frame: {}; Update: {}; Draw: {}; Total: {};", 
                     state.frame_counter,
                     update_end - frame_start,
                     draw_end - update_end,
                     draw_end - frame_start).c_str());
        }
    }
    return OK;
}

} // namespace app
