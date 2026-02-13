//! Ovie Standard Library - Core Module Runtime Implementation
//! 
//! This module provides the runtime implementation of std::core types and functions
//! that are specified in std/core/mod.ov. These implementations are used by the
//! Ovie compiler when generating code that uses standard library functionality.

use std::collections::HashMap as StdHashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Result type for operations that can fail
/// This is the primary error handling mechanism in Ovie
#[derive(Debug, Clone, PartialEq)]
pub enum OvieResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> OvieResult<T, E> {
    /// Check if the result is Ok
    pub fn is_ok(&self) -> bool {
        matches!(self, OvieResult::Ok(_))
    }
    
    /// Check if the result is Err
    pub fn is_err(&self) -> bool {
        matches!(self, OvieResult::Err(_))
    }
    
    /// Unwrap the Ok value, panic on Err
    pub fn unwrap(self) -> T {
        match self {
            OvieResult::Ok(value) => value,
            OvieResult::Err(_) => panic!("Called unwrap on an Err value"),
        }
    }
    
    /// Unwrap the Ok value, return default on Err
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            OvieResult::Ok(value) => value,
            OvieResult::Err(_) => default,
        }
    }
    
    /// Unwrap the Ok value, or compute default from closure
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            OvieResult::Ok(value) => value,
            OvieResult::Err(error) => f(error),
        }
    }
    
    /// Unwrap the Err value, panic on Ok
    pub fn unwrap_err(self) -> E {
        match self {
            OvieResult::Err(error) => error,
            OvieResult::Ok(_) => panic!("Called unwrap_err on an Ok value"),
        }
    }
    
    /// Map the Ok value to a new type
    pub fn map<U, F>(self, f: F) -> OvieResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            OvieResult::Ok(value) => OvieResult::Ok(f(value)),
            OvieResult::Err(error) => OvieResult::Err(error),
        }
    }
    
    /// Map the Err value to a new type
    pub fn map_err<F, G>(self, f: F) -> OvieResult<T, G>
    where
        F: FnOnce(E) -> G,
    {
        match self {
            OvieResult::Ok(value) => OvieResult::Ok(value),
            OvieResult::Err(error) => OvieResult::Err(f(error)),
        }
    }
    
    /// Chain operations that return Results
    pub fn and_then<U, F>(self, f: F) -> OvieResult<U, E>
    where
        F: FnOnce(T) -> OvieResult<U, E>,
    {
        match self {
            OvieResult::Ok(value) => f(value),
            OvieResult::Err(error) => OvieResult::Err(error),
        }
    }
}

/// Helper functions for creating Results
pub fn ok<T, E>(value: T) -> OvieResult<T, E> {
    OvieResult::Ok(value)
}

pub fn err<T, E>(error: E) -> OvieResult<T, E> {
    OvieResult::Err(error)
}

/// Option type for values that may or may not exist
/// This eliminates null pointer exceptions
#[derive(Debug, Clone, PartialEq)]
pub enum OvieOption<T> {
    Some(T),
    None,
}

impl<T> OvieOption<T> {
    /// Check if the option is Some
    pub fn is_some(&self) -> bool {
        matches!(self, OvieOption::Some(_))
    }
    
    /// Check if the option is None
    pub fn is_none(&self) -> bool {
        matches!(self, OvieOption::None)
    }
    
    /// Unwrap the Some value, panic on None
    pub fn unwrap(self) -> T {
        match self {
            OvieOption::Some(value) => value,
            OvieOption::None => panic!("Called unwrap on a None value"),
        }
    }
    
    /// Unwrap the Some value, return default on None
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            OvieOption::Some(value) => value,
            OvieOption::None => default,
        }
    }
    
    /// Unwrap the Some value, or compute default from closure
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            OvieOption::Some(value) => value,
            OvieOption::None => f(),
        }
    }
    
    /// Map the Some value to a new type
    pub fn map<U, F>(self, f: F) -> OvieOption<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            OvieOption::Some(value) => OvieOption::Some(f(value)),
            OvieOption::None => OvieOption::None,
        }
    }
    
    /// Chain operations that return Options
    pub fn and_then<U, F>(self, f: F) -> OvieOption<U>
    where
        F: FnOnce(T) -> OvieOption<U>,
    {
        match self {
            OvieOption::Some(value) => f(value),
            OvieOption::None => OvieOption::None,
        }
    }
    
    /// Convert Option to Result
    pub fn ok_or<E>(self, error: E) -> OvieResult<T, E> {
        match self {
            OvieOption::Some(value) => OvieResult::Ok(value),
            OvieOption::None => OvieResult::Err(error),
        }
    }
}

/// Helper functions for creating Options
pub fn some<T>(value: T) -> OvieOption<T> {
    OvieOption::Some(value)
}

pub fn none<T>() -> OvieOption<T> {
    OvieOption::None
}

/// Dynamic array type with deterministic behavior
#[derive(Debug, Clone)]
pub struct OvieVec<T> {
    data: Vec<T>,
}

