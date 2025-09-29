#pragma once

#include <array>
#include <cassert>
#include <cstdint>
#include <cstdlib>
#include <vector>

#include "config.h"
#include "map.h"

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

enum class FacilityType : uint8_t {
    ControlCenter = 0,
    MetalExtractor,
    OilExtractor,
};

enum class FacilityState : uint8_t {
    Operating = 0,
    Placing,
    Destroyed,
};

typedef struct Facility {
    HexCoord location;
    FacilityType facility_type;
    FacilityState facility_state;
} Facility;

typedef struct Player {
    uint8_t id;
    std::vector<Facility> facilities;

    Player(uint8_t id)
        : id{id} {}
} Player;

typedef struct ClientState {
    MapCoord map_origin;
    std::array<Hex, HEX_COUNT> hexes;
    std::vector<Player> players;
} ClientState;

inline ClientState state{
    .map_origin = MapCoord{.x = 0, .y = 0},
    .hexes{},
    .players{},
};

} // namespace app
