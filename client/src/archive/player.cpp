#include <cstdint>
#include <sys/cdefs.h>
#include <vector>

#include "config.h"
#include "state.h"

namespace app {

void initPlayers(uint8_t const player_count) {
    state.players.reserve(player_count);

    for (uint8_t p = 0; p < player_count; p++) {
        state.players.push_back(Player{p});
        Player &player = state.players.at(p);

        HexCoord facility_location = HexCoord{
            .i = static_cast<uint16_t>(HEX_COUNT_SQRT / player_count * p),
            .j = static_cast<uint16_t>(HEX_COUNT_SQRT / player_count * p)
        };
        Facility facility = Facility{
            .location = facility_location,
            .facility_type = FacilityType::ControlCenter,
            .facility_state = FacilityState::Operating,
        };
        player.facilities.push_back(facility);
    }
}

} // namespace app