impl<T> OvieVec<T> {
    /// Create a new empty vector
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    
    /// Create a vector with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    
    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get the capacity of the vector
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    
    /// Push an element to the end
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }
    
    /// Pop an element from the end
    pub fn pop(&mut self) -> OvieOption<T> {
        match self.data.pop() {
            Some(item) => OvieOption::Some(item),
            None => OvieOption::None,
        }
    }
    
    /// Get an element at index
    pub fn get(&self, index: usize) -> OvieOption<T>
    where
        T: Clone,
    {
        match self.data.get(index) {
            Some(item) => OvieOption::Some(item.clone()),
            None => OvieOption::None,
        }
    }
    
    /// Get a reference to an element at index (without cloning)
    pub fn get_ref(&self, index: usize) -> OvieOption<&T> {
        match self.data.get(index) {
            Some(item) => OvieOption::Some(item),
            None => OvieOption::None,
        }
    }
    
    /// Set an element at index
    pub fn set(&mut self, index: usize, item: T) -> OvieResult<(), String> {
        if index >= self.data.len() {
            return OvieResult::Err("Index out of bounds".to_string());
        }
        
        self.data[index] = item;
        OvieResult::Ok(())
    }
    
    /// Insert an element at index
    pub fn insert(&mut self, index: usize, item: T) -> OvieResult<(), String> {
        if index > self.data.len() {
            return OvieResult::Err("Index out of bounds".to_string());
        }
        
        self.data.insert(index, item);
        OvieResult::Ok(())
    }
    
    /// Remove an element at index
    pub fn remove(&mut self, index: usize) -> OvieResult<T, String> {
        if index >= self.data.len() {
            return OvieResult::Err("Index out of bounds".to_string());
        }
        
        let item = self.data.remove(index);
        OvieResult::Ok(item)
    }
    
    /// Clear all elements
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Convert to slice
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
    
    /// Get iterator over elements
    pub fn iter(&self) -> OvieVecIterator<T> {
        OvieVecIterator {
            vec: self,
            index: 0,
        }
    }
    
    /// Get mutable iterator over elements
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }
    
    /// Extend vector with elements from iterator
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.data.extend(iter);
    }
    
    /// Append another vector to this one
    pub fn append(&mut self, other: &mut OvieVec<T>) {
        self.data.append(&mut other.data);
    }
    
    /// Split off the vector at the given index
    pub fn split_off(&mut self, at: usize) -> OvieVec<T> {
        OvieVec {
            data: self.data.split_off(at),
        }
    }
    
    /// Retain only elements that match the predicate
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.data.retain(f);
    }
    
    /// Reverse the vector in place
    pub fn reverse(&mut self) {
        self.data.reverse();
    }
    
    /// Sort the vector in place
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.data.sort();
    }
    
    /// Sort the vector in place with a comparison function
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.data.sort_by(compare);
    }
}

impl<T> Default for OvieVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Hash map for key-value storage with deterministic iteration order
#[derive(Debug, Clone)]
pub struct OvieHashMap<K, V> {
    data: StdHashMap<K, V>,
    insertion_order: Vec<K>,
}

impl<K, V> OvieHashMap<K, V>
where
    K: Hash + Eq + Clone,
{
    /// Create a new empty hash map
    pub fn new() -> Self {
        Self {
            data: StdHashMap::new(),
            insertion_order: Vec::new(),
        }
    }
    
    /// Get the number of key-value pairs
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Insert a key-value pair
    pub fn insert(&mut self, key: K, value: V) -> OvieOption<V> {
        let old_value = self.data.insert(key.clone(), value);
        
        // Track insertion order for deterministic iteration
        if old_value.is_none() {
            self.insertion_order.push(key);
        }
        
        match old_value {
            Some(old) => OvieOption::Some(old),
            None => OvieOption::None,
        }
    }
    
    /// Get a value by key
    pub fn get(&self, key: &K) -> OvieOption<V>
    where
        V: Clone,
    {
        match self.data.get(key) {
            Some(value) => OvieOption::Some(value.clone()),
            None => OvieOption::None,
        }
    }
    
    /// Get a reference to a value by key (without cloning)
    pub fn get_ref(&self, key: &K) -> OvieOption<&V> {
        match self.data.get(key) {
            Some(value) => OvieOption::Some(value),
            None => OvieOption::None,
        }
    }
    
    /// Remove a key-value pair
    pub fn remove(&mut self, key: &K) -> OvieOption<V> {
        let old_value = self.data.remove(key);
        
        // Remove from insertion order
        if old_value.is_some() {
            self.insertion_order.retain(|k| k != key);
        }
        
        match old_value {
            Some(value) => OvieOption::Some(value),
            None => OvieOption::None,
        }
    }
    
    /// Check if a key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    
    /// Get iterator over key-value pairs in insertion order
    pub fn iter(&self) -> OvieHashMapIterator<K, V> {
        OvieHashMapIterator {
            map: self,
            index: 0,
        }
    }
    
    /// Get iterator over keys in insertion order
    pub fn keys(&self) -> OvieHashMapKeysIterator<K, V> {
        OvieHashMapKeysIterator {
            map: self,
            index: 0,
        }
    }
    
    /// Get iterator over values in insertion order
    pub fn values(&self) -> OvieHashMapValuesIterator<K, V> {
        OvieHashMapValuesIterator {
            map: self,
            index: 0,
        }
    }
    
    /// Clear all key-value pairs
    pub fn clear(&mut self) {
        self.data.clear();
        self.insertion_order.clear();
    }
    
    /// Get mutable reference to value by key
    pub fn get_mut(&mut self, key: &K) -> OvieOption<&mut V> {
        match self.data.get_mut(key) {
            Some(value) => OvieOption::Some(value),
            None => OvieOption::None,
        }
    }
    
    /// Insert key-value pair only if key doesn't exist
    pub fn try_insert(&mut self, key: K, value: V) -> OvieResult<(), V> {
        if self.contains_key(&key) {
            OvieResult::Err(value)
        } else {
            self.insert(key, value);
            OvieResult::Ok(())
        }
    }
    
    /// Get key at specific index in insertion order
    pub fn get_key_at(&self, index: usize) -> OvieOption<K> {
        if index < self.insertion_order.len() {
            OvieOption::Some(self.insertion_order[index].clone())
        } else {
            OvieOption::None
        }
    }
    
    /// Get value at specific index in insertion order
    pub fn get_value_at(&self, index: usize) -> OvieOption<V>
    where
        V: Clone,
    {
        if index < self.insertion_order.len() {
            let key = &self.insertion_order[index];
            match self.data.get(key) {
                Some(value) => OvieOption::Some(value.clone()),
                None => OvieOption::None,
            }
        } else {
            OvieOption::None
        }
    }
}

