#!/bin/bash
echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                    Ovie Programming Language                 ║"
echo "║                        Stage 2 - Self-Hosted                ║"
echo "║                           Version 2.1.0                     ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Installing Ovie Programming Language for Linux x64..."
mkdir -p ~/.local/bin
cp ovie ~/.local/bin/
cp oviec ~/.local/bin/
chmod +x ~/.local/bin/ovie ~/.local/bin/oviec
echo "✅ Ovie installed successfully!"
echo ""
echo "Make sure ~/.local/bin is in your PATH"
echo "Add this to your ~/.bashrc or ~/.zshrc:"
echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "Run 'ovie --help' to get started."
echo "Visit: https://ovie-lang.org for documentation"