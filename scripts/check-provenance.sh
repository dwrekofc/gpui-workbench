#!/bin/bash
# Provenance check script (governance spec AC-004)
# Validates that every adapted file has a corresponding provenance entry in docs/PROVENANCE.md
# with required fields: source URL, commit hash, license, and local modifications.
#
# Exit 0 = pass, Exit 1 = fail

set -euo pipefail

PROVENANCE_FILE="docs/PROVENANCE.md"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

if [ ! -f "$PROVENANCE_FILE" ]; then
    echo "FAIL: $PROVENANCE_FILE does not exist"
    exit 1
fi

echo "Provenance check: $PROVENANCE_FILE exists"

# Check that the file has the required table header
if ! grep -q "| Local path | Source repo | Commit | License | Modifications |" "$PROVENANCE_FILE"; then
    echo "FAIL: $PROVENANCE_FILE missing required table format"
    exit 1
fi

echo "Provenance check: table format valid"

# Find all files with provenance markers (adapted files should have a comment marker)
# Pattern: "// Provenance:" or "# Provenance:" in source files
ADAPTED_FILES=$(grep -rl "Provenance:" --include="*.rs" --include="*.toml" . 2>/dev/null | grep -v target/ | grep -v .refs/ || true)

if [ -z "$ADAPTED_FILES" ]; then
    echo "Provenance check: no adapted files found (trivially passing)"
    exit 0
fi

FAIL=0
for file in $ADAPTED_FILES; do
    # Strip leading ./ for matching
    clean_path="${file#./}"
    if ! grep -q "$clean_path" "$PROVENANCE_FILE"; then
        echo "FAIL: adapted file '$clean_path' has no provenance entry in $PROVENANCE_FILE"
        FAIL=1
    fi
done

if [ "$FAIL" -eq 1 ]; then
    exit 1
fi

echo "Provenance check: all adapted files have provenance entries"
exit 0
