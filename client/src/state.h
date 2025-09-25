#include <cstdint>
#include <vector>

#include "app.h"

namespace app {

typedef struct Hex {
    // x-coordinate
    uint16_t i;
    // y-coordinate
    uint16_t j;
} Hex;

std::vector<Hex> const hexes(HEX_COUNT);

} // namespace app
