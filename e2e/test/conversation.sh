#!/bin/zsh
set -uo pipefail
source "$(dirname "$0")/../harness.sh"

print_header "Conversation E2E Tests"

# A and B are conversation members. C is not.
EMAIL_A="e2e_conv_a@test.com"
EMAIL_B="e2e_conv_b@test.com"
PASSWORD="pass123"

ACCOUNT_A=$(create_account "$EMAIL_A" "e2e_conv_a" "$PASSWORD")
ACCOUNT_A_ID=$(echo "$ACCOUNT_A" | jq -r '.id')
ACCOUNT_B=$(create_account "$EMAIL_B" "e2e_conv_b" "$PASSWORD")
ACCOUNT_B_ID=$(echo "$ACCOUNT_B" | jq -r '.id')
TOKEN_A=$(login "$EMAIL_A" "$PASSWORD")
TOKEN_B=$(login "$EMAIL_B" "$PASSWORD")

# Create conversation
CONV=$(curl -s -X POST "$BASE_URL/conversation" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"member_account_ids":["$ACCOUNT_B_ID"],"name":"Test Chat"}
EOF
)")
CONV_ID=$(echo "$CONV" | jq -r '.id')
STATUS=$(echo "$CONV" | jq -r '.name')
assert_equals "Create conversation name" "Test Chat" "$STATUS"

# List conversations for A
BODY=$(curl -s "$BASE_URL/conversation" -H "Authorization: Bearer $TOKEN_A")
assert_equals "A has 1 conversation" "1" "$(echo "$BODY" | jq 'length')"

# List conversations for B
BODY=$(curl -s "$BASE_URL/conversation" -H "Authorization: Bearer $TOKEN_B")
assert_equals "B has 1 conversation" "1" "$(echo "$BODY" | jq 'length')"

# Get conversation details
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/conversation/$CONV_ID" -H "Authorization: Bearer $TOKEN_A")
assert_status "Get conversation details" "200" "$STATUS"

# Non-member can't access
EMAIL_C="e2e_conv_c@test.com"
ACCOUNT_C=$(create_account "$EMAIL_C" "e2e_conv_c" "$PASSWORD")
ACCOUNT_C_ID=$(echo "$ACCOUNT_C" | jq -r '.id')
TOKEN_C=$(login "$EMAIL_C" "$PASSWORD")
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/conversation/$CONV_ID" -H "Authorization: Bearer $TOKEN_C")
assert_status "Non-member can't get details" "403" "$STATUS"

# Send message from A
MSG=$(curl -s -X POST "$BASE_URL/conversation/$CONV_ID/messages" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"conversation_id":"$CONV_ID","content":"hello from A"}
EOF
)")
assert_equals "Message sender is A" "$ACCOUNT_A_ID" "$(echo "$MSG" | jq -r '.sender_account_id')"
assert_equals "Message content" "hello from A" "$(echo "$MSG" | jq -r '.content')"

# Send message from B
MSG2=$(curl -s -X POST "$BASE_URL/conversation/$CONV_ID/messages" \
    -H "Authorization: Bearer $TOKEN_B" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"conversation_id":"$CONV_ID","content":"hello from B"}
EOF
)")
assert_equals "B's message content" "hello from B" "$(echo "$MSG2" | jq -r '.content')"

# Get message history
BODY=$(curl -s "$BASE_URL/conversation/$CONV_ID/messages" -H "Authorization: Bearer $TOKEN_A")
assert_equals "2 messages in history" "2" "$(echo "$BODY" | jq 'length')"

# Get message history with limit
BODY=$(curl -s "$BASE_URL/conversation/$CONV_ID/messages?limit=1" -H "Authorization: Bearer $TOKEN_A")
assert_equals "Limit to 1 message" "1" "$(echo "$BODY" | jq 'length')"

# Non-member can't send message
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/conversation/$CONV_ID/messages" \
    -H "Authorization: Bearer $TOKEN_C" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"conversation_id":"$CONV_ID","content":"should fail"}
EOF
)")
assert_status "Non-member can't send message" "403" "$STATUS"

# Non-member can't read messages
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/conversation/$CONV_ID/messages" -H "Authorization: Bearer $TOKEN_C")
assert_status "Non-member can't read messages" "403" "$STATUS"

# Add member C
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/conversation/$CONV_ID/member" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"account_id":"$ACCOUNT_C_ID"}
EOF
)")
assert_status "Add member C" "200" "$STATUS"

# Duplicate add member C
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/conversation/$CONV_ID/member" \
    -H "Authorization: Bearer $TOKEN_A" \
    -H 'Content-Type: application/json' \
    -d "$(cat <<EOF
{"account_id":"$ACCOUNT_C_ID"}
EOF
)")
assert_status "Duplicate add member" "409" "$STATUS"

# C can now read messages
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/conversation/$CONV_ID/messages" -H "Authorization: Bearer $TOKEN_C")
assert_status "C can now read messages" "200" "$STATUS"

# A leaves conversation
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/conversation/$CONV_ID/leave" -H "Authorization: Bearer $TOKEN_A")
assert_status "A leaves conversation" "200" "$STATUS"

# A can no longer access
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/conversation/$CONV_ID" -H "Authorization: Bearer $TOKEN_A")
assert_status "A can't access after leaving" "403" "$STATUS"

# B leaves, C leaves — conversation should be auto-deleted
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/conversation/$CONV_ID/leave" -H "Authorization: Bearer $TOKEN_B")
assert_status "B leaves conversation" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/conversation/$CONV_ID/leave" -H "Authorization: Bearer $TOKEN_C")
assert_status "C leaves conversation (last member)" "200" "$STATUS"

# Conversation should be deleted
BODY=$(curl -s "$BASE_URL/conversation" -H "Authorization: Bearer $TOKEN_B")
assert_equals "B has 0 conversations after all left" "0" "$(echo "$BODY" | jq 'length')"

# Cleanup
delete_account "$TOKEN_A"
delete_account "$TOKEN_B"
delete_account "$TOKEN_C"

print_results
