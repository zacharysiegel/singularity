#pragma once

#include <vector>

#include "raylib.h"

#include "config.h"
#include "state.h"

namespace app {

using MapCoord = Vector2;
using RenderCoord = Vector2;

static std::vector<Hex> hexes(HEX_COUNT);

void initMap();
void drawMap(MapCoord render_origin);

}

