#pragma once

#include "raylib.h"

namespace app {

using MapCoord = Vector2;
using RenderCoord = Vector2;

void initMap();
void drawMap(MapCoord const map_origin);
void drawPlayers(MapCoord const map_origin);

} // namespace app