impl<K, V> Default for OvieHashMap<K, V>
where
    K: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for OvieVec
pub struct OvieVecIterator<'a, T> {
    vec: &'a OvieVec<T>,
    index: usize,
}

impl<'a, T> OvieIterator for OvieVecIterator<'a, T>
where
    T: Clone,
{
    type Item = T;
    
    fn next(&mut self) -> OvieOption<T> {
        if self.index < self.vec.len() {
            let item = self.vec.get(self.index);
            self.index += 1;
            item
        } else {
            OvieOption::None
        }
    }
}

impl<'a, T> OvieVecIterator<'a, T>
where
    T: Clone,
{
    /// Get the next item (legacy method for compatibility)
    pub fn next_legacy(&mut self) -> OvieOption<T> {
        <Self as OvieIterator>::next(self)
    }
    
    /// Collect all remaining items into a vector (legacy method)
    pub fn collect_legacy(self) -> OvieVec<T> {
        <Self as OvieIterator>::collect(self)
    }
    
    /// Count the number of remaining items (legacy method)
    pub fn count_legacy(self) -> usize {
        <Self as OvieIterator>::count(self)
    }
}

/// Iterator for OvieHashMap
pub struct OvieHashMapIterator<'a, K, V> {
    map: &'a OvieHashMap<K, V>,
    index: usize,
}

impl<'a, K, V> OvieIterator for OvieHashMapIterator<'a, K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    type Item = (K, V);
    
    fn next(&mut self) -> OvieOption<(K, V)> {
        if self.index < self.map.insertion_order.len() {
            let key = &self.map.insertion_order[self.index];
            let value = self.map.data.get(key).unwrap().clone();
            self.index += 1;
            OvieOption::Some((key.clone(), value))
        } else {
            OvieOption::None
        }
    }
}

impl<'a, K, V> OvieHashMapIterator<'a, K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Get the next key-value pair (legacy method)
    pub fn next_legacy(&mut self) -> OvieOption<(K, V)> {
        <Self as OvieIterator>::next(self)
    }
}

/// Iterator for OvieHashMap keys
pub struct OvieHashMapKeysIterator<'a, K, V> {
    map: &'a OvieHashMap<K, V>,
    index: usize,
}

impl<'a, K, V> OvieIterator for OvieHashMapKeysIterator<'a, K, V>
where
    K: Clone,
{
    type Item = K;
    
    fn next(&mut self) -> OvieOption<K> {
        if self.index < self.map.insertion_order.len() {
            let key = &self.map.insertion_order[self.index];
            self.index += 1;
            OvieOption::Some(key.clone())
        } else {
            OvieOption::None
        }
    }
}

impl<'a, K, V> OvieHashMapKeysIterator<'a, K, V>
where
    K: Clone,
{
    /// Get the next key (legacy method)
    pub fn next_legacy(&mut self) -> OvieOption<K> {
        <Self as OvieIterator>::next(self)
    }
}

/// Iterator for OvieHashMap values
pub struct OvieHashMapValuesIterator<'a, K, V> {
    map: &'a OvieHashMap<K, V>,
    index: usize,
}

impl<'a, K, V> OvieIterator for OvieHashMapValuesIterator<'a, K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    type Item = V;
    
    fn next(&mut self) -> OvieOption<V> {
        if self.index < self.map.insertion_order.len() {
            let key = &self.map.insertion_order[self.index];
            let value = self.map.data.get(key).unwrap().clone();
            self.index += 1;
            OvieOption::Some(value)
        } else {
            OvieOption::None
        }
    }
}

impl<'a, K, V> OvieHashMapValuesIterator<'a, K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Get the next value (legacy method)
    pub fn next_legacy(&mut self) -> OvieOption<V> {
        <Self as OvieIterator>::next(self)
    }
}

