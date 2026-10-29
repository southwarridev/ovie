import os, re

BASE = r"c:\Users\Home PC\Desktop\ovie"
YEAR = "2026"

# ── 1. Rewrite LICENSE ────────────────────────────────────────────────────────
mit = f"""MIT License

Copyright (c) {YEAR} Nashedy Ltd

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""
with open(os.path.join(BASE, "LICENSE"), "w", encoding="utf-8") as f:
    f.write(mit)
print("LICENSE updated")

# ── 2. Text replacements across all text files ───────────────────────────────
REPLACEMENTS = [
    # Owner
    ("Ovie Language Team",        "Nashedy Ltd"),
    ("ovie language team",        "Nashedy Ltd"),
    ("Ovie Team",                 "Nashedy Ltd"),
    # License badge/text
    ("Apache License, Version 2.0", "MIT License"),
    ("Licensed under the Apache License, Version 2.0", "Licensed under the MIT License"),
    ("http://www.apache.org/licenses/LICENSE-2.0", "https://opensource.org/licenses/MIT"),
    ("the Apache License",        "the MIT License"),
    ("Apache-2.0",                "MIT"),
    # Copyright lines
    ("Copyright 2024-2026 Ovie Language Team", f"Copyright (c) {YEAR} Nashedy Ltd"),
    ("Copyright (c) 2026 Ovie Language Team",  f"Copyright (c) {YEAR} Nashedy Ltd"),
    ("Copyright 2025 Ovie Language Team",       f"Copyright (c) {YEAR} Nashedy Ltd"),
    # README/docs badge
    ('[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)',
     '[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)'),  # keep, already correct after above
    # Source link fix (keep southwarridev reference, just update company name)
]

EXTENSIONS = {".md", ".ov", ".html", ".toml", ".txt", ".rs", ".bat", ".ps1", ".sh", ".yml", ".yaml", ".json"}
SKIP_DIRS  = {".git", "target", "node_modules", ".cargo"}

changed = 0
for root, dirs, files in os.walk(BASE):
    dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
    for fname in files:
        ext = os.path.splitext(fname)[1].lower()
        if ext not in EXTENSIONS:
            continue
        path = os.path.join(root, fname)
        try:
            with open(path, encoding="utf-8", errors="ignore") as f:
                content = f.read()
            original = content
            for old, new in REPLACEMENTS:
                content = content.replace(old, new)
            if content != original:
                with open(path, "w", encoding="utf-8") as f:
                    f.write(content)
                changed += 1
                print(f"  updated: {path[len(BASE)+1:]}")
        except Exception as e:
            print(f"  SKIP {fname}: {e}")

print(f"\nDone. {changed} files updated.")
