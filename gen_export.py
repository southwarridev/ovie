import json
BASE = r'c:\Users\Home PC\Desktop\ovie'
kb = {'version':'2.3.0','license':'MIT','copyright':'Copyright (c) 2026 Nashedy Ltd','aproko':'Urhobo word — one who notices everything and tells it straight'}
kb['keywords'] = {'oya':{'kind':'declaration','mutability':'immutable','desc':'Immutable variable. Use by default.'},'mut':{'kind':'declaration','mutability':'mutable','desc':'Mutable variable.'},'seeAm':{'kind':'output','desc':'Print to terminal. Nigerian Pidgin English.'},'fn':{'kind':'declaration','desc':'Define a function.'},'if':{'kind':'control','desc':'Conditional.'},'else':{'kind':'control','desc':'Alternative.'},'while':{'kind':'control','desc':'Loop while true.'},'for':{'kind':'control','desc':'Iterate over range.'},'return':{'kind':'control','desc':'Return from function.'},'break':{'kind':'control','desc':'Exit loop.'},'continue':{'kind':'control','desc':'Skip iteration.'},'match':{'kind':'control','desc':'Pattern match.'},'struct':{'kind':'declaration','desc':'Data structure.'},'enum':{'kind':'declaration','desc':'Enumeration.'},'const':{'kind':'declaration','desc':'Compile-time constant.'},'use':{'kind':'module','desc':'Import from stdlib.'},'import':{'kind':'module','desc':'Import from file.'},'export':{'kind':'module','desc':'Export for others.'},'unsafe':{'kind':'safety','desc':'Unsafe block.'},'true':{'kind':'constant','desc':'Boolean true.'},'false':{'kind':'constant','desc':'Boolean false.'}}
kb['types'] = {'Number':'integer or decimal','String':'text','Boolean':'true or false','Array':'ordered list','Result':'Result.Ok(val) or Result.Err(msg)','Option':'Option.Some(val) or Option.None'}
kb['builtins'] = {'string_length':'(s:String)->Number','string_trim':'(s:String)->String','string_split':'(s:String,sep:String)->Array','string_to_number':'(s:String)->Number','number_to_string':'(n:Number)->String','string_contains':'(s:String,sub:String)->Boolean','string_replace':'(s:String,from:String,to:String)->String','array_length':'(arr:Array)->Number','array_get':'(arr:Array,i:Number)->Any','array_push':'(arr:Array,val:Any)->Array','read_line':'()->String','print':'(text:String)->void'}
kb['aproko_rules'] = [{'rule':'prefer_oya','msg':'Use oya for values that never change','severity':'warning'},{'rule':'mut_never_reassigned','msg':'mut never reassigned use oya','severity':'info'},{'rule':'while_true_loop','msg':'while true without visible break','severity':'warning'},{'rule':'division_by_zero','msg':'Division by literal zero','severity':'error'},{'rule':'read_line_not_trimmed','msg':'read_line not wrapped in string_trim','severity':'warning'}]
kb['patterns'] = {'immutable_constant':'oya NAME = value','mutable_variable':'mut name = value','function':'fn name(param: Type) -> ReturnType { return value }','struct':'struct Name { field: Type }','input_loop':'mut running = true  while running { print(prompt) mut input = string_trim(read_line()) if input == quit { running = false } }'}
kb['stdlib'] = ['std::core','std::math','std::io','std::fs','std::time','std::env','std::cli','std::log','std::testing','std::module','std::aproko']
js = json.dumps(kb, indent=2)
out_path = BASE + r'\oviec\src\aproko_export.rs'
hdr = 'use crate::CliArgs;' + chr(10) + 'use oviec::OvieResult;' + chr(10) + 'use std::fs;' + chr(10)
const_open = '/// Ovie knowledge base — local JSON for LLM consumption.' + chr(10) + 'pub const KNOWLEDGE_JSON: &str = r' + chr(35) + chr(34)
const_close = chr(34) + chr(35) + ';' + chr(10)
fn_body = '''pub fn aproko_export(args: CliArgs) -> OvieResult<()> {
    let out = args.output_file.unwrap_or_else(|| String::from(chr(34) + 'ovie_knowledge.json' + chr(34)));
    fs::write(&out, KNOWLEDGE_JSON).map_err(|e| oviec::OvieError::io_error(format!(chr(34) + 'Cannot write {}: {}' + chr(34) + ', out, e)))?;
    println!(chr(34) + 'Aproko knowledge exported to: {}' + chr(34) + ', out);
    println!(chr(34) + 'Feed this file to any local LLM. Fully offline.' + chr(34) + ');
    Ok(())
}'''
fn_body = fn_body.replace('chr(34)', chr(39))
dq=chr(34); nl=chr(10)
rs = hdr + const_open + nl + js + nl + const_close + nl
rs += 'pub fn aproko_export(args: CliArgs) -> OvieResult<()> {' + nl
rs += '    let out = args.output_file.unwrap_or_else(|| String::from(' + dq + 'ovie_knowledge.json' + dq + '));' + nl
rs += '    fs::write(&out, KNOWLEDGE_JSON).map_err(|e| oviec::OvieError::io_error(format!(' + dq + 'Cannot write {}: {}' + dq + ', out, e)))?;' + nl
rs += '    println!(' + dq + 'Aproko knowledge base exported to: {}' + dq + ', out);' + nl
rs += '    println!(' + dq + 'Fully local. Feed to any LLM to teach it Ovie.' + dq + ');' + nl
rs += '    Ok(())' + nl + '}' + nl
open(out_path, 'w', encoding='utf-8').write(rs)
print('OK aproko_export.rs written:', len(rs), 'bytes')
