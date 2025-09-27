#pragma once

#include "raylib.h"
namespace app {

using MapCoord = Vector2;
using RenderCoord = Vector2;

void initMap();
void drawMap(MapCoord render_origin);

}

