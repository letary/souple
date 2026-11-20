#!/usr/bin/env bash
# Everything, on this machine, one benchmark at a time: build both harnesses, generate the
# synthetic trees, run Yoga (clang-cl, then MSVC) and Souple on the same files, write the tables.
#
#   bash crates/bench/run-all.sh            (from the workspace root, Git Bash on Windows)
#
# Environment: BENCH_SECONDS (default 0.5 per scenario), YOGA_CAPTURES (default: the lecodes-next
# checkout beside this repo), BENCH_MACHINE (default: the CPU's name).
set -euo pipefail
cd "$(dirname "$0")/../.."
OUT=crates/bench/out
CAPTURES=${YOGA_CAPTURES:-../lecodes-next/engines/ui/libs/yoga/benchmark/captures}
S=${BENCH_SECONDS:-0.5}
if [ -z "${BENCH_MACHINE:-}" ]; then
  BENCH_MACHINE="$(powershell.exe -NoProfile -Command '(Get-CimInstance Win32_Processor).Name' | tr -d '\r'), Windows $(powershell.exe -NoProfile -Command '[Environment]::OSVersion.Version.ToString()' | tr -d '\r')"
fi
export BENCH_MACHINE

cargo build -p souple-bench --release
cmd.exe //c "crates\\bench\\yoga\\build.bat"
mkdir -p "$OUT/rects"
./target/release/souple-bench gen "$OUT/trees"

./crates/bench/yoga-build/clang/yoga-bench --seconds "$S" --out "$OUT/yoga.json" --rects "$OUT/rects" "$CAPTURES" "$OUT/trees"
./crates/bench/yoga-build/msvc/Release/yoga-bench --seconds "$S" --out "$OUT/yoga-msvc.json" "$CAPTURES" "$OUT/trees"
./target/release/souple-bench run --seconds "$S" --out "$OUT/souple.json" --rects "$OUT/rects" "$CAPTURES" "$OUT/trees"

./target/release/souple-bench report "$OUT/souple.json" "$OUT/yoga.json" --rects "$OUT/rects" --md "$OUT/results.md"
./target/release/souple-bench report "$OUT/souple.json" "$OUT/yoga-msvc.json" --md "$OUT/results-msvc.md"
echo "wrote $OUT/results.md and $OUT/results-msvc.md"
