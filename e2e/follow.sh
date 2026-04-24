#!/bin/zsh
set -euo pipefail

BASE_URL="http://localhost:10000"
PASS=0
FAIL=0

function assert_status {
    local test_name="$1"
    local expected="$2"
    local actual="$3"
    if [ "$actual" = "$expected" ]; then
        echo "  PASS: $test_name (HTTP $actual)"
        PASS=$((PASS + 1))
    else
        echo "  FAIL: $test_name (expected HTTP $expected, got HTTP $actual)"
        FAIL=$((FAIL + 1))
    fi
}

echo "=== Follow E2E Tests ==="

# Setup
ACCOUNT_A=$(curl -s -X POST "$BASE_URL/account" -H 'Content-Type: application/json' -d '{"email":"e2e_follow_a@test.com","username":"e2e_follow_a","password":"pass123"}')
ACCOUNT_A_ID=$(echo "$ACCOUNT_A" | jq -r '.id')
ACCOUNT_B=$(curl -s -X POST "$BASE_URL/account" -H 'Content-Type: application/json' -d '{"email":"e2e_follow_b@test.com","username":"e2e_follow_b","password":"pass123"}')
ACCOUNT_B_ID=$(echo "$ACCOUNT_B" | jq -r '.id')
TOKEN_A=$(curl -s -X POST "$BASE_URL/session" -H 'Content-Type: application/json' -d '{"email":"e2e_follow_a@test.com","password":"pass123"}' | jq -r '.token')
TOKEN_B=$(curl -s -X POST "$BASE_URL/session" -H 'Content-Type: application/json' -d '{"email":"e2e_follow_b@test.com","password":"pass123"}' | jq -r '.token')

# Test 1: A follows B
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/account/$ACCOUNT_B_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "A follows B" "200" "$STATUS"

# Test 2: B follows A
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/account/$ACCOUNT_A_ID/follow" -H "Authorization: Bearer $TOKEN_B")
assert_status "B follows A" "200" "$STATUS"

# Test 3: Mutuals
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_A_ID/mutuals")
MUTUAL_COUNT=$(echo "$BODY" | jq 'length')
IS_MUTUAL=$(echo "$BODY" | jq -r '.[0].is_mutual')
assert_status "Mutuals returns B" "1" "$MUTUAL_COUNT"
assert_status "is_mutual is true" "true" "$IS_MUTUAL"

# Test 4: Followers
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_A_ID/followers")
FOLLOWER_COUNT=$(echo "$BODY" | jq 'length')
assert_status "A has 1 follower" "1" "$FOLLOWER_COUNT"

# Test 5: Following
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_A_ID/following")
FOLLOWING_COUNT=$(echo "$BODY" | jq 'length')
assert_status "A is following 1" "1" "$FOLLOWING_COUNT"

# Test 6: Duplicate follow
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/account/$ACCOUNT_B_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "Duplicate follow returns 409" "409" "$STATUS"

# Test 7: Self-follow
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/account/$ACCOUNT_A_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "Self-follow returns 500" "500" "$STATUS"

# Test 8: Unfollow
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE_URL/account/$ACCOUNT_B_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "A unfollows B" "200" "$STATUS"

# Test 9: Unfollow non-existent
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE_URL/account/$ACCOUNT_B_ID/follow" -H "Authorization: Bearer $TOKEN_A")
assert_status "Unfollow non-existent returns 404" "404" "$STATUS"

# Test 10: Mutuals after unfollow
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_A_ID/mutuals")
MUTUAL_COUNT=$(echo "$BODY" | jq 'length')
assert_status "Mutuals empty after unfollow" "0" "$MUTUAL_COUNT"

# Test 11: B's followers after A unfollow
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_B_ID/followers")
FOLLOWER_COUNT=$(echo "$BODY" | jq 'length')
assert_status "B has 0 followers after A unfollow" "0" "$FOLLOWER_COUNT"

# Test 12: Account public profile still works
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/account/$ACCOUNT_A_ID")
assert_status "Account public profile works" "200" "$STATUS"

# Test 13: B re-follows A, then soft delete A
curl -s -o /dev/null -X POST "$BASE_URL/account/$ACCOUNT_A_ID/follow" -H "Authorization: Bearer $TOKEN_B"
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE_URL/account" -H "Authorization: Bearer $TOKEN_A")
assert_status "Soft delete A" "200" "$STATUS"

# Test 14: B's following after A deleted
BODY=$(curl -s "$BASE_URL/account/$ACCOUNT_B_ID/following")
FOLLOWING_COUNT=$(echo "$BODY" | jq 'length')
assert_status "B following empty after A deleted" "0" "$FOLLOWING_COUNT"

# Cleanup: delete remaining account B
curl -s -o /dev/null -X DELETE "$BASE_URL/account" -H "Authorization: Bearer $TOKEN_B"

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
