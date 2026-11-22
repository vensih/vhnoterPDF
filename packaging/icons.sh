#!/usr/bin/env bash
# Regenerate the app icons from assets/app-icon/v-mark.svg and the Windows PDF file icon from
# assets/app-icon/v-file.svg. Both are master vectors (256x256 viewBox, no live text).
#
# Needs: resvg (cargo install resvg) and python3 (stdlib only, for the .ico and .icns). The
# outputs are committed, so building and packaging never need these tools. After running it,
# update the sha256 values in ATTRIBUTION.toml, then `cargo xtask assets --write`.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
MARK="$DIR/v-mark.svg"
FILE="$DIR/v-file.svg"
ID="ai.storyteller.pdfcraft"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v resvg >/dev/null || { echo "error: resvg not found (cargo install resvg)" >&2; exit 1; }

render() {
  local src="$1" size="$2" out="$3"
  resvg -w "$size" -h "$size" "$src" "$out" </dev/null
}

# macOS icons follow Apple's grid: an 824 px body centred on a transparent 1024 canvas. The mark
# is nested inline (resvg doesn't load SVG files through <image>).
MAC="$TMP/macos.svg"
python3 - "$MARK" "$MAC" <<'PY'
import sys
text = open(sys.argv[1], encoding="utf-8").read()
inner = text[text.index(">") + 1 : text.rindex("</svg>")]
open(sys.argv[2], "w", encoding="utf-8").write(
    '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">'
    f'<svg x="100" y="100" width="824" height="824" viewBox="0 0 256 256">{inner}</svg></svg>')
PY

# The document icon's 16, 24 and 32 px frames darken the page outline (bronze #7A5C2E), so the
# page edge still shows on a white background. Larger frames keep the master as drawn.
SMALL_FILE="$TMP/v-file-small.svg"
sed 's/stroke="#E4DCC8" stroke-width="2"/stroke="#7A5C2E" stroke-width="12"/' "$FILE" >"$SMALL_FILE"

# 1024 px PNG on Apple's grid (also the runtime Dock icon on macOS, see apps/pdfcraft/src/main.rs).
render "$MAC" 1024 "$DIR/pdfcraft-1024.png"

# Linux hicolor theme (hicolor/256x256 is also the runtime icon on Windows and Linux).
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$MARK" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
cp "$MARK" "$DIR/hicolor/scalable/apps/$ID.svg"

# Windows .ico: PNG-compressed entries, 16-256 px. The document icon has its own resource
# in vhnoterpdf.exe; the app icon remains the first/default icon.
write_ico() {
  python3 - "$@" <<'PY'
import struct, sys
out, pngs = sys.argv[1], sys.argv[2:]
blobs = [open(p, "rb").read() for p in pngs]
head = struct.pack("<HHH", 0, 1, len(blobs))
entries, data, offset = b"", b"", 6 + 16 * len(blobs)
for b in blobs:
    w, h = struct.unpack(">II", b[16:24])  # IHDR
    entries += struct.pack("<BBBBHHII", w % 256, h % 256, 0, 0, 1, 32, len(b), offset)
    data += b
    offset += len(b)
open(out, "wb").write(head + entries + data)
PY
}
APP_PNGS=() DOC_PNGS=()
for s in 16 24 32 48 64 128 256; do
  render "$MARK" "$s" "$TMP/app-$s.png"
  APP_PNGS+=("$TMP/app-$s.png")
  if [ "$s" -le 32 ]; then src="$SMALL_FILE"; else src="$FILE"; fi
  render "$src" "$s" "$TMP/doc-$s.png"
  DOC_PNGS+=("$TMP/doc-$s.png")
done
write_ico "$DIR/pdfcraft.ico" "${APP_PNGS[@]}"
write_ico "$DIR/pdfcraft-document.ico" "${DOC_PNGS[@]}"

# macOS .icns with PNG entries (16-1024 px, 1x and 2x), written directly: no iconutil needed.
ICNS_ARGS=()
for pair in icp4:16 icp5:32 icp6:64 ic07:128 ic08:256 ic09:512 ic10:1024 ic11:32 ic12:64 ic13:256 ic14:512; do
  kind="${pair%%:*}" size="${pair##*:}"
  render "$MAC" "$size" "$TMP/mac-$kind.png"
  ICNS_ARGS+=("$kind" "$TMP/mac-$kind.png")
done
python3 - "$DIR/pdfcraft.icns" "${ICNS_ARGS[@]}" <<'PY'
import struct, sys
out, rest = sys.argv[1], sys.argv[2:]
body = b""
for kind, path in zip(rest[::2], rest[1::2]):
    data = open(path, "rb").read()
    body += kind.encode() + struct.pack(">I", 8 + len(data)) + data
open(out, "wb").write(b"icns" + struct.pack(">I", 8 + len(body)) + body)
PY
echo "icons written to $DIR"
