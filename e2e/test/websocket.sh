#!/bin/zsh
set -uo pipefail
source "$(dirname "$0")/../harness.sh"

echo "=== WebSocket E2E Tests ==="

WS_BASE_URL="ws://localhost:10000"

# Setup
create_account "e2e_ws@test.com" "e2e_ws" "pass123" > /dev/null
TOKEN=$(login "e2e_ws@test.com" "pass123")

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

FIFO=$(mktemp -u /tmp/e2e_ws_fifo.XXXXXX)
mkfifo "$FIFO"
RESPONSE_FILE=$(mktemp /tmp/e2e_ws_out.XXXXXX)

websocat "$WS_BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN" < "$FIFO" > "$RESPONSE_FILE" 2>/dev/null &
WS_PID=$!
exec 3>"$FIFO"
sleep 0.3
echo "not json" >&3
sleep 0.3
exec 3>&-
kill $WS_PID 2>/dev/null; wait $WS_PID 2>/dev/null
RESPONSE=$(head -1 "$RESPONSE_FILE")
assert_equals "Invalid text returns error" "Error" "$(echo "$RESPONSE" | jq -r '.type')"
rm -f "$FIFO" "$RESPONSE_FILE"

FIFO=$(mktemp -u /tmp/e2e_ws_fifo.XXXXXX)
mkfifo "$FIFO"
RESPONSE_FILE=$(mktemp /tmp/e2e_ws_out.XXXXXX)

websocat "$WS_BASE_URL/ws/live" -H "Authorization: Bearer $TOKEN" < "$FIFO" > "$RESPONSE_FILE" 2>/dev/null &
WS_PID=$!
exec 3>"$FIFO"
sleep 0.3
echo "not json" >&3
sleep 0.3
exec 3>&-
kill $WS_PID 2>/dev/null; wait $WS_PID 2>/dev/null
RESPONSE=$(head -1 "$RESPONSE_FILE")
assert_equals "Invalid text on live returns error" "Error" "$(echo "$RESPONSE" | jq -r '.type')"
rm -f "$FIFO" "$RESPONSE_FILE"

# --- Session invalidation ---

delete_account "$TOKEN"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN")
assert_status "Lobby WS after account deleted" "401" "$STATUS"

print_results
