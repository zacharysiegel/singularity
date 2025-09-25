#include <cassert>
#include <cstdint>
#include <sys/cdefs.h>

#include "raylib.h"

#include "app.h"
#include "map.h"
#include "state.h"
#include "util.h"

namespace app {

uint8_t const HEX_SIDES{6};
uint8_t const HEX_RADIUS{32};
double const HEX_SIDE_LENGTH{2 * SIN_PI_DIV_6 * HEX_RADIUS};
double const HEX_HEIGHT{SIN_PI_DIV_3 * HEX_RADIUS * 2};

uint16_t getIndexFromHexCoord(HexCoord hex_coord) {
    return hex_coord.i + hex_coord.j * HEX_COUNT_SQRT;
}

void initMap() {
    for (uint16_t i = 0; i < HEX_COUNT_SQRT; i++) {
        for (uint16_t j = 0; j < HEX_COUNT_SQRT; j++) {
            HexCoord hex_coord{.i = i, .j = j};
            hexes.at(getIndexFromHexCoord(hex_coord)) = Hex{.hex_coord = hex_coord};
        }
    }
}

Hex &getHexFromHexCoord(std::vector<Hex> &hexes, HexCoord hex_coord) {
    return hexes.at(getIndexFromHexCoord(hex_coord));
}

Vector2 mapCoordFromHexCoord(HexCoord hex_coord) {
    bool even_row = hex_coord.j % 2 == 0;
    float x = (hex_coord.i * HEX_HEIGHT) + (even_row ? 0 : HEX_HEIGHT / 2);
    float y = hex_coord.j * (HEX_RADIUS + HEX_SIDE_LENGTH / 2);
    return Vector2{
        .x = x,
        .y = y
    };
}

HexCoord hexCoordFromMapCoord(Vector2 map_coord) {
    uint16_t j = static_cast<uint16_t>(map_coord.y / (HEX_RADIUS + HEX_SIDE_LENGTH / 2));
    bool even_row = j % 2 == 0;
    uint16_t i = static_cast<uint16_t>(map_coord.x / (HEX_HEIGHT) + (even_row ? 0 : HEX_HEIGHT / 2));
    return HexCoord{
        .i = i,
        .j = j
    };
}

Vector2 renderCoordFromMapCoord(Vector2 render_origin, Vector2 map_coord) {
    return Vector2{
        .x = map_coord.x - render_origin.x,
        .y = map_coord.y - render_origin.y,
    };
}

void drawMapHex(Vector2 center_render_coord) {
    // { // Colored hex fill for debugging
    //     static bool color_switch{true};
    //     color_switch = !color_switch;
    //     DrawPoly(center_render_coord, HEX_SIDES, HEX_RADIUS, 30.0f, color_switch ? RED : BLUE);
    // }

    DrawPolyLinesEx(center_render_coord, HEX_SIDES, HEX_RADIUS, 30.0f, 1.0f, RAYWHITE);
}

void drawMap(Vector2 render_origin) {
    int32_t screen_width{GetScreenWidth()}; // is it any faster to call this function only once per frame? or does raylib already include this caching optimization?
    int32_t screen_height{GetScreenHeight()};
    HexCoord min_hex_coord{hexCoordFromMapCoord(render_origin)};
    HexCoord hex_coord{min_hex_coord};
    Vector2 map_coord{mapCoordFromHexCoord(hex_coord)};
    Vector2 render_coord{renderCoordFromMapCoord(render_origin, map_coord)};

    while (render_coord.y < screen_height + HEX_RADIUS) {
        while (render_coord.x < screen_width + HEX_HEIGHT) {
            drawMapHex(render_coord);

            hex_coord.i += 1;
            if (hex_coord.i >= HEX_COUNT_SQRT) {
                hex_coord.i = min_hex_coord.i;
            }
            map_coord = mapCoordFromHexCoord(hex_coord);
            render_coord = renderCoordFromMapCoord(render_origin, map_coord);
        }

        hex_coord.i = min_hex_coord.i;
        hex_coord.j += 1;
        if (hex_coord.j >= HEX_COUNT_SQRT) {
            hex_coord.j = min_hex_coord.j;
        }
        map_coord = mapCoordFromHexCoord(hex_coord);
        render_coord = renderCoordFromMapCoord(render_origin, map_coord);
    }
}

} // namespace app