/// Smart pointer for reference counting
#[derive(Debug)]
pub struct OvieRc<T> {
    data: std::rc::Rc<T>,
}

impl<T> OvieRc<T> {
    /// Create a new reference-counted value
    pub fn new(value: T) -> Self {
        Self {
            data: std::rc::Rc::new(value),
        }
    }
    
    /// Clone the reference (increment ref count)
    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
    
    /// Get the reference count
    pub fn strong_count(&self) -> usize {
        std::rc::Rc::strong_count(&self.data)
    }
    
    /// Get reference to the inner value
    pub fn get(&self) -> &T {
        &self.data
    }
}

/// Box for heap allocation
#[derive(Debug)]
pub struct OvieBox<T> {
    data: Box<T>,
}

impl<T> OvieBox<T> {
    /// Create a new boxed value
    pub fn new(value: T) -> Self {
        Self {
            data: Box::new(value),
        }
    }
    
    /// Get reference to the inner value
    pub fn get(&self) -> &T {
        &self.data
    }
    
    /// Convert to the inner value
    pub fn into_inner(self) -> T {
        *self.data
    }
}

/// Panic with a message
pub fn ovie_panic(message: &str) -> ! {
    panic!("OVIE PANIC: {}", message);
}

/// Assert that a condition is true
pub fn ovie_assert(condition: bool, message: &str) {
    if !condition {
        ovie_panic(&format!("Assertion failed: {}", message));
    }
}

/// Assert that two values are equal
pub fn ovie_assert_eq<T>(left: &T, right: &T, message: &str)
where
    T: PartialEq + std::fmt::Debug,
{
    if left != right {
        ovie_panic(&format!("Assertion failed: {} (left: {:?}, right: {:?})", message, left, right));
    }
}

/// Assert that two values are not equal
pub fn ovie_assert_ne<T>(left: &T, right: &T, message: &str)
where
    T: PartialEq + std::fmt::Debug,
{
    if left == right {
        ovie_panic(&format!("Assertion failed: {} (values are equal: {:?})", message, left));
    }
}

/// Identity function
pub fn identity<T>(value: T) -> T {
    value
}

/// Swap two values
pub fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

/// Minimum of two values
pub fn min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b { a } else { b }
}

/// Maximum of two values
pub fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b { a } else { b }
}

/// Clamp a value between min and max
pub fn clamp<T>(value: T, min_val: T, max_val: T) -> T
where
    T: PartialOrd,
{
    if value < min_val {
        min_val
    } else if value > max_val {
        max_val
    } else {
        value
    }
}

/// Deterministic hash function
pub fn deterministic_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

/// Iterator trait for Ovie collections
pub trait OvieIterator {
    type Item;
    
    /// Get the next item from the iterator
    fn next(&mut self) -> OvieOption<Self::Item>;
    
    /// Count the remaining items
    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        while self.next().is_some() {
            count += 1;
        }
        count
    }
    
    /// Collect all items into a Vec
    fn collect(mut self) -> OvieVec<Self::Item>
    where
        Self: Sized,
    {
        let mut result = OvieVec::new();
        while let OvieOption::Some(item) = self.next() {
            result.push(item);
        }
        result
    }
    
    /// Map each item to a new type
    fn map<U, F>(self, f: F) -> OvieMapIterator<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> U,
    {
        OvieMapIterator {
            iter: self,
            f,
        }
    }
    
    /// Filter items based on a predicate
    fn filter<F>(self, predicate: F) -> OvieFilterIterator<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        OvieFilterIterator {
            iter: self,
            predicate,
        }
    }
    
    /// Find the first item matching a predicate
    fn find<F>(mut self, mut predicate: F) -> OvieOption<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        while let OvieOption::Some(item) = self.next() {
            if predicate(&item) {
                return OvieOption::Some(item);
            }
        }
        OvieOption::None
    }
    
    /// Check if any item matches a predicate
    fn any<F>(mut self, mut predicate: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        while let OvieOption::Some(item) = self.next() {
            if predicate(item) {
                return true;
            }
        }
        false
    }
    
    /// Check if all items match a predicate
    fn all<F>(mut self, mut predicate: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        while let OvieOption::Some(item) = self.next() {
            if !predicate(item) {
                return false;
            }
        }
        true
    }
    
    /// Fold/reduce the iterator to a single value
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut acc = init;
        while let OvieOption::Some(item) = self.next() {
            acc = f(acc, item);
        }
        acc
    }
}

/// Map iterator adapter
pub struct OvieMapIterator<I, F> {
    iter: I,
    f: F,
}

impl<I, U, F> OvieIterator for OvieMapIterator<I, F>
where
    I: OvieIterator,
    F: FnMut(I::Item) -> U,
{
    type Item = U;
    
    fn next(&mut self) -> OvieOption<U> {
        match self.iter.next() {
            OvieOption::Some(item) => OvieOption::Some((self.f)(item)),
            OvieOption::None => OvieOption::None,
        }
    }
}

/// Filter iterator adapter
pub struct OvieFilterIterator<I, F> {
    iter: I,
    predicate: F,
}

