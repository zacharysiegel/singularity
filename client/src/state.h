#include <cstdint>
#include <vector>

#include "app.h"

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

typedef struct Hex {
    HexCoord hex_coord;
    ResourceType resource_type;
} Hex;

static std::vector<Hex> hexes(HEX_COUNT);

} // namespace app
