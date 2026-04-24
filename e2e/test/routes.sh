#!/bin/zsh
set -uo pipefail
source "$(dirname "$0")/../harness.sh"

echo "=== Route Scoping E2E Tests ==="

# Setup
ACCOUNT=$(create_account "e2e_routes@test.com" "e2e_routes" "pass123")
ACCOUNT_ID=$(echo "$ACCOUNT" | jq -r '.id')
TOKEN=$(login "e2e_routes@test.com" "pass123")

GAME=$(curl -s -X POST "$BASE_URL/game" -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' -d '{"name":"E2E Route Test"}')
GAME_ID=$(echo "$GAME" | jq -r '.id')

# Health
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/health")
assert_status "GET /health" "200" "$STATUS"

# Account endpoints
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/account" -H "Authorization: Bearer $TOKEN")
assert_status "GET /account (own)" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/account/$ACCOUNT_ID")
assert_status "GET /account/{id} (public)" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/account/$ACCOUNT_ID/accolades")
assert_status "GET /account/{id}/accolades" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/account/$ACCOUNT_ID/statistics")
assert_status "GET /account/{id}/statistics" "200" "$STATUS"

# Game endpoints
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/game")
assert_status "GET /game (browser)" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/game/$GAME_ID")
assert_status "GET /game/{id}" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/game/$GAME_ID/accolades")
assert_status "GET /game/{id}/accolades" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/game/$GAME_ID/statistics")
assert_status "GET /game/{id}/statistics" "200" "$STATUS"

# Game member (already joined via create)
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/game/$GAME_ID/member" -H "Authorization: Bearer $TOKEN")
assert_status "POST /game/{id}/member (already joined)" "409" "$STATUS"

# Game enter/exit
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/game/$GAME_ID/enter" -H "Authorization: Bearer $TOKEN")
assert_status "POST /game/{id}/enter" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/game/$GAME_ID/exit" -H "Authorization: Bearer $TOKEN")
assert_status "POST /game/{id}/exit" "200" "$STATUS"

# Cleanup
delete_account "$TOKEN"

print_results
