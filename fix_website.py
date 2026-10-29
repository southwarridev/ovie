import re, sys
p = r"c:\Users\Home PC\Desktop\ovie\website\index.html"
with open(p, encoding="utf-8") as f:
    html = f.read()

# 1. Replace hero code block
old_block = re.search(r'<pre><code class="language-ovie">// Ovie v2\.3.*?</code></pre>', html, re.DOTALL)
if old_block:
    new_code = '''<pre><code class="language-ovie">// Ovie v2.3 — oya &amp; mut keywords
oya APP_NAME = "Grade Calculator"
oya PASS     = 60

fn grade(score: Number) -&gt; String {
    if score &gt;= 90 { return "A" }
    if score &gt;= 80 { return "B" }
    if score &gt;= PASS { return "C" }
    return "F"
}

print("Enter score: ")
mut score  = string_to_number(string_trim(read_line()))
mut result = grade(score)

seeAm APP_NAME
seeAm "Grade: " + result</code></pre>'''
    html = html[:old_block.start()] + new_code + html[old_block.end():]
    print("hero replaced")
else:
    print("hero NOT found")

# 2. Remove "13 keywords" from the Accessible feature card
html = html.replace(
    "Natural syntax with <code>seeAm</code> (Nigerian Pidgin). 13 keywords. Built for developers everywhere.",
    "Natural syntax. <code>oya</code> for immutable, <code>mut</code> for mutable, <code>seeAm</code> for output. Built for developers everywhere."
)

with open(p, "w", encoding="utf-8") as f:
    f.write(html)
print("done")
