#!/usr/bin/env bash
set -Eeuo pipefail

# deb_deconstruct_and_study.sh
# Usage:
#   ./deb_deconstruct_and_study.sh [-o OUTDIR] [--top N] [--no-docker] [--skip-sbom] [--skip-r2] PACKAGE.deb
#
# Outputs:
#   OUTDIR/
#     _ar/       # raw ar members
#     DEBIAN/    # control files
#     rootfs/    # extracted filesystem
#     reports/   # analysis outputs + SUMMARY.md

# ---------- CLI parsing ----------
OUT=""
TOP_N=3
USE_DOCKER=1
DO_SBOM=1
DO_R2=1

die() { echo "Error: $*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || echo "WARN: '$1' not found; skipping related steps."; }

while [[ $# -gt 0 ]]; do
    case "$1" in
        -o|--out) OUT="$2"; shift 2 ;;
        --top) TOP_N="$2"; shift 2 ;;
        --no-docker) USE_DOCKER=0; shift ;;
        --skip-sbom) DO_SBOM=0; shift ;;
        --skip-r2) DO_R2=0; shift ;;
        -h|--help)
            sed -n '1,50p' "$0"; exit 0 ;;
        *) DEB="${1}"; shift ;;
    esac
done

[[ -f "${DEB:-}" ]] || die "Provide PACKAGE.deb as argument (see --help)"
OUT="${OUT:-"${DEB%.deb}_deconstructed"}"

# ---------- helpers ----------
sha256_of() {
    if command -v sha256sum >/dev/null; then sha256sum "$1" | awk '{print $1}';
    elif command -v shasum >/dev/null; then shasum -a 256 "$1" | awk '{print $1}';
    else echo "sha256 unavailable"; fi
}

on_linux() { [[ "$(uname -s)" == "Linux" ]]; }
have() { command -v "$1" >/dev/null 2>&1; }
bsdtar_extract() {
    local tarpath="$1" dest="$2"
    if have bsdtar; then
        bsdtar -xf "$tarpath" -C "$dest"
    else
        case "$tarpath" in
            *.xz)  tar -xJf "$tarpath" -C "$dest" ;;
            *.gz)  tar -xzf "$tarpath" -C "$dest" ;;
            *.zst) tar --zstd -xf "$tarpath" -C "$dest" ;;
            *)     tar -xf "$tarpath" -C "$dest" ;;
        esac
    fi
}

docker_ldd() {
    local bin="$1"
    if [[ $USE_DOCKER -eq 1 ]] && have docker; then
        docker run --rm -v "$PWD":"$PWD" -w "$PWD" debian:bookworm \
            bash -lc "ldd '$bin' || true"
    else
        echo "Docker not used/available; skipping containerized ldd"
    fi
}

# ---------- prepare ----------
mkdir -p "$OUT"/{_ar,DEBIAN,rootfs,reports}
echo "[*] Package: $DEB"
echo "[*] Output : $OUT"

# ---------- verify & extract ----------
echo "[*] sha256: $(sha256_of "$DEB")" | tee "$OUT/reports/deb.sha256.txt"
if ! ar t "$DEB" >/dev/null 2>&1; then die "Not a valid ar archive (.deb)"; fi

echo "[*] Extracting ar members..."
DEB_ABS="$(cd "$(dirname "$DEB")" && pwd)/$(basename "$DEB")"
( cd "$OUT/_ar" && ar x "$DEB_ABS" )

[[ -f "$OUT/_ar/debian-binary" ]] || die "Missing debian-binary"
CONTROL_TAR="$(ls "$OUT/_ar"/control.tar.* 2>/dev/null | head -1 || true)"
DATA_TAR="$(ls "$OUT/_ar"/data.tar.* 2>/dev/null | head -1 || true)"
[[ -n "$CONTROL_TAR" && -n "$DATA_TAR" ]] || die "Missing control.tar.* or data.tar.*"

echo "[*] Unpacking control ($CONTROL_TAR) -> DEBIAN/"
bsdtar_extract "$CONTROL_TAR" "$OUT/DEBIAN"

echo "[*] Unpacking data ($DATA_TAR) -> rootfs/"
bsdtar_extract "$DATA_TAR" "$OUT/rootfs"

# ---------- control metadata ----------
CTRL="$OUT/DEBIAN/control"
PKG="$(grep -E '^Package:' "$CTRL" 2>/dev/null | cut -d' ' -f2- || true)"
VER="$(grep -E '^Version:' "$CTRL" 2>/dev/null | cut -d' ' -f2- || true)"
ARCH="$(grep -E '^Architecture:' "$CTRL" 2>/dev/null | cut -d' ' -f2- || true)"
DESC="$(grep -E '^Description:' "$CTRL" 2>/dev/null | cut -d' ' -f2- || true)"

echo "[*] Package: ${PKG:-unknown}  Version: ${VER:-unknown}  Arch: ${ARCH:-unknown}"
echo "[*] Description: ${DESC:-n/a}"

