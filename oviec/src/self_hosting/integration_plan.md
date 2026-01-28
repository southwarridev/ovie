# Ovie-in-Ovie Lexer Integration Plan
## Stage 1 Self-Hosting Foundation

### Overview

This document outlines the plan for integrating the Ovie-in-Ovie lexer specification with the existing Rust-based Stage 0 compiler. This is the first step toward partial self-hosting (Stage 1) where critical compiler components are implemented in Ovie itself.

### Integration Strategy

#### Phase 1: Ovie Lexer Implementation
1. **Ovie Source Compilation**: The `lexer_spec.ov` file will be compiled using the existing Stage 0 Rust compiler
2. **IR Generation**: The Ovie lexer code will be transformed into Ovie IR
3. **Execution Environment**: The lexer will run within the IR interpreter or compiled to WASM/LLVM

#### Phase 2: Bootstrap Verification
1. **Dual Lexer Operation**: Both Rust and Ovie lexers will run in parallel
2. **Output Comparison**: Token streams from both lexers will be compared for identical results
3. **Hash Verification**: Cryptographic hashes will verify identical tokenization
4. **Performance Benchmarking**: Compare performance characteristics

#### Phase 3: Gradual Transition
1. **Feature Flag**: Add `--use-ovie-lexer` flag to enable Ovie lexer
2. **Fallback Mechanism**: Rust lexer remains as fallback for errors
3. **Testing Integration**: All existing lexer tests must pass with Ovie implementation
4. **Property Test Validation**: All property-based tests must pass

### Technical Implementation Details

#### Token Structure Mapping
The Ovie token structures map directly to the Rust equivalents:

```rust
// Rust TokenType enum
pub enum TokenType { Fn, Mut, If, ... }

// Ovie TokenType enum  
enum TokenType { Fn, Mut, If, ... }
```

#### Memory Management
- **Rust Side**: Manages memory for the Ovie lexer execution environment
- **Ovie Side**: Uses Ovie's ownership system for token and string management
- **Interface**: Clean FFI boundary with serialized token exchange

#### Error Handling
- **Ovie Errors**: Lexer errors in Ovie code are caught and translated
- **Rust Integration**: Errors are converted to `OvieError` types
- **Location Preservation**: Source locations are preserved across the boundary

### Standard Library Requirements

The Ovie lexer requires these standard library functions:

#### String Operations
- `char_at(text: String, index: Number) -> String`
- `substring(text: String, start: Number, end: Number) -> String`
- `length(text: String) -> Number`

#### Array Operations
- `append(mut array: [Token], item: Token)`
- Array indexing and iteration

#### Character Classification
- `is_digit(c: String) -> Boolean`
- `is_alpha(c: String) -> Boolean`
- `is_alpha_numeric(c: String) -> Boolean`

### Bootstrap Verification Process

#### Hash-Based Verification
1. **Input Hash**: SHA-256 of source code
2. **Token Stream Hash**: SHA-256 of serialized token stream
3. **Comparison**: Rust and Ovie lexer outputs must produce identical hashes

#### Property Preservation
All existing property-based tests must pass:
- **Property 1**: Language Grammar Compliance (Lexer component)
- Token type correctness
- Location accuracy
- Error handling consistency

### Performance Considerations

#### Expected Performance Impact
- **Initial Implementation**: 2-5x slower than Rust lexer (acceptable for Stage 1)
- **Optimization Target**: Within 50% of Rust performance by Stage 2
- **Memory Usage**: Similar memory footprint with Ovie's ownership system

#### Optimization Strategies
1. **Hot Path Optimization**: Optimize common tokenization paths
2. **String Interning**: Reuse common lexemes and keywords
3. **Batch Processing**: Process multiple characters per iteration

### Testing Strategy

#### Unit Tests
- All existing Rust lexer tests ported to Ovie
- New tests for Ovie-specific functionality
- Edge case handling verification

#### Integration Tests
- Full compilation pipeline with Ovie lexer
- Cross-platform compatibility testing
- Performance regression testing

#### Property-Based Tests
- Existing property tests must pass with Ovie lexer
- New properties for bootstrap verification
- Deterministic output verification

### Migration Timeline

#### Week 1: Foundation
- [ ] Implement standard library functions
- [ ] Create Ovie lexer execution environment
- [ ] Basic tokenization working

#### Week 2: Integration
- [ ] FFI boundary implementation
- [ ] Error handling integration
- [ ] Basic test suite passing

#### Week 3: Verification
- [ ] Bootstrap verification system
- [ ] All property tests passing
- [ ] Performance benchmarking

#### Week 4: Production Ready
- [ ] Feature flag implementation
- [ ] Documentation updates
- [ ] Stage 1 transition complete

### Risk Mitigation

#### Technical Risks
1. **Performance**: Fallback to Rust lexer if performance is unacceptable
2. **Correctness**: Extensive testing and verification before transition
3. **Complexity**: Gradual migration with feature flags

#### Process Risks
1. **Testing Coverage**: Comprehensive test suite before migration
2. **Rollback Plan**: Ability to disable Ovie lexer instantly
3. **Monitoring**: Performance and correctness monitoring in production

### Success Criteria

#### Functional Requirements
- [ ] All existing tests pass with Ovie lexer
- [ ] Identical token output to Rust lexer
- [ ] Error handling parity
- [ ] Cross-platform compatibility

#### Performance Requirements
- [ ] Lexer performance within 5x of Rust implementation
- [ ] Memory usage within 2x of Rust implementation
- [ ] No regression in overall compilation time > 10%

#### Quality Requirements
- [ ] Zero correctness regressions
- [ ] Deterministic output maintained
- [ ] Security properties preserved

### Next Steps

1. **Implement Standard Library**: Create the required string and array operations
2. **Create Execution Environment**: Set up the Ovie lexer execution context
3. **Build FFI Interface**: Create the boundary between Rust and Ovie code
4. **Implement Bootstrap Verification**: Create the hash-based verification system
5. **Port Test Suite**: Ensure all existing tests work with the Ovie lexer

This integration plan provides a systematic approach to transitioning from the Rust lexer to the Ovie lexer while maintaining all correctness, performance, and security guarantees of the Stage 0 system.