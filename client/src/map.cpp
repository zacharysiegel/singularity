#include <cassert>
#include <cstdint>
#include <format>

#include "raylib.h"

#include "config.h"
#include "map.h"
#include "state.h"

namespace app {

uint16_t getIndexFromHexCoord(HexCoord hex_coord) {
    return hex_coord.i + hex_coord.j * HEX_COUNT_SQRT;
}

// todo: implement planned strategy (plan.md)
ResourceType initResourceTypeFromHexCoord(uint16_t i, uint16_t j) {
    if (i % (HEX_COUNT_SQRT / 4) == 10 && j % (HEX_COUNT_SQRT / 4) == 4) {
        return ResourceType::Metal;
    } else if (i % (HEX_COUNT_SQRT / 4) == 2 && j % (HEX_COUNT_SQRT / 4) == 12) {
        return ResourceType::Oil;
    }
    return ResourceType::None;
}

void initMap() {
    for (uint16_t i = 0; i < HEX_COUNT_SQRT; i++) {
        for (uint16_t j = 0; j < HEX_COUNT_SQRT; j++) {
            Hex hex{
                .hex_coord = HexCoord{.i = i, .j = j},
                .resource_type = initResourceTypeFromHexCoord(i, j)
            };
            hexes.at(getIndexFromHexCoord(hex.hex_coord)) = hex;
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
    uint16_t i = static_cast<uint16_t>((map_coord.x - (even_row ? 0 : HEX_HEIGHT / 2)) / HEX_HEIGHT);
    return HexCoord{
        .i = i,
        .j = j
    };
}

Vector2 renderCoordFromMapCoord(Vector2 map_origin, Vector2 map_coord) {
    float x{map_coord.x - map_origin.x};
    float y{map_coord.y - map_origin.y};
    x = (x < -HEX_HEIGHT / 2)
            ? x + getMapWidthPixels()
            : x;
    y = (y < -HEX_RADIUS)
            ? y + getMapHeightPixels() - (HEX_SIDE_LENGTH / 2)
            : y;
    return Vector2{
        .x = x,
        .y = y,
    };
}

void drawMapHex(Vector2 map_origin, HexCoord hex_coord) {
    // { // Colored hex fill for debugging
    //     static bool color_switch{true};
    //     color_switch = !color_switch;
    //     DrawPoly(center_render_coord, HEX_SIDES, HEX_RADIUS, 30.0f, color_switch ? RED : BLUE);
    // }

    Vector2 map_coord = mapCoordFromHexCoord(hex_coord);
    Vector2 render_coord = renderCoordFromMapCoord(map_origin, map_coord);
    Hex hex = getHexFromHexCoord(hexes, hex_coord);
    Color color = colorFromResourceType(hex.resource_type);

    if (hex.resource_type != ResourceType::None) {
        DrawPoly(render_coord, HEX_SIDES, HEX_RADIUS, HEX_ROTATION, color);
    }
    DrawPolyLinesEx(render_coord, HEX_SIDES, HEX_RADIUS, HEX_ROTATION, 1.0f, HEX_OUTLINE_COLOR);
    DrawText(std::format("({}, {})", hex_coord.i, hex_coord.j).c_str(), render_coord.x, render_coord.y, 10, RAYWHITE);
}

/**
 * {map_origin} Map coordinate pinned to the top left corner of the screen
 */
void drawMap(Vector2 map_origin) {
    int32_t screen_width{GetScreenWidth()}; // is it any faster to call this function only once per frame? or does raylib already include this caching optimization?
    int32_t screen_height{GetScreenHeight()};
    HexCoord min_hex_coord{hexCoordFromMapCoord(map_origin)};
    HexCoord hex_coord{min_hex_coord};

    uint16_t max_hexes_i = getHexCountWidth(screen_width);
    uint16_t max_hexes_j = getHexCountHeight(screen_height);
    for (uint16_t hexes_drawn_j = 0; hexes_drawn_j <= max_hexes_j + 1; hexes_drawn_j += 1) {
        for (uint16_t hexes_drawn_i = 0; hexes_drawn_i <= max_hexes_i + 1; hexes_drawn_i += 1) {
            drawMapHex(map_origin, hex_coord);

            hex_coord.i += 1;
            if (hex_coord.i >= HEX_COUNT_SQRT) {
                hex_coord.i = 0;
            }
        }

        hex_coord.i = min_hex_coord.i;
        hex_coord.j += 1;
        if (hex_coord.j >= HEX_COUNT_SQRT) {
            hex_coord.j = 0;
        }
    }
}

} // namespace app
