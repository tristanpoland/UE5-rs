//! UE5-style dynamic array (TArray)

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// UE5-style dynamic array (equivalent to TArray)
/// 
/// A growable array with UE5-compatible methods and naming conventions.
/// Internally uses Vec<T> but provides UE5-style API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TArray<T> {
    data: Vec<T>,
}

impl<T: fmt::Display> fmt::Display for TArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TArray[")?;
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> BinarySerializable for TArray<T> {}

impl<T> TArray<T> {
    /// Create a new empty array
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Create an array with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }

    /// Create an array from a Vec
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { data: vec }
    }

    /// Convert to Vec
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Get the number of elements (UE5: Num())
    pub fn num(&self) -> i32 {
        self.data.len() as i32
    }

    /// Get the number of elements as usize
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Add an element to the end (UE5: Add())
    pub fn add(&mut self, item: T) -> i32 {
        self.data.push(item);
        (self.data.len() - 1) as i32
    }

    /// Add an element using push semantics
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// Remove and return the last element (UE5: Pop())
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Insert an element at the given index (UE5: Insert())
    pub fn insert(&mut self, index: i32, item: T) {
        if index >= 0 && (index as usize) <= self.data.len() {
            self.data.insert(index as usize, item);
        }
    }

    /// Remove an element at the given index (UE5: RemoveAt())
    pub fn remove_at(&mut self, index: i32) -> Option<T> {
        if index >= 0 && (index as usize) < self.data.len() {
            Some(self.data.remove(index as usize))
        } else {
            None
        }
    }

    /// Remove the first occurrence of an item (UE5: Remove())
    pub fn remove(&mut self, item: &T) -> bool 
    where 
        T: PartialEq 
    {
        if let Some(pos) = self.data.iter().position(|x| x == item) {
            self.data.remove(pos);
            true
        } else {
            false
        }
    }

    /// Remove all elements (UE5: Empty())
    pub fn empty(&mut self) {
        self.data.clear();
    }

    /// Clear all elements
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Reserve space for at least additional elements (UE5: Reserve())
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Set the number of elements, filling with default values if growing
    pub fn set_num(&mut self, new_size: i32) 
    where 
        T: Default + Clone 
    {
        if new_size >= 0 {
            self.data.resize(new_size as usize, T::default());
        }
    }

    /// Get an element by index (UE5: operator[])
    pub fn get(&self, index: i32) -> Option<&T> {
        if index >= 0 && (index as usize) < self.data.len() {
            Some(&self.data[index as usize])
        } else {
            None
        }
    }

    /// Get a mutable reference to an element by index
    pub fn get_mut(&mut self, index: i32) -> Option<&mut T> {
        if index >= 0 && (index as usize) < self.data.len() {
            Some(&mut self.data[index as usize])
        } else {
            None
        }
    }

    /// Check if the array contains an item (UE5: Contains())
    pub fn contains(&self, item: &T) -> bool 
    where 
        T: PartialEq 
    {
        self.data.contains(item)
    }

    /// Find the index of an item (UE5: Find())
    pub fn find(&self, item: &T) -> i32 
    where 
        T: PartialEq 
    {
        self.data.iter().position(|x| x == item).map_or(-1, |pos| pos as i32)
    }

    /// Check if a valid index (UE5: IsValidIndex())
    pub fn is_valid_index(&self, index: i32) -> bool {
        index >= 0 && (index as usize) < self.data.len()
    }

    /// Get the last element (UE5: Last())
    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    /// Get a mutable reference to the last element
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    /// Append another array (UE5: Append())
    pub fn append(&mut self, other: &mut TArray<T>) {
        self.data.append(&mut other.data);
    }

    /// Sort the array in place
    pub fn sort(&mut self) 
    where 
        T: Ord 
    {
        self.data.sort();
    }

    /// Sort by a key function
    pub fn sort_by_key<K, F>(&mut self, f: F) 
    where 
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.data.sort_by_key(f);
    }

    /// Get an iterator over the elements
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    /// Get a mutable iterator over the elements
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }

    /// Convert to slice
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Convert to mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Default for TArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for TArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for TArray<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a TArray<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tarray_basic_operations() {
        let mut arr = TArray::new();
        assert_eq!(arr.num(), 0);
        assert!(arr.is_empty());

        arr.add(1);
        arr.add(2);
        arr.add(3);
        
        assert_eq!(arr.num(), 3);
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(2), Some(&3));
        assert_eq!(arr.last(), Some(&3));
    }

    #[test]
    fn test_tarray_remove_operations() {
        let mut arr = TArray::from_vec(vec![1, 2, 3, 2, 4]);
        
        assert!(arr.remove(&2)); // Remove first occurrence
        assert_eq!(arr.as_slice(), &[1, 3, 2, 4]);
        
        assert_eq!(arr.remove_at(2), Some(2));
        assert_eq!(arr.as_slice(), &[1, 3, 4]);
    }

    #[test]
    fn test_tarray_find_and_contains() {
        let arr = TArray::from_vec(vec!["hello", "world", "test"]);
        
        assert!(arr.contains(&"world"));
        assert!(!arr.contains(&"missing"));
        assert_eq!(arr.find(&"world"), 1);
        assert_eq!(arr.find(&"missing"), -1);
    }

    #[test]
    fn test_tarray_display() {
        let arr = TArray::from_vec(vec![1, 2, 3]);
        let display_str = format!("{}", arr);
        assert_eq!(display_str, "TArray[1, 2, 3]");
    }

    #[test]
    fn test_serialization() {
        let arr = TArray::from_vec(vec![1, 2, 3]);
        
        let json = serde_json::to_string(&arr).unwrap();
        let deserialized: TArray<i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(arr, deserialized);
        
        let binary = arr.to_binary().unwrap();
        let deserialized = TArray::from_binary(&binary).unwrap();
        assert_eq!(arr, deserialized);
    }

    #[test]
    fn test_tarray_capacity_and_reallocation() {
        let mut arr = TArray::with_capacity(2);
        assert_eq!(arr.capacity(), 2);
        assert_eq!(arr.num(), 0);
        
        // Add elements up to capacity
        arr.add(1);
        arr.add(2);
        assert_eq!(arr.num(), 2);
        assert_eq!(arr.capacity(), 2);
        
        // Force reallocation
        arr.add(3);
        arr.add(4);
        
        assert_eq!(arr.num(), 4);
        assert!(arr.capacity() >= 4); // Should have grown
        
        // Verify all elements are still there
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(2), Some(&3));
        assert_eq!(arr.get(3), Some(&4));
    }

    #[test]
    fn test_tarray_large_operations() {
        let mut arr = TArray::new();
        
        // Add many items to test reallocation and performance
        for i in 0..1000 {
            arr.add(i);
        }
        
        assert_eq!(arr.num(), 1000);
        
        // Test finding items in large array
        assert_eq!(arr.find(&500), 500);
        assert_eq!(arr.find(&999), 999);
        assert_eq!(arr.find(&1000), -1); // Not found
        
        // Test removing from middle
        assert_eq!(arr.remove_at(500), Some(500));
        assert_eq!(arr.num(), 999);
        assert_eq!(arr.get(500), Some(&501)); // Element shifted down
        
        // Test contains on large array
        assert!(arr.contains(&0));
        assert!(arr.contains(&999));
        assert!(!arr.contains(&500)); // Was removed
    }

    #[test]
    fn test_tarray_edge_cases() {
        let mut arr = TArray::new();
        
        // Test operations on empty array
        assert_eq!(arr.get(0), None);
        assert_eq!(arr.last(), None);
        assert_eq!(arr.remove_at(0), None);
        assert_eq!(arr.find(&42), -1);
        assert!(!arr.contains(&42));
        assert!(arr.is_empty());
        
        // Test with single element
        arr.add(42);
        assert!(!arr.is_empty());
        assert_eq!(arr.num(), 1);
        assert_eq!(arr.get(0), Some(&42));
        assert_eq!(arr.last(), Some(&42));
        assert!(arr.contains(&42));
        assert_eq!(arr.find(&42), 0);
        
        // Test removing the only element
        assert_eq!(arr.remove_at(0), Some(42));
        assert!(arr.is_empty());
        assert_eq!(arr.num(), 0);
    }

    #[test] 
    fn test_tarray_index_bounds() {
        let mut arr = TArray::from_vec(vec![1, 2, 3]);
        
        // Test valid indices
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(2), Some(&3));
        
        // Test invalid indices
        assert_eq!(arr.get(3), None);
        assert_eq!(arr.get(100), None);
        
        // Test negative index handling in remove_at
        assert_eq!(arr.remove_at(-1), None);
        assert_eq!(arr.remove_at(100), None);
        
        // Array should be unchanged
        assert_eq!(arr.num(), 3);
        assert_eq!(arr.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_tarray_memory_efficiency() {
        // Test that TArray doesn't waste too much memory  
        let arr = TArray::with_capacity(10);
        assert_eq!(arr.capacity(), 10);
        assert_eq!(arr.num(), 0);
        
        // After adding one element, capacity should remain the same
        let mut arr = arr;
        arr.add(1);
        assert_eq!(arr.capacity(), 10);
        assert_eq!(arr.num(), 1);
        
        // Test shrinking (if supported)
        let mut large_arr = TArray::with_capacity(1000);
        large_arr.add(1);
        large_arr.add(2);
        
        // Empty should clear all elements
        large_arr.empty();
        assert_eq!(large_arr.num(), 0);
        assert!(large_arr.is_empty());
    }

    #[test]
    fn test_tarray_with_complex_types() {
        use crate::types::Vector;
        
        let mut arr = TArray::new();
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = Vector::new(7.0, 8.0, 9.0);
        
        arr.add(v1);
        arr.add(v2);
        arr.add(v3);
        
        assert_eq!(arr.num(), 3);
        assert_eq!(arr.get(0), Some(&v1));
        assert_eq!(arr.find(&v2), 1);
        assert!(arr.contains(&v3));
        
        // Test operations with complex types
        let center = Vector::new(5.0, 6.0, 7.0);
        let closest_index = arr.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let dist_a = (center - **a).length();
                let dist_b = (center - **b).length();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(i, _)| i);
        
        assert_eq!(closest_index, Some(1)); // v2 should be closest to center
    }
}