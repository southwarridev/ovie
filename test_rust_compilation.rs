#!/usr/bin/env rust-script
//! Simple Rust compilation test for Ovie codebase
//! 
//! This script tests basic compilation of core Ovie modules
//! to verify all syntax and type errors are resolved.

use std::process::Command;
use std::path::Path;

fn main() {
    println!("🦀 Testing Rust compilation for Ovie codebase...\n");
    
    // Test 1: Check basic syntax with cargo check
    println!("1️⃣ Running cargo check on main workspace...");
    let output = Command::new("cargo")
        .args(&["check", "--workspace", "--all-targets"])
        .output()
        .expect("Failed to execute cargo check");
    
    if output.status.success() {
        println!("✅ Main workspace check passed");
    } else {
        println!("❌ Main workspace check failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    
    // Test 2: Check oviec crate specifically
    println!("\n2️⃣ Running cargo check on oviec crate...");
    let output = Command::new("cargo")
        .args(&["check", "--manifest-path", "oviec/Cargo.toml"])
        .output()
        .expect("Failed to execute cargo check on oviec");
    
    if output.status.success() {
        println!("✅ Oviec crate check passed");
    } else {
        println!("❌ Oviec crate check failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    
    // Test 3: Check aproko crate
    println!("\n3️⃣ Running cargo check on aproko crate...");
    let output = Command::new("cargo")
        .args(&["check", "--manifest-path", "aproko/Cargo.toml"])
        .output()
        .expect("Failed to execute cargo check on aproko");
    
    if output.status.success() {
        println!("✅ Aproko crate check passed");
    } else {
        println!("❌ Aproko crate check failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    
    // Test 4: Check ovie CLI crate
    println!("\n4️⃣ Running cargo check on ovie CLI crate...");
    let output = Command::new("cargo")
        .args(&["check", "--manifest-path", "ovie/Cargo.toml"])
        .output()
        .expect("Failed to execute cargo check on ovie CLI");
    
    if output.status.success() {
        println!("✅ Ovie CLI crate check passed");
    } else {
        println!("❌ Ovie CLI crate check failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    
    // Test 5: Try to compile a simple test
    println!("\n5️⃣ Testing basic compilation with simple test...");
    let output = Command::new("cargo")
        .args(&["test", "--lib", "--no-run", "--manifest-path", "oviec/Cargo.toml"])
        .output()
        .expect("Failed to execute cargo test");
    
    if output.status.success() {
        println!("✅ Test compilation passed");
    } else {
        println!("❌ Test compilation failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    
    println!("\n🎉 All Rust compilation tests passed!");
    println!("✨ The codebase appears to be error-free and ready for use.");
}