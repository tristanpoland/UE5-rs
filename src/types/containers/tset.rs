//! UE5-style set (TSet)

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

/// UE5-style set (equivalent to TSet)
/// 
/// A hash set with UE5-compatible methods and naming conventions.
/// Internally uses HashSet<T> but provides UE5-style API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TSet<T: Eq + Hash> {
    data: HashSet<T>,
}

impl<T: fmt::Display + Eq + Hash> fmt::Display for TSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TSet{{")?;
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "}}")
    }
}

impl<T> BinarySerializable for TSet<T> 
where 
    T: serde::Serialize + serde::de::DeserializeOwned + Eq + Hash,
{}

impl<T> TSet<T> 
where 
    T: Eq + Hash,
{
    /// Create a new empty set
    pub fn new() -> Self {
        Self { data: HashSet::new() }
    }

    /// Create a set with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self { data: HashSet::with_capacity(capacity) }
    }

    /// Create from a HashSet
    pub fn from_hash_set(set: HashSet<T>) -> Self {
        Self { data: set }
    }

    /// Convert to HashSet
    pub fn into_hash_set(self) -> HashSet<T> {
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

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Add an element to the set (UE5: Add())
    pub fn add(&mut self, item: T) -> bool {
        self.data.insert(item)
    }

    /// Insert an element into the set
    pub fn insert(&mut self, item: T) -> bool {
        self.data.insert(item)
    }

    /// Remove an element from the set (UE5: Remove())
    pub fn remove(&mut self, item: &T) -> bool {
        self.data.remove(item)
    }

    /// Check if the set contains an element (UE5: Contains())
    pub fn contains(&self, item: &T) -> bool {
        self.data.contains(item)
    }

    /// Clear all elements (UE5: Empty())
    pub fn empty(&mut self) {
        self.data.clear();
    }

    /// Clear all elements
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Reserve space for at least additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Get an iterator over the elements
    pub fn iter(&self) -> std::collections::hash_set::Iter<T> {
        self.data.iter()
    }

    /// Get the union with another set
    pub fn union<'a>(&'a self, other: &'a TSet<T>) -> std::collections::hash_set::Union<'a, T, std::collections::hash_map::RandomState> {
        self.data.union(&other.data)
    }

    /// Get the intersection with another set
    pub fn intersection<'a>(&'a self, other: &'a TSet<T>) -> std::collections::hash_set::Intersection<'a, T, std::collections::hash_map::RandomState> {
        self.data.intersection(&other.data)
    }

    /// Get the difference with another set
    pub fn difference<'a>(&'a self, other: &'a TSet<T>) -> std::collections::hash_set::Difference<'a, T, std::collections::hash_map::RandomState> {
        self.data.difference(&other.data)
    }
}

impl<T> Default for TSet<T> 
where 
    T: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for TSet<T> 
where 
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_hash_set(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tset_basic_operations() {
        let mut set = TSet::new();
        assert_eq!(set.num(), 0);
        assert!(set.is_empty());

        assert!(set.add(1));
        assert!(set.add(2));
        assert!(!set.add(1)); // Already exists
        
        assert_eq!(set.num(), 2);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
        
        assert!(set.remove(&1));
        assert!(!set.remove(&3)); // Doesn't exist
        assert_eq!(set.num(), 1);
    }

    #[test]
    fn test_tset_display() {
        let set: TSet<i32> = TSet::from_iter(vec![1, 2, 3]);
        let display_str = format!("{}", set);
        assert!(display_str.starts_with("TSet{"));
        assert!(display_str.ends_with("}"));
        // Order is not guaranteed in hash sets
        assert!(display_str.contains("1"));
        assert!(display_str.contains("2"));
        assert!(display_str.contains("3"));
    }

    #[test]
    fn test_serialization() {
        let set = TSet::from_iter(vec![1, 2, 3]);
        
        let json = serde_json::to_string(&set).unwrap();
        let deserialized: TSet<i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(set, deserialized);
        
        let binary = set.to_binary().unwrap();
        let deserialized = TSet::from_binary(&binary).unwrap();
        assert_eq!(set, deserialized);
    }
}