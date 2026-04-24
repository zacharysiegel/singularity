#!/bin/zsh

function create_account {
    local email="$1"
    local username="$2"
    local password="$3"
    curl -s -X POST "$BASE_URL/account" -H 'Content-Type: application/json' -d "{\"email\":\"$email\",\"username\":\"$username\",\"password\":\"$password\"}"
}

function login {
    local email="$1"
    local password="$2"
    curl -s -X POST "$BASE_URL/session" -H 'Content-Type: application/json' -d "{\"email\":\"$email\",\"password\":\"$password\"}" | jq -r '.token'
}

function delete_account {
    local token="$1"
    curl -s -o /dev/null -X DELETE "$BASE_URL/account" -H "Authorization: Bearer $token"
}

