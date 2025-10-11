#!/usr/bin/env bash
# file_db_time_trial.sh — Git+File vs Postgres write benchmark (10KB JSON)
# Usage:
#   chmod +x file_db_time_trial.sh
#   PGURL="postgres://postgres:postgres@localhost:5432/postgres" N=10000 ./file_db_time_trial.sh
# Tunables (env):
#   BENCH_ROOT="$HOME/vpr_bench"  N=10000  PGURL="postgres://user:pass@host:5432/db"

set -euo pipefail

# --------------------------
# Config
# --------------------------
BENCH_ROOT="${BENCH_ROOT:-$HOME/vpr_bench}"
N=1000
PGURL="${PGURL:-postgres://postgres:postgres@localhost:5432/postgres}"

# --------------------------
# Helpers
# --------------------------
banner(){ echo; echo "=== $* ==="; }
ts_utc(){ date -u +"%Y-%m-%dT%H:%M:%SZ"; }
need(){ command -v "$1" >/dev/null 2>&1; }
psql_ok(){
  if ! need psql; then return 1; fi
  # Extract password if present; ignore if not
  PGPASSWORD="$(echo "$PGURL" | sed -n 's#.*postgres://[^:]*:\([^@]*\)@.*#\1#p')" \
  psql "$PGURL" -q -c "SELECT 1;" >/dev/null 2>&1
}

# Millisecond stopwatch (portable)
now_ms(){
  python3 - <<'PY'
import time; print(int(time.time()*1000))
PY
}

# Fsync a file path (portable)
fsync_file(){
  python3 - "$1" <<'PY'
import os, sys
p=sys.argv[1]
fd=os.open(p, os.O_RDONLY)
os.fsync(fd)
os.close(fd)
PY
}

ops_line(){
  local label="$1" start_ms="$2" end_ms="$3" n="$4"
  local dur=$((end_ms - start_ms))
  # Ops/sec with awk to keep decimals
  local ops=$(awk -v N="$n" -v MS="$dur" 'BEGIN{printf "%.2f", (N*1000.0)/MS}')
  printf "%s: %d ops in %d ms -> %s ops/sec\n" "$label" "$n" "$dur" "$ops"
}

# --------------------------
# Setup: dirs, repo, payloads
# --------------------------
banner "Preparing directories"
rm -rf "$BENCH_ROOT"
mkdir -p "$BENCH_ROOT/gitrepo" "$BENCH_ROOT/payloads"

banner "Init Git repo (auto-GC disabled, local identity)"
pushd "$BENCH_ROOT/gitrepo" >/dev/null
git init >/dev/null
git config gc.auto 0
git config maintenance.auto false
git config user.name "bench"
git config user.email "bench@example.com"
popd >/dev/null

banner "Generating $N JSON payloads (~10KB each)"
python3 - <<PY
import json, os, time
root = os.environ["BENCH_ROOT"] + "/payloads"
os.makedirs(root, exist_ok=True)
blob = "x"*9950
N = int(os.environ.get("N","10000"))
for i in range(N):
    key=f"{i:06d}"
    p=f"{root}/{key}.json"
    if os.path.exists(p): continue
    doc = {"uid": f"ulid_{key}", "created_at": time.time(),
           "data": blob, "meta": {"template_id":"quill.clinic_letter.v1","signed": False}}
    with open(p,"w") as f: json.dump(doc,f,separators=(",",":"))
PY

# --------------------------
# Bench functions (self-contained; no sub-scripts)
# --------------------------
run_git_per_commit(){
  banner "Git per-commit (1 commit per write)"
  cd "$BENCH_ROOT/gitrepo"
  git reset -q --hard; git clean -fdx
  mkdir -p records
  local start end
  start=$(now_ms)
  local i=0
  while [ $i -lt $N ]; do
    local key pid pa ti dir tmp final
    key=$(printf "%06d" "$i")
    pid="patient-$(printf %06d $((i % 2000)))"
    pa="${pid:0:2}"; ti="${pid:2:2}"
    dir="records/patients/${pa}/${ti}/${pid}/compositions/letters/uid-${key}"
    mkdir -p "$dir"
    tmp="$dir/00000001.json.tmp"
    final="$dir/00000001.json"
    cp "$BENCH_ROOT/payloads/$key.json" "$tmp"
    fsync_file "$tmp"
    mv "$tmp" "$final"
    git add -A "$dir"
    TS="$(ts_utc)"
    GIT_COMMITTER_DATE="$TS" GIT_AUTHOR_DATE="$TS" git commit -q -m "write $key"
    i=$((i+1))
  done
  end=$(now_ms)
  ops_line "git_per_commit" "$start" "$end" "$N"
}

