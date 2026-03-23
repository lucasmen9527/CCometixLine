#!/bin/bash
set -e

REPO="https://github.com/lucasmen9527/CCometixLine.git"
INSTALL_DIR="$HOME/.claude/ccline"
TMP_DIR=$(mktemp -d)

echo "==> Cloning CCometixLine..."
git clone --depth 1 "$REPO" "$TMP_DIR/CCometixLine"

echo "==> Building (release mode)..."
cd "$TMP_DIR/CCometixLine"
cargo build --release

echo "==> Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
cp target/release/ccometixline "$INSTALL_DIR/ccline"
chmod +x "$INSTALL_DIR/ccline"

# Copy default config if not exists
if [ ! -f "$INSTALL_DIR/config.toml" ]; then
  "$INSTALL_DIR/ccline" --help > /dev/null 2>&1 || true
  echo "==> Default config initialized"
fi

echo "==> Cleaning up..."
rm -rf "$TMP_DIR"

echo ""
echo "Done! ccline installed to: $INSTALL_DIR/ccline"
echo ""
echo "Add to your Claude Code settings.json:"
echo '  {'
echo '    "statusLine": {'
echo '      "type": "command",'
echo '      "command": "~/.claude/ccline/ccline",'
echo '      "padding": 0'
echo '    }'
echo '  }'
echo ""
echo "Run 'ccline -c' to configure themes and segments."
