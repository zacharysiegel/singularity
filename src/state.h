#include <cstdint>
#include <vector>

#include "app.h"

namespace app {

typedef struct Hex {
    uint16_t i;
    uint16_t j;
} Hex;

std::vector<Hex> const hexes(HEX_COUNT);

} // namespace app
