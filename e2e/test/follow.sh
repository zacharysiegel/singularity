#!/bin/zsh
set -euo pipefail
source "$(dirname "$0")/../harness.sh"

echo "=== Follow E2E Tests ==="

# Setup
ACCOUNT_A=$(create_account "e2e_follow_a@test.com" "e2e_follow_a" "pass123")
ACCOUNT_A_ID=$(echo "$ACCOUNT_A" | jq -r '.id')
ACCOUNT_B=$(create_account "e2e_follow_b@test.com" "e2e_follow_b" "pass123")
ACCOUNT_B_ID=$(echo "$ACCOUNT_B" | jq -r '.id')
TOKEN_A=$(login "e2e_follow_a@test.com" "pass123")
TOKEN_B=$(login "e2e_follow_b@test.com" "pass123")

# A follows B
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/account/$ACCOUNT_B_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "A follows B" "200" "$STATUS"

# B follows A
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/account/$ACCOUNT_A_ID/follow" -H "Authorization: Bearer $TOKEN_B")
assert_status "B follows A" "200" "$STATUS"

# Mutuals
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_A_ID/mutuals")
assert_equals "Mutuals returns 1" "1" "$(echo "$BODY" | jq 'length')"
assert_equals "is_mutual is true" "true" "$(echo "$BODY" | jq -r '.[0].is_mutual')"

# Followers
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_A_ID/followers")
assert_equals "A has 1 follower" "1" "$(echo "$BODY" | jq 'length')"

# Following
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_A_ID/following")
assert_equals "A is following 1" "1" "$(echo "$BODY" | jq 'length')"

# Duplicate follow
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/account/$ACCOUNT_B_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "Duplicate follow" "409" "$STATUS"

# Self-follow
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/account/$ACCOUNT_A_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "Self-follow" "500" "$STATUS"

# Unfollow
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE_URL/account/$ACCOUNT_B_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "A unfollows B" "200" "$STATUS"

# Unfollow non-existent
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE_URL/account/$ACCOUNT_B_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "Unfollow non-existent" "404" "$STATUS"

# Mutuals after unfollow
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_A_ID/mutuals")
assert_equals "Mutuals empty after unfollow" "0" "$(echo "$BODY" | jq 'length')"

# B's followers after A unfollow
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_B_ID/followers")
assert_equals "B has 0 followers" "0" "$(echo "$BODY" | jq 'length')"

# Public profile still works
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/account/$ACCOUNT_A_ID")
assert_status "Account public profile" "200" "$STATUS"

# Soft delete cleanup
curl -s -o /dev/null -X POST "$BASE_URL/account/$ACCOUNT_A_ID/follow" -H "Authorization: Bearer $TOKEN_B"
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE_URL/account" -H "Authorization: Bearer $TOKEN_A")
assert_status "Soft delete A" "200" "$STATUS"

BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_B_ID/following")
assert_equals "B following empty after A deleted" "0" "$(echo "$BODY" | jq 'length')"

# Cleanup
delete_account "$TOKEN_B"

print_results
