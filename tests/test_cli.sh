#!/bin/bash

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

test_command() {
    echo "Testing: $1"
    eval "$1"
    exit_status=$?

    if [ $exit_status -eq 0 ]; then
        echo -e "${GREEN}✓ Success${NC}"
    else
        echo -e "${RED}✗ Failed with exit code: $exit_status${NC}"
        exit 1
    fi
    echo
}

# entry points
test_command "uv run pyrust num-cpus"
test_command "uv run pyrust estimate-pi -n 100 -t 2"
test_command "uv run python -m pyrust greet"

echo -e "${GREEN}All tests passed!${NC}"
