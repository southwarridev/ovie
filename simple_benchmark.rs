use std::time::Instant;

fn main() {
    println!("=== Simple Ovie Compiler Performance Test ===\n");
    
    // Test basic compilation performance
    let test_programs = vec![
        ("Hello World", r#"seeAm "Hello, World!";"#),
        ("Variables", r#"x = 42; y = "test"; seeAm x;"#),
        ("Multiple Statements", r#"
            a = 1;
            b = 2;
            c = a + b;
            seeAm c;
            seeAm "Done";
        "#),
    ];
    
    for (name, program) in test_programs {
        println!("Testing: {}", name);
        
        // Time lexing
        let start = Instant::now();
        let mut lexer = oviec::lexer::Lexer::new(program);
        let tokens = lexer.tokenize();
        let lex_time = start.elapsed();
        
        if let Ok(tokens) = tokens {
            println!("  Lexing: {:?} ({} tokens)", lex_time, tokens.len());
            
            // Time parsing
            let start = Instant::now();
            let mut parser = oviec::parser::Parser::new(tokens);
            let ast = parser.parse();
            let parse_time = start.elapsed();
            
            if let Ok(ast) = ast {
                println!("  Parsing: {:?} ({} statements)", parse_time, ast.statements.len());
                
                // Time IR generation
                let start = Instant::now();
                let mut ir_builder = oviec::ir::IrBuilder::new();
                let ir_result = ir_builder.transform_ast(&ast);
                let ir_time = start.elapsed();
                
                if ir_result.is_ok() {
                    let ir_program = ir_builder.build();
                    println!("  IR Generation: {:?} ({} functions)", ir_time, ir_program.functions.len());
                    
                    // Total time
                    let total_time = lex_time + parse_time + ir_time;
                    println!("  Total: {:?}", total_time);
                } else {
                    println!("  IR Generation: FAILED");
                }
            } else {
                println!("  Parsing: FAILED");
            }
        } else {
            println!("  Lexing: FAILED");
        }
        
        println!();
    }
    
    // Test scalability with repeated statements
    println!("=== Scalability Test ===");
    let sizes = vec![10, 50, 100];
    
    for size in sizes {
        let program = (0..size)
            .map(|i| format!("x_{} = {}; seeAm x_{};", i, i, i))
            .collect::<Vec<_>>()
            .join("\n");
        
        let start = Instant::now();
        
        let mut lexer = oviec::lexer::Lexer::new(&program);
        if let Ok(tokens) = lexer.tokenize() {
            let mut parser = oviec::parser::Parser::new(tokens);
            if let Ok(ast) = parser.parse() {
                let mut ir_builder = oviec::ir::IrBuilder::new();
                if ir_builder.transform_ast(&ast).is_ok() {
                    let total_time = start.elapsed();
                    println!("{} statements: {:?} ({:.2} stmt/ms)", 
                        size, total_time, size as f64 / total_time.as_millis() as f64);
                }
            }
        }
    }
    
    println!("\nPerformance benchmarking completed!");
}