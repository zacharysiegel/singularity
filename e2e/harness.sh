#!/bin/zsh
# Common test harness for e2e tests. Source this file, don't execute it.

source "$(dirname "$0")/actions.sh"

LOBBY_HOST="${LOBBY_HOST:-localhost}"
LOBBY_PORT="${LOBBY_PORT:-10000}"

# shellcheck disable=SC2034 # used by dependent scripts
BASE_URL="http://${LOBBY_HOST}:${LOBBY_PORT}"
# shellcheck disable=SC2034 # used by dependent scripts
WS_BASE_URL="ws://${LOBBY_HOST}:${LOBBY_PORT}"
PASS=0
FAIL=0

function assert_equals {
    local test_name="$1"
    local expected="$2"
    local actual="$3"
    if [ "$actual" = "$expected" ]; then
        echo "  PASS: $test_name"
        PASS=$((PASS + 1))
    else
        echo "  FAIL: $test_name (expected [$expected], got [$actual])"
        FAIL=$((FAIL + 1))
    fi
}

function assert_status {
    local test_name="$1"
    local expected="$2"
    local actual="$3"
    assert_equals "$test_name" "$expected" "$actual"
}

function print_header {
    local suite_name="$1"
    echo "=== $suite_name ==="
}

function print_results {
    echo ""
    echo "=== Results: $PASS passed, $FAIL failed ==="
    if [ "$FAIL" -gt 0 ]; then
        exit 1
    fi
}
