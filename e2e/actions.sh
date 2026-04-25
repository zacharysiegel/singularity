#!/bin/zsh

function create_account {
    local email="$1"
    local username="$2"
    local password="$3"
    curl -s -X POST "$BASE_URL/account" \
        -H 'Content-Type: application/json' \
        -d "$(cat <<EOF
{"email":"$email","username":"$username","password":"$password"}
EOF
)"
}

function login {
    local email="$1"
    local password="$2"
    curl -s -X POST "$BASE_URL/session" \
        -H 'Content-Type: application/json' \
        -d "$(cat <<EOF
{"email":"$email","password":"$password"}
EOF
)" | jq -r '.token'
}

function delete_account {
    local token="$1"
    curl -s -o /dev/null -X DELETE "$BASE_URL/account" -H "Authorization: Bearer $token"
}

# Send a message over a WebSocket connection and return the first response.
# Uses a FIFO to keep the connection alive during async server processing.
# Usage: RESPONSE=$(ws_send <path> <token> <message>)
function ws_send {
    local ws_path="$1"
    local token="$2"
    local message="$3"

    local fifo=$(mktemp -u /tmp/e2e_ws_fifo.XXXXXX)
    mkfifo "$fifo"
    local outfile=$(mktemp /tmp/e2e_ws_out.XXXXXX)

    websocat "ws://localhost:10000${ws_path}" -H "Authorization: Bearer $token" < "$fifo" > "$outfile" 2>/dev/null &
    local ws_pid=$!

    { echo "$message"; sleep 0.5; } > "$fifo"

    kill $ws_pid 2>/dev/null; wait $ws_pid 2>/dev/null
    head -1 "$outfile"
    rm -f "$fifo" "$outfile"
}

# Open a persistent WebSocket listener that collects all received messages to a file.
# Returns immediately. Use ws_listener_collect to close and read the output.
# Usage: ws_listener_open <path> <token> <state_dir_var>
function ws_listener_open {
    local ws_path="$1"
    local token="$2"
    local state_dir_var="$3"

    local state_dir=$(mktemp -d /tmp/e2e_ws_listener.XXXXXX)
    mkfifo "$state_dir/fifo"

    websocat "ws://localhost:10000${ws_path}" -H "Authorization: Bearer $token" < "$state_dir/fifo" > "$state_dir/out" 2>/dev/null &
    echo $! > "$state_dir/pid"

    eval "exec 8>$state_dir/fifo"
    sleep 0.3

    eval "$state_dir_var='$state_dir'"
}

# Close a WebSocket listener and print collected output.
# Usage: OUTPUT=$(ws_listener_collect <state_dir>)
function ws_listener_collect {
    local state_dir="$1"
    local pid=$(cat "$state_dir/pid")

    eval "exec 8>&-"
    kill $pid 2>/dev/null; wait $pid 2>/dev/null
    cat "$state_dir/out"
    rm -rf "$state_dir"
}

