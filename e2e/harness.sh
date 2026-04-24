#!/bin/zsh
# Common test harness for e2e tests. Source this file, don't execute it.

BASE_URL="http://localhost:10000"
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

function create_account {
    local email="$1"
    local username="$2"
    local password="$3"
    curl -s -X POST "$BASE_URL/account" -H 'Content-Type: application/json' -d "{\"email\":\"$email\",\"username\":\"$username\",\"password\":\"$password\"}"
}

function login {
    local email="$1"
    local password="$2"
    curl -s -X POST "$BASE_URL/session" -H 'Content-Type: application/json' -d "{\"email\":\"$email\",\"password\":\"$password\"}" | jq -r '.token'
}

function delete_account {
    local token="$1"
    curl -s -o /dev/null -X DELETE "$BASE_URL/account" -H "Authorization: Bearer $token"
}

function print_results {
    echo ""
    echo "=== Results: $PASS passed, $FAIL failed ==="
    if [ "$FAIL" -gt 0 ]; then
        exit 1
    fi
}
