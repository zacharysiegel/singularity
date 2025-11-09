#!/bin/zsh

set -euo pipefail

repo_dir=$(git rev-parse --show-toplevel)
cd "${repo_dir}"

source ./.env

DBMATE_GLOBAL_OPTIONS=(--migrations-dir './lobby/db/migrations' --wait --url "$DATABASE_URL/lobby?sslmode=disable")
dbmate "${DBMATE_GLOBAL_OPTIONS[@]}" "${@?Must provide dbmate command (and optionally command options)}"
