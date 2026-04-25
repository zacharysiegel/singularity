#!/bin/zsh
set -uo pipefail
source "$(dirname "$0")/../harness.sh"

echo "=== WebSocket Routing E2E Tests ==="

WS_BASE_URL="ws://localhost:10000"

# Helper: open a persistent WS connection via FIFO
# Usage: ws_open <token> <fifo_var> <outfile_var> <pid_var>
function ws_open {
    local token="$1"
    local fifo_name="$2"
    local outfile_name="$3"
    local pid_name="$4"

    local fifo=$(mktemp -u /tmp/e2e_ws_fifo.XXXXXX)
    mkfifo "$fifo"
    local outfile=$(mktemp /tmp/e2e_ws_out.XXXXXX)

    websocat "$WS_BASE_URL/ws/lobby" -H "Authorization: Bearer $token" < "$fifo" > "$outfile" 2>/dev/null &
    local pid=$!

    eval "$fifo_name='$fifo'"
    eval "$outfile_name='$outfile'"
    eval "$pid_name=$pid"

    eval "exec 3>'$fifo'"
    sleep 0.3
}

# Helper: send a message on an open WS connection
function ws_send {
    local message="$1"
    echo "$message" >&3
}

# Helper: close a WS connection and collect output
function ws_close {
    local fifo="$1"
    local pid="$2"
    exec 3>&-
    kill "$pid" 2>/dev/null; wait "$pid" 2>/dev/null
    rm -f "$fifo"
}

# Setup: two accounts in a conversation
ACCOUNT_A=$(create_account "e2e_wsrt_a@test.com" "e2e_wsrt_a" "pass123")
ACCOUNT_A_ID=$(echo "$ACCOUNT_A" | jq -r '.id')
ACCOUNT_B=$(create_account "e2e_wsrt_b@test.com" "e2e_wsrt_b" "pass123")
ACCOUNT_B_ID=$(echo "$ACCOUNT_B" | jq -r '.id')
TOKEN_A=$(login "e2e_wsrt_a@test.com" "pass123")
TOKEN_B=$(login "e2e_wsrt_b@test.com" "pass123")

CONV=$(curl -s -X POST "$BASE_URL/conversation" -H "Authorization: Bearer $TOKEN_A" -H 'Content-Type: application/json' -d "{\"member_account_ids\":[\"$ACCOUNT_B_ID\"],\"name\":\"WS Route Test\"}")
CONV_ID=$(echo "$CONV" | jq -r '.id')

# --- Send chat message, verify persisted via REST ---

ws_open "$TOKEN_A" A_FIFO A_OUT A_PID
ws_send "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"hello from ws\"}"
sleep 0.3
ws_close "$A_FIFO" "$A_PID"

MESSAGES=$(curl -s "$BASE_URL/conversation/$CONV_ID/messages" -H "Authorization: Bearer $TOKEN_A")
assert_equals "Message persisted via WS" "hello from ws" "$(echo "$MESSAGES" | jq -r '.[0].content')"
assert_equals "Sender is A" "$ACCOUNT_A_ID" "$(echo "$MESSAGES" | jq -r '.[0].sender_account_id')"

# --- B receives chat message event ---

# Open B's listener first, then A sends
B_FIFO=$(mktemp -u /tmp/e2e_ws_fifo.XXXXXX)
mkfifo "$B_FIFO"
B_OUT=$(mktemp /tmp/e2e_ws_out.XXXXXX)
websocat "$WS_BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN_B" < "$B_FIFO" > "$B_OUT" 2>/dev/null &
B_PID=$!
exec 4>"$B_FIFO"
sleep 0.3

# A sends via a separate connection
ws_open "$TOKEN_A" A_FIFO A_OUT A_PID
ws_send "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"realtime test\"}"
sleep 0.3
ws_close "$A_FIFO" "$A_PID"

# Close B's listener
exec 4>&-
kill "$B_PID" 2>/dev/null; wait "$B_PID" 2>/dev/null

B_EVENT=$(head -1 "$B_OUT")
assert_equals "B received event type" "ChatMessage" "$(echo "$B_EVENT" | jq -r '.type')"
assert_equals "B received event content" "realtime test" "$(echo "$B_EVENT" | jq -r '.content')"
assert_equals "B received event sender" "$ACCOUNT_A_ID" "$(echo "$B_EVENT" | jq -r '.sender_account_id')"
assert_equals "B received event conversation" "$CONV_ID" "$(echo "$B_EVENT" | jq -r '.conversation_id')"
rm -f "$B_FIFO" "$B_OUT"

# --- Sender also receives their own message ---

ws_open "$TOKEN_A" A_FIFO A_OUT A_PID
ws_send "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"echo to self\"}"
sleep 0.3
ws_close "$A_FIFO" "$A_PID"

A_EVENT=$(head -1 "$A_OUT")
assert_equals "Sender receives own message" "echo to self" "$(echo "$A_EVENT" | jq -r '.content')"
rm -f "$A_OUT"

# --- Non-member gets error ---

ACCOUNT_C=$(create_account "e2e_wsrt_c@test.com" "e2e_wsrt_c" "pass123")
TOKEN_C=$(login "e2e_wsrt_c@test.com" "pass123")

C_FIFO=$(mktemp -u /tmp/e2e_ws_fifo.XXXXXX)
mkfifo "$C_FIFO"
C_OUT=$(mktemp /tmp/e2e_ws_out.XXXXXX)
websocat "$WS_BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN_C" < "$C_FIFO" > "$C_OUT" 2>/dev/null &
C_PID=$!
exec 3>"$C_FIFO"
sleep 0.3
echo "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"should fail\"}" >&3
sleep 0.3
exec 3>&-
kill "$C_PID" 2>/dev/null; wait "$C_PID" 2>/dev/null

C_RESPONSE=$(head -1 "$C_OUT")
assert_equals "Non-member gets error type" "Error" "$(echo "$C_RESPONSE" | jq -r '.type')"
rm -f "$C_FIFO" "$C_OUT"

# --- Invalid JSON gets error ---

ws_open "$TOKEN_A" A_FIFO A_OUT A_PID
ws_send "not json at all"
sleep 0.3
ws_close "$A_FIFO" "$A_PID"

INVALID_RESPONSE=$(head -1 "$A_OUT")
assert_equals "Invalid JSON gets error type" "Error" "$(echo "$INVALID_RESPONSE" | jq -r '.type')"
rm -f "$A_OUT"

# --- Non-member does NOT receive message ---

C_FIFO=$(mktemp -u /tmp/e2e_ws_fifo.XXXXXX)
mkfifo "$C_FIFO"
C_OUT=$(mktemp /tmp/e2e_ws_out.XXXXXX)
websocat "$WS_BASE_URL/ws/lobby" -H "Authorization: Bearer $TOKEN_C" < "$C_FIFO" > "$C_OUT" 2>/dev/null &
C_PID=$!
exec 4>"$C_FIFO"
sleep 0.3

ws_open "$TOKEN_A" A_FIFO A_OUT A_PID
ws_send "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"private msg\"}"
sleep 0.3
ws_close "$A_FIFO" "$A_PID"
rm -f "$A_OUT"

exec 4>&-
kill "$C_PID" 2>/dev/null; wait "$C_PID" 2>/dev/null

C_EVENT=$(cat "$C_OUT")
assert_equals "Non-member receives nothing" "" "$C_EVENT"
rm -f "$C_FIFO" "$C_OUT"

# Cleanup
delete_account "$TOKEN_A"
delete_account "$TOKEN_B"
delete_account "$TOKEN_C"

print_results
