#!/bin/bash
# Ovie Testing Script - Linux/macOS
# Quick test runner for all Ovie features

echo "🧪 Ovie Testing Suite"
echo "====================="
echo ""

# Test 1: Bootstrap Compiler
echo "✅ Test 1: Bootstrap Compiler (Ovie compiling Ovie)"
cargo run --bin oviec -- oviec/src/self_hosting/bootstrap_compiler_simple.ov
if [ $? -eq 0 ]; then
    echo "✅ Bootstrap compiler PASSED"
else
    echo "❌ Bootstrap compiler FAILED"
fi
echo ""

# Test 2: Array Operations
echo "✅ Test 2: Array Literals and Operations"
cargo run --bin oviec -- test_array_simple.ov
if [ $? -eq 0 ]; then
    echo "✅ Array operations PASSED"
else
    echo "❌ Array operations FAILED"
fi
echo ""

# Test 3: Struct Operations
echo "✅ Test 3: Struct Instantiation and Field Access"
cargo run --bin oviec -- test_struct_comprehensive.ov
if [ $? -eq 0 ]; then
    echo "✅ Struct operations PASSED"
else
    echo "❌ Struct operations FAILED"
fi
echo ""

# Test 4: Hello World
echo "✅ Test 4: Hello World Example"
cargo run --bin oviec -- examples/hello.ov
if [ $? -eq 0 ]; then
    echo "✅ Hello World PASSED"
else
    echo "❌ Hello World FAILED"
fi
echo ""

# Test 5: Functions
echo "✅ Test 5: Function Declarations and Calls"
cargo run --bin oviec -- examples/functions.ov
if [ $? -eq 0 ]; then
    echo "✅ Functions PASSED"
else
    echo "❌ Functions FAILED"
fi
echo ""

echo "🎉 Testing Complete!"
echo "====================="