impl<I, F> OvieIterator for OvieFilterIterator<I, F>
where
    I: OvieIterator,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    
    fn next(&mut self) -> OvieOption<I::Item> {
        while let OvieOption::Some(item) = self.iter.next() {
            if (self.predicate)(&item) {
                return OvieOption::Some(item);
            }
        }
        OvieOption::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_operations() {
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
    fn test_result_and_then() {
        let ok_result: OvieResult<i32, String> = ok(42);
        let err_result: OvieResult<i32, String> = err("error".to_string());
        
        let doubled = ok_result.and_then(|x| ok(x * 2));
        assert_eq!(doubled, ok(84));
        
        let failed = err_result.and_then(|x| ok(x * 2));
        assert_eq!(failed, err("error".to_string()));
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
    fn test_option_and_then() {
        let some_option = OvieOption::Some(42);
        let none_option: OvieOption<i32> = OvieOption::None;
        
        let doubled = some_option.and_then(|x| OvieOption::Some(x * 2));
        assert_eq!(doubled, OvieOption::Some(84));
        
        let failed = none_option.and_then(|x| OvieOption::Some(x * 2));
        assert_eq!(failed, OvieOption::None);
    }
    
    #[test]
    fn test_option_ok_or() {
        let some_option = OvieOption::Some(42);
        let none_option: OvieOption<i32> = OvieOption::None;
        
        let ok_result = some_option.ok_or("error".to_string());
        assert_eq!(ok_result, ok(42));
        
        let err_result = none_option.ok_or("error".to_string());
        assert_eq!(err_result, err("error".to_string()));
    }
    
    #[test]
    fn test_vec_advanced_operations() {
        let mut vec = OvieVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        // Test set operation
        assert!(vec.set(1, 10).is_ok());
        assert_eq!(vec.get(1), OvieOption::Some(10));
        
        // Test insert operation
        assert!(vec.insert(1, 5).is_ok());
        assert_eq!(vec.len(), 4);
        assert_eq!(vec.get(1), OvieOption::Some(5));
        assert_eq!(vec.get(2), OvieOption::Some(10));
        
        // Test remove operation
        let removed = vec.remove(1);
        assert_eq!(removed, ok(5));
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(1), OvieOption::Some(10));
        
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
        assert_eq!(iter.next(), OvieOption::Some(1));
        assert_eq!(iter.next(), OvieOption::Some(2));
        assert_eq!(iter.next(), OvieOption::Some(3));
        assert_eq!(iter.next(), OvieOption::None);
        
        // Test count
        let iter2 = vec.iter();
        assert_eq!(iter2.count(), 3);
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
    
    // Property-based tests for Result type
    #[test]
    fn test_result_properties() {
        // Property: Result::Ok should always be Ok
        let ok_values = [0, 1, -1, 42, i32::MAX, i32::MIN];
        for &value in &ok_values {
            let result: OvieResult<i32, String> = ok(value);
            assert!(result.is_ok());
            assert!(!result.is_err());
            assert_eq!(result.unwrap(), value);
        }
        
        // Property: Result::Err should always be Err
        let err_values = ["", "error", "test", "long error message"];
        for &error in &err_values {
            let result: OvieResult<i32, String> = err(error.to_string());
            assert!(!result.is_ok());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), error.to_string());
        }
        
        // Property: map preserves Ok/Err status
        let test_cases = [
            (ok(5), true),
            (err("error".to_string()), false),
        ];
        
        for (result, should_be_ok) in test_cases {
            let mapped = result.map(|x| x * 2);
            assert_eq!(mapped.is_ok(), should_be_ok);
            if should_be_ok {
                assert_eq!(mapped.unwrap(), 10);
            }
        }
        
        // Property: and_then chains correctly
        let chain_test = ok(5)
            .and_then(|x| ok(x * 2))
            .and_then(|x| ok(x + 1))
            .and_then(|x| if x > 10 { ok(x) } else { err("too small".to_string()) });
        
        assert!(chain_test.is_ok());
        assert_eq!(chain_test.unwrap(), 11);
        
        // Property: error propagation in chain
        let error_chain: OvieResult<i32, String> = ok(5)
            .and_then(|_| err("first error".to_string()))
            .and_then(|x: i32| ok(x * 2)); // Should not execute
        
        assert!(error_chain.is_err());
        assert_eq!(error_chain.unwrap_err(), "first error".to_string());
    }
    
    #[test]
    fn test_result_unwrap_or_properties() {
        // Property: unwrap_or returns value for Ok
        let test_values = [0, 1, -1, 42, 100];
        for &value in &test_values {
            let result: OvieResult<i32, String> = ok(value);
            assert_eq!(result.unwrap_or(999), value);
        }
        
        // Property: unwrap_or returns default for Err
        let default_values = [0, 1, -1, 42, 100];
        for &default in &default_values {
            let result: OvieResult<i32, String> = err("error".to_string());
            assert_eq!(result.unwrap_or(default), default);
        }
    }
    
    #[test]
    fn test_result_map_properties() {
        // Property: map on Ok applies function
        let inputs = [1, 2, 5, 10, -3];
        for &input in &inputs {
            let result: OvieResult<i32, String> = ok(input).map(|x| x * x);
            assert_eq!(result, ok(input * input));
        }
        
        // Property: map on Err preserves error
        let errors = ["error1", "error2", "test"];
        for &error in &errors {
            let result: OvieResult<i32, String> = err(error.to_string());
            let mapped = result.map(|x| x * 2);
            assert_eq!(mapped, err(error.to_string()));
        }
        
        // Property: map_err on Err applies function
        for &error in &errors {
            let result: OvieResult<i32, String> = err(error.to_string());
            let mapped = result.map_err(|e| format!("Error: {}", e));
            assert_eq!(mapped, err(format!("Error: {}", error)));
        }
        
        // Property: map_err on Ok preserves value
        for &input in &inputs {
            let result: OvieResult<i32, String> = ok(input);
            let mapped = result.map_err(|e| format!("Error: {}", e));
            assert_eq!(mapped, ok(input));
        }
    }
    
    // Property-based tests for Option type
    #[test]
    fn test_option_properties() {
        // Property: Option::Some should always be Some
        let some_values = [0, 1, -1, 42, i32::MAX, i32::MIN];
        for &value in &some_values {
            let option = some(value);
            assert!(option.is_some());
            assert!(!option.is_none());
            assert_eq!(option.unwrap(), value);
        }
        
        // Property: Option::None should always be None
        let none_option: OvieOption<i32> = none();
        assert!(!none_option.is_some());
        assert!(none_option.is_none());
        
        // Property: map preserves Some/None status
        let test_cases = [
            (some(5), true),
            (none(), false),
        ];
        
        for (option, should_be_some) in test_cases {
            let mapped = option.map(|x| x * 2);
            assert_eq!(mapped.is_some(), should_be_some);
            if should_be_some {
                assert_eq!(mapped.unwrap(), 10);
            }
        }
        
        // Property: and_then chains correctly
        let chain_test = some(5)
            .and_then(|x| some(x * 2))
            .and_then(|x| some(x + 1))
            .and_then(|x| if x > 10 { some(x) } else { none() });
        
        assert!(chain_test.is_some());
        assert_eq!(chain_test.unwrap(), 11);
        
        // Property: None propagation in chain
        let none_chain: OvieOption<i32> = some(5)
            .and_then(|_| none())
            .and_then(|x: i32| some(x * 2)); // Should not execute
        
        assert!(none_chain.is_none());
    }
    
    #[test]
    fn test_option_unwrap_or_properties() {
        // Property: unwrap_or returns value for Some
        let test_values = [0, 1, -1, 42, 100];
        for &value in &test_values {
            let option = some(value);
            assert_eq!(option.unwrap_or(999), value);
        }
        
        // Property: unwrap_or returns default for None
        let default_values = [0, 1, -1, 42, 100];
        for &default in &default_values {
            let option: OvieOption<i32> = none();
            assert_eq!(option.unwrap_or(default), default);
        }
    }
    
    #[test]
    fn test_option_ok_or_properties() {
        // Property: ok_or converts Some to Ok
        let test_values = [0, 1, -1, 42, 100];
        for &value in &test_values {
            let option = some(value);
            let result = option.ok_or("error".to_string());
            assert_eq!(result, ok(value));
        }
        
        // Property: ok_or converts None to Err
        let error_values = ["error1", "error2", "test"];
        for &error in &error_values {
            let option: OvieOption<i32> = none();
            let result = option.ok_or(error.to_string());
            assert_eq!(result, err(error.to_string()));
        }
    }
    
    // Property-based tests for Vec type
    #[test]
    fn test_vec_properties() {
        // Property: new vec is empty
        let vec: OvieVec<i32> = OvieVec::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
        
        // Property: with_capacity creates empty vec with capacity
        let capacities = [0, 1, 5, 10, 100];
        for &cap in &capacities {
            let vec: OvieVec<i32> = OvieVec::with_capacity(cap);
            assert!(vec.is_empty());
            assert_eq!(vec.len(), 0);
            assert!(vec.capacity() >= cap);
        }
        
        // Property: push increases length
        let mut vec = OvieVec::new();
        let values = [1, 2, 3, 4, 5];
        for (i, &value) in values.iter().enumerate() {
            vec.push(value);
            assert_eq!(vec.len(), i + 1);
            assert!(!vec.is_empty());
            assert_eq!(vec.get(i), some(value));
        }
        
        // Property: pop decreases length and returns last element
        let mut vec = OvieVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert_eq!(vec.pop(), some(3));
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.pop(), some(2));
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.pop(), some(1));
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.pop(), none());
        
        // Property: get returns None for out-of-bounds
        let vec: OvieVec<i32> = OvieVec::new();
        assert_eq!(vec.get(0), none());
        assert_eq!(vec.get(100), none());
        
        let mut vec = OvieVec::new();
        vec.push(42);
        assert_eq!(vec.get(0), some(42));
        assert_eq!(vec.get(1), none());
    }
    
    #[test]
    fn test_vec_insert_remove_properties() {
        // Property: insert at valid index succeeds
        let mut vec = OvieVec::new();
        vec.push(1);
        vec.push(3);
        
        assert!(vec.insert(1, 2).is_ok());
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(0), some(1));
        assert_eq!(vec.get(1), some(2));
        assert_eq!(vec.get(2), some(3));
        
        // Property: insert at invalid index fails
        assert!(vec.insert(10, 99).is_err());
        assert_eq!(vec.len(), 3); // Length unchanged
        
        // Property: remove at valid index succeeds
        assert_eq!(vec.remove(1), ok(2));
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.get(0), some(1));
        assert_eq!(vec.get(1), some(3));
        
        // Property: remove at invalid index fails
        assert!(vec.remove(10).is_err());
        assert_eq!(vec.len(), 2); // Length unchanged
    }
    
    #[test]
    fn test_vec_set_properties() {
        // Property: set at valid index succeeds
        let mut vec = OvieVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert!(vec.set(1, 99).is_ok());
        assert_eq!(vec.get(1), some(99));
        assert_eq!(vec.len(), 3); // Length unchanged
        
        // Property: set at invalid index fails
        assert!(vec.set(10, 99).is_err());
        assert_eq!(vec.len(), 3); // Length unchanged
    }
    
    #[test]
    fn test_vec_iterator_properties() {
        // Property: iterator visits all elements in order
        let mut vec = OvieVec::new();
        let values = [1, 2, 3, 4, 5];
        for &value in &values {
            vec.push(value);
        }
        
        let mut iter = vec.iter();
        for &expected in &values {
            assert_eq!(iter.next_legacy(), some(expected));
        }
        assert_eq!(iter.next_legacy(), none());
        
        // Property: iterator count matches vec length
        let iter = vec.iter();
        assert_eq!(iter.count_legacy(), vec.len());
        
        // Property: collect recreates the vector
        let iter = vec.iter();
        let collected = iter.collect_legacy();
        assert_eq!(collected.len(), vec.len());
        for i in 0..vec.len() {
            assert_eq!(collected.get(i), vec.get(i));
        }
    }
}

    
    // Property-based tests for HashMap type
    #[test]
    fn test_hashmap_properties() {
        // Property: new hashmap is empty
        let map: OvieHashMap<String, i32> = OvieHashMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        
        // Property: insert increases length
        let mut map = OvieHashMap::new();
        let pairs = [("a", 1), ("b", 2), ("c", 3)];
        for (i, &(key, value)) in pairs.iter().enumerate() {
            map.insert(key.to_string(), value);
            assert_eq!(map.len(), i + 1);
            assert!(!map.is_empty());
            assert_eq!(map.get(&key.to_string()), some(value));
        }
        
        // Property: insert with existing key replaces value
        let mut map = OvieHashMap::new();
        assert_eq!(map.insert("key".to_string(), 1), none());
        assert_eq!(map.len(), 1);
        assert_eq!(map.insert("key".to_string(), 2), some(1));
        assert_eq!(map.len(), 1); // Length unchanged
        assert_eq!(map.get(&"key".to_string()), some(2));
        
        // Property: remove decreases length
        let mut map = OvieHashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);
        
        assert_eq!(map.remove(&"b".to_string()), some(2));
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&"b".to_string()), none());
        
        // Property: remove non-existent key returns None
        assert_eq!(map.remove(&"nonexistent".to_string()), none());
        assert_eq!(map.len(), 2); // Length unchanged
    }
    
    #[test]
    fn test_hashmap_contains_key_properties() {
        // Property: contains_key returns true for inserted keys
        let mut map = OvieHashMap::new();
        let keys = ["a", "b", "c", "d", "e"];
        
        for &key in &keys {
            map.insert(key.to_string(), 1);
            assert!(map.contains_key(&key.to_string()));
        }
        
        // Property: contains_key returns false for non-existent keys
        assert!(!map.contains_key(&"nonexistent".to_string()));
        assert!(!map.contains_key(&"z".to_string()));
        
        // Property: contains_key returns false after removal
        map.remove(&"a".to_string());
        assert!(!map.contains_key(&"a".to_string()));
    }
    
    #[test]
    fn test_hashmap_deterministic_iteration() {
        // Property: iteration order matches insertion order
        let mut map = OvieHashMap::new();
        let pairs = [("first", 1), ("second", 2), ("third", 3), ("fourth", 4)];
        
        for &(key, value) in &pairs {
            map.insert(key.to_string(), value);
        }
        
        let mut iter = map.iter();
        for &(expected_key, expected_value) in &pairs {
            let (key, value) = iter.next_legacy().unwrap();
            assert_eq!(key, expected_key.to_string());
            assert_eq!(value, expected_value);
        }
        assert_eq!(iter.next_legacy(), none());
        
        // Property: keys iterator returns keys in insertion order
        let mut keys_iter = map.keys();
        for &(expected_key, _) in &pairs {
            assert_eq!(keys_iter.next_legacy(), some(expected_key.to_string()));
        }
        assert_eq!(keys_iter.next_legacy(), none());
        
        // Property: values iterator returns values in insertion order
        let mut values_iter = map.values();
        for &(_, expected_value) in &pairs {
            assert_eq!(values_iter.next_legacy(), some(expected_value));
        }
        assert_eq!(values_iter.next_legacy(), none());
    }
    
    #[test]
    fn test_hashmap_try_insert_properties() {
        // Property: try_insert succeeds for new keys
        let mut map = OvieHashMap::new();
        assert!(map.try_insert("key1".to_string(), 1).is_ok());
        assert_eq!(map.get(&"key1".to_string()), some(1));
        
        // Property: try_insert fails for existing keys
        let result = map.try_insert("key1".to_string(), 2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 2); // Returns the value that wasn't inserted
        assert_eq!(map.get(&"key1".to_string()), some(1)); // Original value unchanged
    }
    
    #[test]
    fn test_hashmap_get_mut_properties() {
        // Property: get_mut allows modification
        let mut map = OvieHashMap::new();
        map.insert("key".to_string(), 42);
        
        if let OvieOption::Some(value) = map.get_mut(&"key".to_string()) {
            *value = 84;
        }
        
        assert_eq!(map.get(&"key".to_string()), some(84));
        
        // Property: get_mut returns None for non-existent keys
        assert_eq!(map.get_mut(&"nonexistent".to_string()), none());
    }
    
    #[test]
    fn test_hashmap_clear_properties() {
        // Property: clear removes all elements
        let mut map = OvieHashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);
        
        assert_eq!(map.len(), 3);
        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        assert_eq!(map.get(&"a".to_string()), none());
        assert_eq!(map.get(&"b".to_string()), none());
        assert_eq!(map.get(&"c".to_string()), none());
    }
    
    // Property-based tests for Iterator trait
    #[test]
    fn test_iterator_trait_properties() {
        // Property: map transforms all elements
        let mut vec = OvieVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        let mapped: OvieVec<i32> = vec.iter().map(|x| x * 2).collect();
        assert_eq!(mapped.len(), 3);
        assert_eq!(mapped.get(0), some(2));
        assert_eq!(mapped.get(1), some(4));
        assert_eq!(mapped.get(2), some(6));
        
        // Property: filter keeps only matching elements
        let mut vec = OvieVec::new();
        for i in 1..=10 {
            vec.push(i);
        }
        
        let filtered: OvieVec<i32> = vec.iter().filter(|&x| x % 2 == 0).collect();
        assert_eq!(filtered.len(), 5);
        assert_eq!(filtered.get(0), some(2));
        assert_eq!(filtered.get(1), some(4));
        assert_eq!(filtered.get(2), some(6));
        assert_eq!(filtered.get(3), some(8));
        assert_eq!(filtered.get(4), some(10));
        
        // Property: chaining map and filter
        let chained: OvieVec<i32> = vec.iter()
            .filter(|&x| x % 2 == 0)
            .map(|x| x * 3)
            .collect();
        
        assert_eq!(chained.len(), 5);
        assert_eq!(chained.get(0), some(6));   // 2 * 3
        assert_eq!(chained.get(1), some(12));  // 4 * 3
        assert_eq!(chained.get(2), some(18));  // 6 * 3
        assert_eq!(chained.get(3), some(24));  // 8 * 3
        assert_eq!(chained.get(4), some(30));  // 10 * 3
    }
    
    #[test]
    fn test_iterator_find_properties() {
        // Property: find returns first matching element
        let mut vec = OvieVec::new();
        for i in 1..=10 {
            vec.push(i);
        }
        
        let found = vec.iter().find(|&x| x > 5);
        assert_eq!(found, some(6));
        
        let not_found = vec.iter().find(|&x| x > 10);
        assert_eq!(not_found, none());
        
        // Property: find with no matches returns None
        let empty_vec: OvieVec<i32> = OvieVec::new();
        let result = empty_vec.iter().find(|&x| x == 1);
        assert_eq!(result, none());
    }
    
    #[test]
    fn test_iterator_any_all_properties() {
        // Property: any returns true if at least one element matches
        let mut vec = OvieVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        
        assert!(vec.iter().any(|x| x > 3));
        assert!(!vec.iter().any(|x| x > 10));
        
        // Property: all returns true if all elements match
        assert!(vec.iter().all(|x| x > 0));
        assert!(!vec.iter().all(|x| x > 3));
        
        // Property: empty iterator
        let empty_vec: OvieVec<i32> = OvieVec::new();
        assert!(!empty_vec.iter().any(|x| x > 0));  // any on empty is false
        assert!(empty_vec.iter().all(|x| x > 0));   // all on empty is true
    }
    
    #[test]
    fn test_iterator_fold_properties() {
        // Property: fold accumulates values
        let mut vec = OvieVec::new();
        for i in 1..=5 {
            vec.push(i);
        }
        
        let sum = vec.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5
        
        let product = vec.iter().fold(1, |acc, x| acc * x);
        assert_eq!(product, 120); // 1 * 2 * 3 * 4 * 5
        
        // Property: fold on empty iterator returns initial value
        let empty_vec: OvieVec<i32> = OvieVec::new();
        let result = empty_vec.iter().fold(42, |acc, x| acc + x);
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_iterator_count_properties() {
        // Property: count returns number of elements
        let mut vec = OvieVec::new();
        for i in 1..=10 {
            vec.push(i);
        }
        
        assert_eq!(vec.iter().count(), 10);
        
        // Property: count after filter
        let even_count = vec.iter().filter(|&x| x % 2 == 0).count();
        assert_eq!(even_count, 5);
        
        // Property: count on empty iterator
        let empty_vec: OvieVec<i32> = OvieVec::new();
        assert_eq!(empty_vec.iter().count(), 0);
    }