//! UE5-style map (TMap)

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

/// UE5-style map (equivalent to TMap)
/// 
/// A hash map with UE5-compatible methods and naming conventions.
/// Internally uses HashMap<K, V> but provides UE5-style API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TMap<K: Eq + Hash, V> {
    data: HashMap<K, V>,
}

impl<K: fmt::Display + Eq + Hash, V: fmt::Display> fmt::Display for TMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TMap{{")?;
        for (i, (k, v)) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, "}}")
    }
}

impl<K, V> BinarySerializable for TMap<K, V> 
where 
    K: serde::Serialize + serde::de::DeserializeOwned + Eq + Hash,
    V: serde::Serialize + serde::de::DeserializeOwned,
{}

impl<K, V> TMap<K, V> 
where 
    K: Eq + Hash,
{
    /// Create a new empty map
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// Create a map with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self { data: HashMap::with_capacity(capacity) }
    }

    /// Create from a HashMap
    pub fn from_hash_map(map: HashMap<K, V>) -> Self {
        Self { data: map }
    }

    /// Convert to HashMap
    pub fn into_hash_map(self) -> HashMap<K, V> {
        self.data
    }

    /// Get the number of key-value pairs (UE5: Num())
    pub fn num(&self) -> i32 {
        self.data.len() as i32
    }

    /// Get the number of key-value pairs as usize
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Add or update a key-value pair (UE5: Add())
    pub fn add(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }

    /// Insert a key-value pair
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }

    /// Remove a key-value pair (UE5: Remove())
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Check if the map contains a key (UE5: Contains())
    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Get a value by key (UE5: Find())
    pub fn find(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Get a mutable reference to a value by key
    pub fn find_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    /// Get a value by key, returning a default if not found
    pub fn find_or_add(&mut self, key: K, default_value: V) -> &mut V 
    where 
        K: Clone,
        V: Clone,
    {
        self.data.entry(key).or_insert(default_value)
    }

    /// Clear all key-value pairs (UE5: Empty())
    pub fn empty(&mut self) {
        self.data.clear();
    }

    /// Clear all key-value pairs
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Reserve space for at least additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Get an iterator over the key-value pairs
    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.data.iter()
    }

    /// Get a mutable iterator over the key-value pairs
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, V> {
        self.data.iter_mut()
    }

    /// Get an iterator over the keys
    pub fn keys(&self) -> std::collections::hash_map::Keys<K, V> {
        self.data.keys()
    }

    /// Get an iterator over the values
    pub fn values(&self) -> std::collections::hash_map::Values<K, V> {
        self.data.values()
    }

    /// Get a mutable iterator over the values
    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<K, V> {
        self.data.values_mut()
    }
}

impl<K, V> Default for TMap<K, V> 
where 
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmap_basic_operations() {
        let mut map = TMap::new();
        assert_eq!(map.num(), 0);
        assert!(map.is_empty());

        map.add("key1", 100);
        map.add("key2", 200);
        
        assert_eq!(map.num(), 2);
        assert_eq!(map.find(&"key1"), Some(&100));
        assert_eq!(map.find(&"key2"), Some(&200));
        assert_eq!(map.find(&"missing"), None);
        
        assert!(map.contains(&"key1"));
        assert!(!map.contains(&"missing"));
    }

    #[test]
    fn test_tmap_remove_and_update() {
        let mut map = TMap::new();
        map.add("test", 42);
        
        assert_eq!(map.remove(&"test"), Some(42));
        assert_eq!(map.remove(&"missing"), None);
        assert!(map.is_empty());
        
        map.add("update", 1);
        let old_value = map.add("update", 2);
        assert_eq!(old_value, Some(1));
        assert_eq!(map.find(&"update"), Some(&2));
    }

    #[test]
    fn test_tmap_display() {
        let mut map = TMap::new();
        map.add("a", 1);
        map.add("b", 2);
        let display_str = format!("{}", map);
        // HashMap ordering is not guaranteed, so just check it contains the right parts
        assert!(display_str.starts_with("TMap{"));
        assert!(display_str.ends_with("}"));
        assert!(display_str.contains("a: 1") || display_str.contains("1: a"));
        assert!(display_str.contains("b: 2") || display_str.contains("2: b"));
    }

    #[test]
    fn test_serialization() {
        let mut map = TMap::new();
        map.add("test".to_string(), 42);
        
        let json = serde_json::to_string(&map).unwrap();
        let deserialized: TMap<String, i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(map, deserialized);
        
        let binary = map.to_binary().unwrap();
        let deserialized = TMap::from_binary(&binary).unwrap();
        assert_eq!(map, deserialized);
    }
}