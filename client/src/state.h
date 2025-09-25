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

typedef struct Hex {
    HexCoord hex_coord;
} Hex;

static std::vector<Hex> hexes(HEX_COUNT);

} // namespace app
