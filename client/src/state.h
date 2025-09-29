#pragma once

#include <cassert>
#include <cstdint>

#include "config.h"

namespace app {

typedef struct HexCoord {
    // x-coordinate
    uint16_t i;
    // y-coordinate
    uint16_t j;
} HexCoord;

enum class ResourceType : uint8_t {
    None = 0,
    Metal,
    Oil,
};

inline constexpr Color colorFromResourceType(ResourceType resource_type) {
    switch (resource_type) {
        case ResourceType::None:
            return Color{.r = 0x00, .g = 0x00, .b = 0x00, .a = 0x00};
        case ResourceType::Metal:
            return METAL_BACKGROUND_COLOR;
        case ResourceType::Oil:
            return OIL_BACKGROUND_COLOR;
        default:
            assert(false && "invalid case");
    }
}

typedef struct Hex {
    HexCoord hex_coord;
    ResourceType resource_type;
} Hex;

} // namespace app
