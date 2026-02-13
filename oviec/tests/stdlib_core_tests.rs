//! Tests for the Ovie Standard Library Core Module
//! 
//! This file contains comprehensive tests for the core stdlib types and functions.

use oviec::stdlib::{
    OvieResult, OvieOption, OvieVec, OvieHashMap,
    ok, err, some, none,
    identity, swap, min, max, clamp,
};

#[test]
fn test_result_basic_operations() {
    let ok_result: OvieResult<i32, String> = ok(42);
    let err_result: OvieResult<i32, String> = err("error".to_string());
    
    assert!(ok_result.is_ok());
    assert!(!ok_result.is_err());
    assert!(!err_result.is_ok());
    assert!(err_result.is_err());
    
    assert_eq!(ok_result.unwrap(), 42);
    assert_eq!(err_result.unwrap_or(0), 0);
}

#[test]
fn test_result_map_operations() {
    let ok_result: OvieResult<i32, String> = ok(42);
    let err_result: OvieResult<i32, String> = err("error".to_string());
    
    let mapped = ok_result.map(|x| x.to_string());
    assert_eq!(mapped, ok("42".to_string()));
    
    let mapped_err = err_result.map_err(|e| format!("Error: {}", e));
    assert_eq!(mapped_err, err("Error: error".to_string()));
}

#[test]
fn test_result_and_then() {
    let ok_result: OvieResult<i32, String> = ok(42);
    let err_result: OvieResult<i32, String> = err("error".to_string());
    
    let doubled = ok_result.and_then(|x| ok(x * 2));
    assert_eq!(doubled, ok(84));
    
    let failed = err_result.and_then(|x| ok(x * 2));
    assert_eq!(failed, err("error".to_string()));
}

#[test]
fn test_option_basic_operations() {
    let some_option = some(42);
    let none_option: OvieOption<i32> = none();
    
    assert!(some_option.is_some());
    assert!(!some_option.is_none());
    assert!(!none_option.is_some());
    assert!(none_option.is_none());
    
    assert_eq!(some_option.unwrap(), 42);
    assert_eq!(none_option.unwrap_or(0), 0);
}

#[test]
fn test_option_and_then() {
    let some_option = some(42);
    let none_option: OvieOption<i32> = none();
    
    let doubled = some_option.and_then(|x| some(x * 2));
    assert_eq!(doubled, some(84));
    
    let failed = none_option.and_then(|x| some(x * 2));
    assert_eq!(failed, none());
}

#[test]
fn test_option_ok_or() {
    let some_option = some(42);
    let none_option: OvieOption<i32> = none();
    
    let ok_result = some_option.ok_or("error".to_string());
    assert_eq!(ok_result, ok(42));
    
    let err_result = none_option.ok_or("error".to_string());
    assert_eq!(err_result, err("error".to_string()));
}

#[test]
fn test_vec_basic_operations() {
    let mut vec = OvieVec::new();
    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
    
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(1), some(2));
    
    let popped = vec.pop();
    assert_eq!(popped, some(3));
    assert_eq!(vec.len(), 2);
}

#[test]
fn test_vec_advanced_operations() {
    let mut vec = OvieVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    // Test set operation
    assert!(vec.set(1, 10).is_ok());
    assert_eq!(vec.get(1), some(10));
    
    // Test insert operation
    assert!(vec.insert(1, 5).is_ok());
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(1), some(5));
    assert_eq!(vec.get(2), some(10));
    
    // Test remove operation
    let removed = vec.remove(1);
    assert_eq!(removed, ok(5));
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(1), some(10));
    
    // Test clear
    vec.clear();
    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_vec_iterator() {
    let mut vec = OvieVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    let mut iter = vec.iter();
    assert_eq!(iter.next(), some(1));
    assert_eq!(iter.next(), some(2));
    assert_eq!(iter.next(), some(3));
    assert_eq!(iter.next(), none());
    
    // Test count
    let iter2 = vec.iter();
    assert_eq!(iter2.count(), 3);
}

#[test]
fn test_hashmap_basic_operations() {
    let mut map = OvieHashMap::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    
    map.insert("key1".to_string(), 42);
    map.insert("key2".to_string(), 84);
    
    assert!(!map.is_empty());
    assert_eq!(map.len(), 2);
    assert!(map.contains_key(&"key1".to_string()));
    assert_eq!(map.get(&"key1".to_string()), some(42));
    
    let removed = map.remove(&"key1".to_string());
    assert_eq!(removed, some(42));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_hashmap_advanced_operations() {
    let mut map = OvieHashMap::new();
    
    // Test try_insert
    assert!(map.try_insert("key1".to_string(), 42).is_ok());
    assert_eq!(map.get(&"key1".to_string()), some(42));
    
    // Test get_mut
    if let OvieOption::Some(value) = map.get_mut(&"key1".to_string()) {
        *value = 84;
    }
    assert_eq!(map.get(&"key1".to_string()), some(84));
    
    // Test keys iterator
    map.insert("key2".to_string(), 100);
    let mut keys_iter = map.keys();
    assert_eq!(keys_iter.next(), some("key1".to_string()));
    assert_eq!(keys_iter.next(), some("key2".to_string()));
    assert_eq!(keys_iter.next(), none());
    
    // Test values iterator
    let mut values_iter = map.values();
    assert_eq!(values_iter.next(), some(84));
    assert_eq!(values_iter.next(), some(100));
    assert_eq!(values_iter.next(), none());
    
    // Test clear
    map.clear();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_deterministic_iteration() {
    let mut map1 = OvieHashMap::new();
    let mut map2 = OvieHashMap::new();
    
    // Insert in different orders
    map1.insert("a", 1);
    map1.insert("b", 2);
    map1.insert("c", 3);
    
    map2.insert("c", 3);
    map2.insert("a", 1);
    map2.insert("b", 2);
    
    // Iteration should be deterministic based on insertion order
    let mut iter1 = map1.iter();
    let mut iter2 = map2.iter();
    
    // First map: a, b, c
    assert_eq!(iter1.next(), some(("a", 1)));
    assert_eq!(iter1.next(), some(("b", 2)));
    assert_eq!(iter1.next(), some(("c", 3)));
    
    // Second map: c, a, b
    assert_eq!(iter2.next(), some(("c", 3)));
    assert_eq!(iter2.next(), some(("a", 1)));
    assert_eq!(iter2.next(), some(("b", 2)));
}

#[test]
fn test_utility_functions() {
    assert_eq!(min(5, 3), 3);
    assert_eq!(max(5, 3), 5);
    assert_eq!(clamp(10, 0, 5), 5);
    assert_eq!(clamp(-5, 0, 5), 0);
    assert_eq!(clamp(3, 0, 5), 3);
    
    assert_eq!(identity(42), 42);
    
    let mut a = 1;
    let mut b = 2;
    swap(&mut a, &mut b);
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}