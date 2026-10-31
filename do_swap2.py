p = r'c:\Users\Home PC\Desktop\ovie\oviec\src\main.rs'
src = open(p, encoding='utf-8').read()
start = 33247
end = 34532
new_fn = open(r'c:\Users\Home PC\Desktop\ovie\aproko_new_fn.rs', encoding='utf-8').read()
result = src[:start] + new_fn + src[end+1:]
open(p, 'w', encoding='utf-8').write(result)
print('OK: replaced', end-start+1, 'chars with', len(new_fn), 'chars')
