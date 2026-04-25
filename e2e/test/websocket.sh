#!/bin/zsh
set -uo pipefail
source "$(dirname "$0")/../harness.sh"

echo "=== WebSocket E2E Tests ==="

WS_BASE_URL="ws://localhost:10000"

# Setup
create_account "e2e_ws@test.com" "e2e_ws" "pass123" > /dev/null
TOKEN=$(login "e2e_ws@test.com" "pass123")

# --- Auth ---

# No auth header → 401
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/lobby")
assert_status "Lobby WS without auth" "401" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/live")
assert_status "Live WS without auth" "401" "$STATUS"

# Bad token → 401
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/lobby" -H "Authorization: Bearer bad_token")
assert_status "Lobby WS with bad token" "401" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/live" -H "Authorization: Bearer bad_token")
assert_status "Live WS with bad token" "401" "$STATUS"

# --- Lobby WS: echo ---

ECHO_RESPONSE=$(echo "hello lobby" | websocat -1 "$WS_BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
assert_equals "Lobby WS echo" "echo: hello lobby" "$ECHO_RESPONSE"

# --- Live WS: echo ---

ECHO_RESPONSE=$(echo "hello live" | websocat -1 "$WS_BASE_URL/ws/live" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
assert_equals "Live WS echo" "echo: hello live" "$ECHO_RESPONSE"

# --- Lobby WS: multiple messages ---

RESPONSES=$(printf "msg1\nmsg2\nmsg3" | websocat "$WS_BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN" 2>/dev/null)
RESPONSE_COUNT=$(echo "$RESPONSES" | wc -l | tr -d ' ')
assert_equals "Lobby WS multiple messages count" "3" "$RESPONSE_COUNT"
FIRST_RESPONSE=$(echo "$RESPONSES" | head -1)
assert_equals "Lobby WS first echo" "echo: msg1" "$FIRST_RESPONSE"

# --- Session invalidation ---

# Delete account, then try WS
delete_account "$TOKEN"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN")
assert_status "Lobby WS after account deleted" "401" "$STATUS"

print_results
