#!/usr/bin/env bash
# Usage: ./scripts/create_patient.sh "First" "Last"
set -euo pipefail

FIRST=${1:-}
LAST=${2:-}

if [ -z "$FIRST" ] || [ -z "$LAST" ]; then
  echo "Usage: $0 FIRST LAST"
  exit 2
fi

DATA=$(printf '{"firstName": "%s", "lastName": "%s"}' "$FIRST" "$LAST")

grpcurl -plaintext \
  -import-path ../crates/api/proto \
  -proto ../crates/api/proto/vpr/v1/vpr.proto \
  -d "$DATA" \
  localhost:50051 vpr.v1.VPR/CreatePatient