# ---------- find ELF candidates ----------
echo "[*] Scanning for ELF executables..."
SIZE_CMD="stat -c%s"
if [[ "$(uname -s)" == "Darwin" ]]; then SIZE_CMD="stat -f%z"; fi

mapfile -t CANDIDATES < <(
    find "$OUT/rootfs" -type f -perm -111 -print0 2>/dev/null \
    | xargs -0 -I{} sh -c 'file -b "{}" | grep -q "^ELF" && echo "{}"' \
    | while read -r f; do
          sz=$($SIZE_CMD "$f" 2>/dev/null || echo 0)
          printf "%012d\t%s\n" "$sz" "$f"
      done \
    | sort -rn | cut -f2 | head -n "$TOP_N"
)

if [[ ${#CANDIDATES[@]} -eq 0 ]]; then
    echo "WARN: No executable ELF found with +x bit. Trying all files..."
    mapfile -t CANDIDATES < <(
        find "$OUT/rootfs" -type f -print0 \
        | xargs -0 -I{} sh -c 'file -b "{}" | grep -q "^ELF" && echo "{}"' \
        | while read -r f; do
              sz=$($SIZE_CMD "$f" 2>/dev/null || echo 0)
              printf "%012d\t%s\n" "$sz" "$f"
          done \
        | sort -rn | cut -f2 | head -n "$TOP_N"
    )
fi

echo "[*] Candidates:"
for b in "${CANDIDATES[@]}"; do echo "    - $b"; done

# ---------- analyze each candidate ----------
have file >/dev/null || die "'file' not found"
need readelf
need rustfilt
need strings
need rg
if [[ $DO_R2 -eq 1 ]]; then need r2; fi

for BIN in "${CANDIDATES[@]}"; do
    BN="$(basename "$BIN")"
    REP="$OUT/reports/${BN}"

    echo "[*] Analyzing $BIN"
    {
        echo "== file =="; file "$BIN"
        echo
        if have readelf; then
            echo "== readelf -h -S -l -d -n =="; readelf -h -S -l -d -n "$BIN" || true
            echo
            echo "== GLIBC versions =="; readelf --version-info "$BIN" 2>/dev/null | grep -E 'GLIBC_' | sort -u || true
            echo
            echo "== RPATH/RUNPATH =="; readelf -d "$BIN" 2>/dev/null | grep -Ei 'rpath|runpath' || true
            echo
            echo "== dynsyms (demangled) =="; (readelf -Ws "$BIN" 2>/dev/null | rustfilt || true) | head -300
            echo
        fi
        echo "== ldd =="
        if on_linux; then
            (ldd "$BIN" || echo "static or not a dynamic executable")
        else
            docker_ldd "$BIN"
        fi
        echo
        echo "== strings (Rust & deps clues) =="
        if have rg; then
            strings -a "$BIN" 2>/dev/null | rg -n 'RUST_BACKTRACE|core::|std::|rust_eh_personality|panic_fmt|tokio|hyper|reqwest|openssl|rustls|curl|sqlite|zstd|xcb|wayland|crashpad|sentry' | head -300 || true
        else
            strings -a "$BIN" 2>/dev/null | head -300 || true
        fi
    } | tee "${REP}.triage.txt"

    if [[ $DO_R2 -eq 1 ]] && have r2; then
        r2 -A -q -c "e asm.demangle=true; aaa; afl | wc -l; afl~main; izz~RUST_BACKTRACE" "$BIN" \
            > "${REP}.r2.txt" 2>&1 || true
    fi
done

# ---------- SBOM & CVE (optional) ----------
if [[ $DO_SBOM -eq 1 ]]; then
    if have syft; then syft dir:"$OUT/rootfs" -o table > "$OUT/reports/rootfs.syft.txt" || true; fi
    if have grype; then grype dir:"$OUT/rootfs" -o table > "$OUT/reports/rootfs.grype.txt" || true; fi
fi

# ---------- summary ----------
{
    echo "## Deconstruction Summary"
    echo
    echo "- Package: ${PKG:-unknown}"
    echo "- Version: ${VER:-unknown}"
    echo "- Arch: ${ARCH:-unknown}"
    echo "- SHA256: $(cat "$OUT/reports/deb.sha256.txt" | awk '{print $3}')"
    echo "- Control files: $(ls "$OUT/DEBIAN" | wc -l | tr -d ' ')"
    echo "- Rootfs files:  $(find "$OUT/rootfs" -type f | wc -l | tr -d ' ')"
    echo
    echo "### Candidate binaries (top $TOP_N by size)"
    for b in "${CANDIDATES[@]}"; do
        sz=$($SIZE_CMD "$b" 2>/dev/null || echo 0)
        echo "- $(printf "%10d" "$sz") bytes  $b"
    done
    echo
    echo "### Notes"
    echo "- No maintainer scripts were executed."
    echo "- If 'ldd' ran in Docker, results reflect the container loader; 'not found' libraries indicate external system deps."
    echo "- Rust fingerprints are inferred from strings/symbols; exact rustc version may not be embedded."
} > "$OUT/reports/SUMMARY.md"

echo
echo "[*] Done. See: $OUT/reports/"
ls -1 "$OUT/reports" || true
