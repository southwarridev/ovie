p = r'c:\Users\Home PC\Desktop\ovie\oviec\src\main.rs'
src = open(p, encoding='utf-8').read()
Q = chr(34)
NL = chr(10)

# Replace the bare 'looks like a constant' block with keyword-aware version
OLD = 'if !var.is_empty() {' + NL
OLD += '                hints.push(H{line:ln,level:' + Q + 'warning' + Q + ',' + NL
OLD += '                    msg:format!(' + Q + chr(39) + '{}' + chr(39) + ' looks like a constant - use oya to prevent accidental changes' + Q + ',var),' + NL
OLD += '                    hint:format!(' + Q + 'oya {} = ...' + Q + ',var)});' + NL
OLD += '            }'
pos = src.find(OLD)
print('OLD block at:', pos)

NEW = 'if !var.is_empty() {' + NL
NEW += '                let bad: &[&str] = &[' + Q + 'see' + Q + ',' + Q + 'var' + Q + ',' + Q + 'let' + Q + ',' + Q + 'def' + Q + ',' + Q + 'val' + Q + ',' + Q + 'dim' + Q + ',' + Q + 'function' + Q + ',' + Q + 'set' + Q + '];' + NL
NEW += '                if bad.contains(&var.as_str()) {' + NL
NEW += '                    hints.push(H{line:ln,level:' + Q + 'error' + Q + ',' + NL
NEW += '                        msg:format!(' + Q + chr(39) + '{}' + chr(39) + ' is not an Ovie keyword - use oya (immutable) or mut (mutable)' + Q + ',var),' + NL
NEW += '                        hint:format!(' + Q + 'oya {} = value  or  mut {} = value' + Q + ',var,var)});' + NL
NEW += '                } else {' + NL
NEW += '                    hints.push(H{line:ln,level:' + Q + 'warning' + Q + ',' + NL
NEW += '                        msg:format!(' + Q + chr(39) + '{}' + chr(39) + ' looks like a constant - use oya to prevent accidental changes' + Q + ',var),' + NL
NEW += '                        hint:format!(' + Q + 'oya {} = ...' + Q + ',var)});' + NL
NEW += '                }' + NL
NEW += '            }'

if pos >= 0:
    src = src[:pos] + NEW + src[pos+len(OLD):]
    print('keyword check added')
else:
    print('OLD block not found')

# Fix get_inline_hint: extract var name from after LAST colon not first
OLD2 = 'let parts: Vec<&str> = msg.split(' + chr(39) + ':' + chr(39) + ').collect();' + NL + '        if parts.len() > 1 {' + NL + '            let var_name = parts[1].trim();'
NEW2 = 'let var_name = msg.split(' + chr(39) + ':' + chr(39) + ').last().unwrap_or(' + Q + Q + ').trim().to_string();' + NL + '        {' + NL + '            let var_name = var_name.as_str();'
pos2 = src.find(OLD2)
print('get_inline_hint extract pos:', pos2)
if pos2 >= 0:
    src = src[:pos2] + NEW2 + src[pos2+len(OLD2):]
    print('var extraction fixed')

open(p, 'w', encoding='utf-8').write(src)
print('done. size:', len(src))
