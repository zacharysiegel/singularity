#!/bin/zsh
set -uo pipefail
source "$(dirname "$0")/../harness.sh"

print_header "In-Game Chat E2E Tests"

# A, B, C are game members. D is not a game member.
EMAIL_A="e2e_igc_a@test.com"
EMAIL_B="e2e_igc_b@test.com"
EMAIL_C="e2e_igc_c@test.com"
EMAIL_D="e2e_igc_d@test.com"
PASSWORD="pass123"

ACCOUNT_A=$(create_account "$EMAIL_A" "e2e_igc_a" "$PASSWORD")
ACCOUNT_A_ID=$(echo "$ACCOUNT_A" | jq -r '.id')
ACCOUNT_B=$(create_account "$EMAIL_B" "e2e_igc_b" "$PASSWORD")
ACCOUNT_B_ID=$(echo "$ACCOUNT_B" | jq -r '.id')
ACCOUNT_C=$(create_account "$EMAIL_C" "e2e_igc_c" "$PASSWORD")
ACCOUNT_C_ID=$(echo "$ACCOUNT_C" | jq -r '.id')
ACCOUNT_D=$(create_account "$EMAIL_D" "e2e_igc_d" "$PASSWORD")
ACCOUNT_D_ID=$(echo "$ACCOUNT_D" | jq -r '.id')
TOKEN_A=$(login "$EMAIL_A" "$PASSWORD")
TOKEN_B=$(login "$EMAIL_B" "$PASSWORD")
TOKEN_C=$(login "$EMAIL_C" "$PASSWORD")
TOKEN_D=$(login "$EMAIL_D" "$PASSWORD")

# Create game. A is the creator and first member.
GAME=$(curl -s -X POST "$BASE_URL/game" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<'EOF'
{"name":"E2E In-Game Chat Test"}
EOF
)")
GAME_ID=$(echo "$GAME" | jq -r '.id')
assert_equals "Game created" "Pending" "$(echo "$GAME" | jq -r '.status')"

# B and C join the game
curl -s -o /dev/null -X POST "$BASE_URL/game/$GAME_ID/member" -H "Authorization: Bearer $TOKEN_B"
curl -s -o /dev/null -X POST "$BASE_URL/game/$GAME_ID/member" -H "Authorization: Bearer $TOKEN_C"

# --- Transition to Active ---

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X PATCH "$BASE_URL/game/$GAME_ID/status" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<'EOF'
{"status":"Active"}
EOF
)")
assert_status "Transition to Active" "200" "$STATUS"

# Verify whole-game conversation was auto-created
CONVS=$(curl -s "$BASE_URL/game/$GAME_ID/conversation" -H "Authorization: Bearer $TOKEN_A")
assert_equals "A has 1 game conversation" "1" "$(echo "$CONVS" | jq 'length')"
GAME_CONV_ID=$(echo "$CONVS" | jq -r '.[0].id')
GAME_CONV_NAME=$(echo "$CONVS" | jq -r '.[0].name')
assert_equals "Game conversation name" "Global [E2E In-Game Chat Test]" "$GAME_CONV_NAME"

# B also sees the conversation
CONVS_B=$(curl -s "$BASE_URL/game/$GAME_ID/conversation" -H "Authorization: Bearer $TOKEN_B")
assert_equals "B has 1 game conversation" "1" "$(echo "$CONVS_B" | jq 'length')"

# Non-member D cannot list game conversations
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/game/$GAME_ID/conversation" -H "Authorization: Bearer $TOKEN_D")
assert_status "Non-member can't list conversations" "403" "$STATUS"

# --- Send message in game conversation ---

MSG=$(curl -s -X POST "$BASE_URL/conversation/$GAME_CONV_ID/messages" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"conversation_id":"$GAME_CONV_ID","content":"hello from game"}
EOF
)")
assert_equals "Message content" "hello from game" "$(echo "$MSG" | jq -r '.content')"

# --- Cannot leave in-game conversation ---

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/conversation/$GAME_CONV_ID/leave" -H "Authorization: Bearer $TOKEN_A")
assert_status "Cannot leave in-game conversation" "403" "$STATUS"

# --- Create in-game DM ---

DM=$(curl -s -X POST "$BASE_URL/game/$GAME_ID/conversation" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"member_account_ids":["$ACCOUNT_B_ID"],"name":"DM"}
EOF
)")
DM_ID=$(echo "$DM" | jq -r '.id')
assert_equals "DM created" "DM" "$(echo "$DM" | jq -r '.name')"

# A now has 2 conversations
CONVS=$(curl -s "$BASE_URL/game/$GAME_ID/conversation" -H "Authorization: Bearer $TOKEN_A")
assert_equals "A has 2 game conversations" "2" "$(echo "$CONVS" | jq 'length')"

# --- Duplicate DM detection ---

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/game/$GAME_ID/conversation" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"member_account_ids":["$ACCOUNT_B_ID"],"name":"DM duplicate"}
EOF
)")
assert_status "Duplicate DM rejected" "409" "$STATUS"

# --- Non-game-member cannot be added ---

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/game/$GAME_ID/conversation" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"member_account_ids":["$ACCOUNT_D_ID"],"name":"Invalid"}
EOF
)")
assert_status "Non-game-member rejected" "400" "$STATUS"

# --- Invalid status transitions ---

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X PATCH "$BASE_URL/game/$GAME_ID/status" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<'EOF'
{"status":"Pending"}
EOF
)")
assert_status "Active to Pending rejected" "400" "$STATUS"

# Non-creator cannot transition
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X PATCH "$BASE_URL/game/$GAME_ID/status" \
    -H "Authorization: Bearer $TOKEN_B" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<'EOF'
{"status":"Completed"}
EOF
)")
assert_status "Non-creator cannot transition" "403" "$STATUS"

# --- Transition to Completed ---

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X PATCH "$BASE_URL/game/$GAME_ID/status" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<'EOF'
{"status":"Completed"}
EOF
)")
assert_status "Transition to Completed" "200" "$STATUS"

# Conversations still accessible after completion
CONVS=$(curl -s "$BASE_URL/game/$GAME_ID/conversation" -H "Authorization: Bearer $TOKEN_A")
assert_equals "Conversations persist after completion" "2" "$(echo "$CONVS" | jq 'length')"

# Cleanup
delete_account "$TOKEN_A"
delete_account "$TOKEN_B"
delete_account "$TOKEN_C"
delete_account "$TOKEN_D"

print_results
