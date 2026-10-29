# -*- coding: utf-8 -*-
import os
BASE = r'c:\Users\Home PC\Desktop\ovie'
NL = chr(10)
BT = chr(96)  # backtick
D = chr(36)   # dollar

# ── README update ───────────────────────────────────────────────
readme = open(BASE + r'\README.md', encoding='utf-8').read()
if 'oviev' not in readme:
    sec = (
        NL + '## oviev - Ovie Self-Hosted Compiler' + NL + NL
        + 'Ovie ships two compilers. Both run .ov files.' + NL + NL
        + '| Compiler | Language | Status |' + NL
        + '|----------|----------|--------|' + NL
        + '| ' + BT + 'oviec' + BT + ' | Rust | Stable - use for everything today |' + NL
        + '| ' + BT + 'oviev' + BT + ' | Ovie (.ov) | Stage 1 - self-hosted, help test it |' + NL + NL
        + BT * 3 + 'bash' + NL
        + 'oviec --version    # Rust stable compiler' + NL
        + 'oviev --version    # Ovie self-hosted compiler' + NL
        + 'oviec run hello.ov # stable' + NL
        + 'oviev run hello.ov # self-hosted' + NL
        + BT * 3 + NL + NL
        + 'oviev mirrors every oviec/src/*.rs as a .ov file:' + NL
        + 'lexer.ov, parser.ov, ast.ov, hir.ov, mir.ov, ir.ov,' + NL
        + 'interpreter.ov, semantic.ov, error.ov, main.ov,' + NL
        + 'backend/llvm.ov, backend/wasm.ov' + NL + NL
        + 'See [oviev/README.md](oviev/README.md) for the full stage plan.' + NL
    )
    marker = '## Documentation'
    if marker in readme:
        readme = readme.replace(marker, sec + marker)
    else:
        readme += sec
    open(BASE + r'\README.md', 'w', encoding='utf-8').write(readme)
    print('README.md updated')
else:
    print('README.md: already has oviev')

# ── getting-started.md ──────────────────────────────────────────
gs = open(BASE + r'\docs\getting-started.md', encoding='utf-8').read()
if 'oviev' not in gs:
    gs += NL + '## oviev - Ovie Self-Hosted Compiler' + NL + NL
    gs += 'Both ' + BT + 'oviec' + BT + ' and ' + BT + 'oviev' + BT + ' run .ov files.' + NL + NL
    gs += BT * 3 + NL + 'oviec run hello.ov   # stable Rust compiler' + NL + 'oviev run hello.ov   # Ovie self-hosted (Stage 1)' + NL + BT * 3 + NL
    open(BASE + r'\docs\getting-started.md', 'w', encoding='utf-8').write(gs)
    print('getting-started.md updated')
else:
    print('getting-started.md: already has oviev')

# ── book chapter 01 ─────────────────────────────────────────────
ch01p = BASE + r'\docs\book\ovie-in-southern-kaduna\chapters\01-introduction.md'
ch01 = open(ch01p, encoding='utf-8').read()
if 'oviev' not in ch01:
    ch01 += NL + '## oviev - The Ovie Self-Hosted Compiler' + NL + NL
    ch01 += 'Ovie now ships two compilers. ' + BT + 'oviec' + BT + ' is Rust. ' + BT + 'oviev' + BT + ' is the same compiler rewritten in Ovie.' + NL + NL
    ch01 += 'Stage 1: oviev bootstraps via oviec. Stage 2: oviev compiles itself.' + NL + NL
    ch01 += BT * 3 + 'ovie' + NL + 'oya STAGE = ' + chr(34) + '1 - bootstrapped via oviec' + chr(34) + NL + 'seeAm ' + chr(34) + 'oviev is coming' + chr(34) + NL + BT * 3 + NL
    open(ch01p, 'w', encoding='utf-8').write(ch01)
    print('01-introduction.md updated')
else:
    print('01-introduction.md: already has oviev')

# ── language-guide.md ───────────────────────────────────────────
lgp = BASE + r'\docs\language-guide.md'
lg = open(lgp, encoding='utf-8').read()
if 'oviev' not in lg:
    lg += NL + '## oviev - Self-Hosted Compiler' + NL + NL + 'See [oviev/README.md](../oviev/README.md).' + NL
    open(lgp, 'w', encoding='utf-8').write(lg)
    print('language-guide.md updated')
else:
    print('language-guide.md: already has oviev')

# ── website index.html ──────────────────────────────────────────
webp = BASE + r'\website\index.html'
web = open(webp, encoding='utf-8').read()
if 'oviev' not in web:
    OVIEV_CARD = (
        '<div class=' + chr(34) + 'feature-card' + chr(34) + '>'
        + '<div class=' + chr(34) + 'feature-icon' + chr(34) + '>&#127381;</div>'
        + '<h3 class=' + chr(34) + 'feature-title' + chr(34) + '>oviev Self-Hosted</h3>'
        + '<p class=' + chr(34) + 'feature-description' + chr(34) + '>The Ovie compiler written in Ovie. 15 .ov files mirror every oviec/src/*.rs file. Stage 1 ships with every install.</p>'
        + '</div>'
    )
    web = web.replace('</div>' + chr(10) + '</section>' + chr(10) + chr(10) + '<!-- Module System', OVIEV_CARD + '</div>' + chr(10) + '</section>' + chr(10) + chr(10) + '<!-- Module System', 1)
    open(webp, 'w', encoding='utf-8').write(web)
    print('website updated with oviev card')
else:
    print('website: already has oviev')

print('ALL DONE')
