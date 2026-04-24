#!/bin/zsh
set -uo pipefail

SCRIPT_DIR="$(dirname "$0")"
TEST_DIR="$SCRIPT_DIR/test"
OUTPUT_FILE=$(mktemp /tmp/e2e_output.XXXXXX)

echo "Running all e2e tests..."
echo ""

SUITE_EXIT=0
for test_script in "$TEST_DIR"/*.sh; do
    echo ">>> $(basename "$test_script")"
    "$test_script" 2>&1 | tee -a "$OUTPUT_FILE"
    if [ "${pipestatus[1]}" -ne 0 ]; then
        SUITE_EXIT=1
    fi
    echo ""
done

TOTAL_PASS=$(grep -c "^  PASS:" "$OUTPUT_FILE" || true)
TOTAL_FAIL=$(grep -c "^  FAIL:" "$OUTPUT_FILE" || true)

rm -f "$OUTPUT_FILE"

echo "=== Suite total: $TOTAL_PASS passed, $TOTAL_FAIL failed ==="
exit $SUITE_EXIT
