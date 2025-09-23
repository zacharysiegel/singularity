#include <raylib.h>
#include <stdio.h>
#include <stdlib.h>

#include "engine.h"
#include "result.h"

int main(int argc, char **argv) {
    result_t result;

    result = init();
    if (result != OK) return result;

    result = run();
    if (result != OK) return result;

    result = destroy();
    if (result != OK) return result;

    return 0;
}
