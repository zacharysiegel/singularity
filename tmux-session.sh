#!/bin/sh

SESSION="singularity"
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

tmux kill-session -t "$SESSION" 2>/dev/null
# -x/-y: match the new session's dimensions to the current terminal to avoid resize flicker on attach
tmux new-session -d -s "$SESSION" -c "$PROJECT_DIR" -x "$(tput cols)" -y "$(tput lines)"

tmux split-window -h -l 14% -c "$PROJECT_DIR"

tmux select-pane -t 0
tmux split-window -h -l 45% -c "$PROJECT_DIR"

tmux select-pane -t 2
tmux split-window -v -l 67% -c "$PROJECT_DIR"

tmux select-pane -t 3
tmux split-window -v -l 50% -c "$PROJECT_DIR"

tmux send-keys -t "$SESSION:0.2" "cargo run -p client"
tmux send-keys -t "$SESSION:0.3" "cargo run -p live"
tmux send-keys -t "$SESSION:0.4" "cargo run -p lobby"

tmux select-pane -t 1
tmux attach-session -t "$SESSION"

