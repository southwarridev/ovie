p = r'c:\Users\Home PC\Desktop\ovie\oviec\src\main.rs'
src = open(p, encoding='utf-8').read()
bin_name_pos = src.find('let bin_name = if cfg!')
# Go back to the newline before this orphaned code
orphan_start = bin_name_pos
while orphan_start > 0 and src[orphan_start-1] != chr(10): orphan_start -= 1
# Go back one more newline to eat the blank line
if orphan_start > 1 and src[orphan_start-1] == chr(10): orphan_start -= 1
print('orphan starts at:', orphan_start)
# The new clean function starts at the doc comment
new_doc = '/// After a runtime or compile error, Aproko'
new_fn_start = src.find(new_doc, bin_name_pos)
print('new fn starts at:', new_fn_start)
# Cut the orphan: everything from orphan_start to new_fn_start
result = src[:orphan_start] + chr(10) + chr(10) + src[new_fn_start:]
open(p, 'w', encoding='utf-8').write(result)
print('FIXED: removed', new_fn_start - orphan_start, 'orphan bytes')
