#!/bin/zsh
set -uo pipefail
source "$(dirname "$0")/../harness.sh"

print_header "WebSocket E2E Tests"

EMAIL="e2e_ws@test.com"
PASSWORD="pass123"

create_account "$EMAIL" "e2e_ws" "$PASSWORD" > /dev/null
TOKEN=$(login "$EMAIL" "$PASSWORD")

# --- Auth ---

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/lobby")
assert_status "Lobby WS without auth" "401" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/live")
assert_status "Live WS without auth" "401" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/lobby" -H "Authorization: Bearer bad_token")
assert_status "Lobby WS with bad token" "401" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/live" -H "Authorization: Bearer bad_token")
assert_status "Live WS with bad token" "401" "$STATUS"

# --- Invalid messages return errors ---

RESPONSE=$(ws_send "/ws/lobby" "$TOKEN" "not json")
assert_equals "Invalid text returns error" "Error" "$(echo "$RESPONSE" | jq -r '.type')"

RESPONSE=$(ws_send "/ws/live" "$TOKEN" "not json")
assert_equals "Invalid text on live returns error" "Error" "$(echo "$RESPONSE" | jq -r '.type')"

# --- Session invalidation ---

delete_account "$TOKEN"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN")
assert_status "Lobby WS after account deleted" "401" "$STATUS"

print_results
