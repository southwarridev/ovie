p = r'c:\Users\Home PC\Desktop\ovie\oviec\src\main.rs'
src = open(p, encoding='utf-8').read()
Q = chr(34); NL = chr(10)

# Fix function name extraction - use last() instead of first()
OLD = ('if parts.len() > 1 { let fn_name = parts[1].trim(); return format!(' + Q
    + '+ chr(39) + 'Function ' + chr(39) + '{}' + chr(39) + ' is not defined. Check spelling or import the module.' + Q + ', fn_name); }')
NEW = ('{ let fn_name = parts.last().unwrap_or(&' + Q + Q + ').trim(); return format!(' + Q
    + '+ chr(39) + 'Function ' + chr(39) + '{}' + chr(39) + ' is not defined. Check spelling. Import with: use std::module::{{{}}}\' + Q + ', fn_name, fn_name); }')
pos = src.find(OLD)
print('fn extract pos:', pos)
if pos >= 0:
    src = src[:pos] + NEW + src[pos+len(OLD):]
    open(p, 'w', encoding='utf-8').write(src)
    print('fixed')
else:
    print('NOT FOUND')
