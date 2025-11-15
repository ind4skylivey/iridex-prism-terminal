#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: scripts/release.sh <new-version>"
  exit 1
fi

version="$1"
release_date="$(date -u +%Y-%m-%d)"

run() {
  echo "+ $*"
  "$@"
}

echo "==> Running quality gates"
run cargo fmt --all -- --check
run cargo clippy --all-targets --all-features -- -D warnings
run cargo test --all-targets --all-features
run cargo bench --bench prompt -- --sample-size=50 --measurement-time=3
run scripts/bench-compare.sh --threshold 0.05

echo "==> Bumping crate version to $version"
python3 - "$version" <<'PY'
import pathlib, re, sys
version = sys.argv[1]
cargo_toml = pathlib.Path("Cargo.toml")
text = cargo_toml.read_text()
new_text, count = re.subn(r'version\s*=\s*"[A-Za-z0-9\.\-\+]+"', f'version = "{version}"', text, count=1)
if count != 1:
    raise SystemExit("failed to update Cargo.toml version")
cargo_toml.write_text(new_text)

lock_path = pathlib.Path("Cargo.lock")
if lock_path.exists():
    lines = lock_path.read_text().splitlines()
    for idx, line in enumerate(lines):
        if line.strip() == 'name = "prism"':
            for j in range(idx + 1, len(lines)):
                if lines[j].strip().startswith("version ="):
                    lines[j] = f'version = "{version}"'
                    break
            break
    if lines:
        lock_path.write_text("\n".join(lines) + "\n")
PY

echo "==> Updating Cargo.lock metadata"
run cargo metadata --format-version 1 --no-deps

echo "==> Updating CHANGELOG.md"
python3 - "$version" "$release_date" <<'PY'
import pathlib, sys, textwrap
version, release_date = sys.argv[1:3]
path = pathlib.Path("CHANGELOG.md")
if path.exists():
    contents = path.read_text()
else:
    contents = "# Changelog\n\nAll notable changes will be documented here.\n\n"

entry = textwrap.dedent(f"""
## {version} - {release_date}

- Validated fmt/clippy/tests plus prompt benchmarks per Sprint 6.3 gates.
- Locked in prompt benchmark baselines and regression guard via scripts/bench-compare.sh.
- Ensured CLI/theme-loader/sync harness coverage remains green before packaging.
""").lstrip("\n")

if f"## {version} - " in contents:
    print(f"CHANGELOG already contains version {version}, skipping append.")
else:
    if contents.endswith("\n") is False:
        contents += "\n"
    contents = contents + entry + "\n"
    path.write_text(contents)
PY

echo "==> Building release artifacts"
run cargo build --release
dist_dir="dist/${version}"
mkdir -p "$dist_dir"
artifact="${dist_dir}/prism-${version}-x86_64-unknown-linux-gnu.tar.gz"
tar czf "$artifact" -C target/release prism

if command -v gpg >/dev/null 2>&1; then
  echo "==> Signing ${artifact}"
  gpg --armor --detach-sign "$artifact"
else
  echo "!! gpg not found; skipping signature (expected for local dry runs)" >&2
fi

echo "==> Release checklist"
cat <<EOF
1. Review git diff (version + CHANGELOG.md + dist artifacts).
2. Commit with message: [RELEASE] Prepare ${version}
3. Tag release: git tag -s ${version} -m "prism ${version}"
4. Push commits + tag, then attach dist artifacts to the GitHub release.
EOF