run_git_batch10(){
  banner "Git batch-of-10 (10 writes per commit)"
  cd "$BENCH_ROOT/gitrepo"
  git reset -q --hard; git clean -fdx
  mkdir -p records
  local start end c=0 i=0
  start=$(now_ms)
  while [ $i -lt $N ]; do
    local key pid pa ti dir tmp final
    key=$(printf "%06d" "$i")
    pid="patient-$(printf %06d $((i % 2000)))"
    pa="${pid:0:2}"; ti="${pid:2:2}"
    dir="records/patients/${pa}/${ti}/${pid}/compositions/letters/uid-${key}"
    mkdir -p "$dir"
    tmp="$dir/00000001.json.tmp"
    final="$dir/00000001.json"
    cp "$BENCH_ROOT/payloads/$key.json" "$tmp"
    fsync_file "$tmp"
    mv "$tmp" "$final"
    git add -A "$dir"
    c=$((c+1))
    if [ $((c % 10)) -eq 0 ]; then
      TS="$(ts_utc)"; GIT_COMMITTER_DATE="$TS" GIT_AUTHOR_DATE="$TS" git commit -q -m "batch commit $c"
    fi
    i=$((i+1))
  done
  if [ $((c % 10)) -ne 0 ]; then
    TS="$(ts_utc)"; GIT_COMMITTER_DATE="$TS" GIT_AUTHOR_DATE="$TS" git commit -q -m "final batch"
  fi
  end=$(now_ms)
  ops_line "git_batch10" "$start" "$end" "$N"
}

prepare_pg(){
  banner "Preparing Postgres table (durable, simple indexes)"
  psql "$PGURL" -q <<'SQL'
DROP TABLE IF EXISTS vpr_records;
CREATE TABLE vpr_records (
  id          bigserial primary key,
  patient_id  text not null,
  kind        text not null,
  uid         text not null,
  version     int  not null,
  created_at  timestamptz not null default now(),
  doc         jsonb not null
);
CREATE INDEX ON vpr_records (patient_id, created_at desc);
CREATE INDEX ON vpr_records (kind, created_at desc);
SQL
}

run_pg_per_tx(){
  banner "Postgres per-transaction (1 insert per tx)"
  local start end i=0
  start=$(now_ms)
  while [ $i -lt $N ]; do
    local key pid uid DOC
    key=$(printf "%06d" "$i")
    pid="patient-$(printf %06d $((i % 2000)))"
    uid="uid-$key"
    DOC=$(cat "$BENCH_ROOT/payloads/$key.json")
    PGPASSWORD="$(echo "$PGURL" | sed -n 's#.*postgres://[^:]*:\([^@]*\)@.*#\1#p')" \
    psql "$PGURL" -q -c \
      "INSERT INTO vpr_records (patient_id,kind,uid,version,doc)
       VALUES ('${pid}','letters','${uid}',1,'${DOC}'::jsonb);"
    i=$((i+1))
  done
  end=$(now_ms)
  ops_line "pg_per_tx" "$start" "$end" "$N"
}

run_pg_batch10(){
  banner "Postgres batch-of-10 (10 inserts per tx)"
  local start end i=0
  start=$(now_ms)
  while [ $i -lt $N ]; do
    local SQL="BEGIN;"
    for j in $(seq 1 10); do
      [ $i -ge $N ] && break
      local key pid uid DOC
      key=$(printf "%06d" "$i")
      pid="patient-$(printf %06d $((i % 2000)))"
      uid="uid-$key"
      DOC=$(cat "$BENCH_ROOT/payloads/$key.json")
      SQL+="INSERT INTO vpr_records (patient_id,kind,uid,version,doc)
            VALUES ('${pid}','letters','${uid}',1,'${DOC}'::jsonb);"
      i=$((i+1))
    done
    SQL+="COMMIT;"
    PGPASSWORD="$(echo "$PGURL" | sed -n 's#.*postgres://[^:]*:\([^@]*\)@.*#\1#p')" \
    psql "$PGURL" -q -c "$SQL"
  done
  end=$(now_ms)
  ops_line "pg_batch10" "$start" "$end" "$N"
}

# --------------------------
# Run
# --------------------------
# banner "Running Git benchmarks (N=$N)"
# run_git_per_commit
# run_git_batch10

if psql_ok; then
  prepare_pg
  banner "Running Postgres benchmarks (N=$N)"
  run_pg_per_tx
  run_pg_batch10
else
  banner "Postgres not reachable — skipping DB benchmarks"
fi

banner "Done."
echo "Tip: tweak N or change batch size (10→50) in functions to compare effects."
