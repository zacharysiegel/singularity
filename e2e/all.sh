#!/bin/zsh
set -euo pipefail

SCRIPT_DIR="$(dirname "$0")"
TEST_DIR="$SCRIPT_DIR/test"

echo "Running all e2e tests..."
echo ""

for test_script in "$TEST_DIR"/*.sh; do
    echo ">>> $(basename "$test_script")"
    "$test_script"
    echo ""
done

echo "=== All e2e tests complete ==="
