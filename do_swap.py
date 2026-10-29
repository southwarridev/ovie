import sys
p = r'c:\Users\Home PC\Desktop\ovie\oviec\src\main.rs'
src = open(p, encoding='utf-8').read()
MARKER = '/// Run Aproko static analysis directly'
FN_SIG = 'fn run_aproko_inline'
start = src.find(MARKER)
if start == -1:
    print('ERROR: marker not found'); sys.exit(1)
fn_start = src.find(FN_SIG, start)
brace_start = src.find('{', fn_start)
depth = 0; end = brace_start
for i, ch in enumerate(src[brace_start:], brace_start):
    if ch == '{': depth += 1
    elif ch == '}': depth -= 1
    if depth == 0: end = i; break
new_fn = open(r'c:\Users\Home PC\Desktop\ovie\aproko_new_fn.rs', encoding='utf-8').read()
result = src[:start] + new_fn + src[end+1:]
open(p, 'w', encoding='utf-8').write(result)
print('OK: replaced', end-start+1, 'chars')
