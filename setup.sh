#!/bin/zsh

set -eo pipefail

echo "Initializing submodules"
git submodule update --init

