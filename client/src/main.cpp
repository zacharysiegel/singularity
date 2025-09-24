#include <raylib.h>
#include <stdio.h>
#include <stdlib.h>

#include "engine.h"
#include "result.h"

int main(int argc, char **argv) {
	app::result_t result;

    result = app::init();
    if (result != app::OK) return result;

    result = app::run();
    if (result != app::OK) return result;

    result = app::destroy();
    if (result != app::OK) return result;

    return 0;
}
