#!/usr/bin/env bash
set -euo pipefail

baseline_path="benchmarks/prompt-baseline.json"
results_root="target/criterion"
threshold="${PRISM_BENCH_REGRESSION_THRESHOLD:-0.05}"

usage() {
  cat <<'EOF'
Usage: scripts/bench-compare.sh [--baseline FILE] [--results DIR] [--threshold FLOAT]

Compares the latest Criterion prompt benchmark results under target/criterion
against benchmarks/prompt-baseline.json. Exits non-zero if the mean or median
runtime regresses by more than the configured threshold (default 5%).
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -b|--baseline)
      baseline_path="$2"
      shift 2
      ;;
    -r|--results)
      results_root="$2"
      shift 2
      ;;
    -t|--threshold)
      threshold="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

python3 - "$baseline_path" "$results_root" "$threshold" <<'PY'
import json
import sys
import glob
from pathlib import Path

baseline_path = Path(sys.argv[1]).expanduser()
results_root = Path(sys.argv[2]).expanduser()
threshold = float(sys.argv[3])

if not baseline_path.is_file():
    print(f"[bench-compare] Baseline not found: {baseline_path}", file=sys.stderr)
    sys.exit(2)

if not results_root.exists():
    print(f"[bench-compare] Results directory not found: {results_root}", file=sys.stderr)
    sys.exit(2)

baseline = json.loads(baseline_path.read_text())
metrics = baseline.get("metrics", {})
if not metrics:
    print(f"[bench-compare] No metrics in baseline {baseline_path}", file=sys.stderr)
    sys.exit(2)

def pct_diff(current, baseline_value):
    if baseline_value == 0:
        return 0.0 if current == 0 else float("inf")
    return (current - baseline_value) / baseline_value

regressions = []
lines = []

for metric_name, metric_values in metrics.items():
    pattern = results_root / metric_name / "**" / "estimates.json"
    matches = glob.glob(str(pattern), recursive=True)
    if not matches:
        print(f"[bench-compare] Missing Criterion results for {metric_name} under {pattern}", file=sys.stderr)
        sys.exit(2)
    latest = max(matches, key=lambda path: Path(path).stat().st_mtime)
    estimates = json.loads(Path(latest).read_text())
    current_mean = estimates["mean"]["point_estimate"]
    current_median = estimates["median"]["point_estimate"]
    baseline_mean = metric_values.get("mean_ns")
    baseline_median = metric_values.get("median_ns")

    mean_diff = pct_diff(current_mean, baseline_mean)
    median_diff = pct_diff(current_median, baseline_median)
    worst_diff = max(mean_diff, median_diff, 0.0)

    status = "OK"
    if mean_diff > threshold or median_diff > threshold:
        status = "REGRESSION"
        regressions.append(metric_name)

    lines.append(
        f"{metric_name}: mean {current_mean:.2f}ns (baseline {baseline_mean:.2f}ns, {mean_diff*100:+.2f}%) | "
        f"median {current_median:.2f}ns (baseline {baseline_median:.2f}ns, {median_diff*100:+.2f}%) -> {status}"
    )

print(f"[bench-compare] Threshold: {threshold*100:.1f}%")
for line in lines:
    print(line)

if regressions:
    print(f"[bench-compare] REGRESSION detected in: {', '.join(regressions)}")
    sys.exit(1)

print("[bench-compare] All metrics within threshold.")
sys.exit(0)
PY
