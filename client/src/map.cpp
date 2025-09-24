#include <cassert>
#include <cstdint>
#include <sys/cdefs.h>

#include "raylib.h"
#include "raymath.h"

#include "map.h"
#include "util.h"

namespace app {

int32_t const MAP_WIDTH{4000};
int32_t const MAP_HEIGHT{MAP_WIDTH};
uint8_t const HEX_SIDES{6};
uint8_t const HEX_RADIUS{32};
double const HEX_SIDE_LENGTH{2 * SIN_PI_DIV_6 * HEX_RADIUS};
double const HEX_HEIGHT{SIN_PI_DIV_3 * HEX_RADIUS * 2};

Vector2 renderCoordFromMapCoord(Vector2 render_origin, Vector2 map_coord) {
    return Vector2{
        .x = map_coord.x - render_origin.x,
        .y = map_coord.y - render_origin.y,
    };
}

void drawMapHex(Vector2 center) {
    // { // Colored hex fill for debugging
    //     static bool color_switch{true};
    //     color_switch = !color_switch;
    //     DrawPoly(center, HEX_SIDES, HEX_RADIUS, 30.0f, color_switch ? RED : BLUE);
    // }

    DrawPolyLinesEx(center, HEX_SIDES, HEX_RADIUS, 30.0f, 1.0f, RAYWHITE);
}

void drawMap(Vector2 render_origin) {
    int32_t screen_width{GetScreenWidth()}; // is it any faster to call this function only once per frame? or does raylib already include this caching optimization?
    int32_t screen_height{GetScreenHeight()};
    int32_t map_render_max_x{modularAddition<int32_t>(MAP_WIDTH, render_origin.x, screen_width)};
    int32_t map_render_max_y{modularAddition<int32_t>(MAP_HEIGHT, render_origin.y, screen_height)};
    Vector2 map_coord{Vector2Add(
        render_origin,
        Vector2{
            .x = -static_cast<float>(HEX_HEIGHT / 2.0),
            .y = -HEX_RADIUS / 2.0
        }
    )};
    bool even_row{false};

    while (map_coord.y < map_render_max_y + HEX_RADIUS / 2.0) {
        while (map_coord.x < map_render_max_x + HEX_RADIUS / 2.0) {
            drawMapHex(renderCoordFromMapCoord(render_origin, map_coord));
            map_coord.x += HEX_HEIGHT;
        }
        even_row = !even_row;
        map_coord.y += HEX_RADIUS + HEX_SIDE_LENGTH / 2;
        map_coord.x = render_origin.x - (even_row ? 0 : HEX_HEIGHT / 2);
    }
}

} // namespace app
