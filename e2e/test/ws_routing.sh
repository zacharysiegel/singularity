#!/bin/zsh
set -uo pipefail
source "$(dirname "$0")/../harness.sh"

echo "=== WebSocket Routing E2E Tests ==="

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

ws_send "/ws/lobby" "$TOKEN_A" "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"hello from ws\"}" > /dev/null

MESSAGES=$(curl -s "$BASE_URL/conversation/$CONV_ID/messages" -H "Authorization: Bearer $TOKEN_A")
assert_equals "Message persisted via WS" "hello from ws" "$(echo "$MESSAGES" | jq -r '.[0].content')"
assert_equals "Sender is A" "$ACCOUNT_A_ID" "$(echo "$MESSAGES" | jq -r '.[0].sender_account_id')"

# --- B receives chat message event ---

ws_listener_open "/ws/lobby" "$TOKEN_B" B_STATE

ws_send "/ws/lobby" "$TOKEN_A" "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"realtime test\"}" > /dev/null

B_EVENT=$(ws_listener_collect "$B_STATE" | head -1)
assert_equals "B received event type" "ChatMessage" "$(echo "$B_EVENT" | jq -r '.type')"
assert_equals "B received event content" "realtime test" "$(echo "$B_EVENT" | jq -r '.content')"
assert_equals "B received event sender" "$ACCOUNT_A_ID" "$(echo "$B_EVENT" | jq -r '.sender_account_id')"
assert_equals "B received event conversation" "$CONV_ID" "$(echo "$B_EVENT" | jq -r '.conversation_id')"

# --- Sender also receives their own message ---

RESPONSE=$(ws_send "/ws/lobby" "$TOKEN_A" "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"echo to self\"}")
assert_equals "Sender receives own message" "echo to self" "$(echo "$RESPONSE" | jq -r '.content')"

# --- Non-member gets error ---

ACCOUNT_C=$(create_account "e2e_wsrt_c@test.com" "e2e_wsrt_c" "pass123")
TOKEN_C=$(login "e2e_wsrt_c@test.com" "pass123")

RESPONSE=$(ws_send "/ws/lobby" "$TOKEN_C" "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"should fail\"}")
assert_equals "Non-member gets error type" "Error" "$(echo "$RESPONSE" | jq -r '.type')"

# --- Invalid JSON gets error ---

RESPONSE=$(ws_send "/ws/lobby" "$TOKEN_A" "not json at all")
assert_equals "Invalid JSON gets error type" "Error" "$(echo "$RESPONSE" | jq -r '.type')"

# --- Non-member does NOT receive message ---

ws_listener_open "/ws/lobby" "$TOKEN_C" C_STATE

ws_send "/ws/lobby" "$TOKEN_A" "{\"type\":\"ChatMessage\",\"conversation_id\":\"$CONV_ID\",\"content\":\"private msg\"}" > /dev/null

C_EVENT=$(ws_listener_collect "$C_STATE")
assert_equals "Non-member receives nothing" "" "$C_EVENT"

# Cleanup
delete_account "$TOKEN_A"
delete_account "$TOKEN_B"
delete_account "$TOKEN_C"

print_results
