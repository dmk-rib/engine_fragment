use std::collections::HashMap;
use std::hash::Hash;

use crate::utils::event::Event;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapItem<K, V> {
    pub key: K,
    pub value: V,
}

pub struct DataMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    entries: Vec<(K, V)>,
    index: HashMap<K, usize>,
    pub on_item_set: Event<MapItem<K, V>>,
    pub on_item_updated: Event<MapItem<K, V>>,
    pub on_item_deleted: Event<K>,
    pub on_before_delete: Event<MapItem<K, V>>,
    pub on_cleared: Event<()>,
    pub guard: Box<dyn FnMut(&K, &V) -> bool>,
}

impl<K, V> Default for DataMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> DataMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: HashMap::new(),
            on_item_set: Event::new(),
            on_item_updated: Event::new(),
            on_item_deleted: Event::new(),
            on_before_delete: Event::new(),
            on_cleared: Event::new(),
            guard: Box::new(|_, _| true),
        }
    }

    pub fn with_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = Self::new();
        for (key, value) in iter {
            map.entries.push((key.clone(), value));
            map.index.insert(key, map.entries.len() - 1);
        }
        map
    }

    pub fn set_events_enabled(&mut self, value: bool) {
        self.on_item_set.enabled = value;
        self.on_item_updated.enabled = value;
        self.on_item_deleted.enabled = value;
        self.on_before_delete.enabled = value;
        self.on_cleared.enabled = value;
    }

    pub fn clear(&mut self) {
        for (key, value) in &self.entries {
            let payload = MapItem {
                key: key.clone(),
                value: value.clone(),
            };
            self.on_before_delete.trigger(&payload);
        }
        self.entries.clear();
        self.index.clear();
        self.on_cleared.trigger(&());
    }

    /// Mirrors TS `set` and triggers the appropriate event.
    pub fn set(&mut self, key: K, value: V) -> &mut Self {
        if !(self.guard)(&key, &value) {
            return self;
        }
        let payload = MapItem {
            key: key.clone(),
            value: value.clone(),
        };
        if let Some(existing_index) = self.index.get(&key).copied() {
            if let Some(entry) = self.entries.get_mut(existing_index) {
                entry.1 = value;
            }
            self.on_item_updated.trigger(&payload);
        } else {
            self.entries.push((key.clone(), value));
            self.index.insert(key, self.entries.len() - 1);
            self.on_item_set.trigger(&payload);
        }
        self
    }

    pub fn delete(&mut self, key: &K) -> bool {
        let index = match self.index.get(key).copied() {
            Some(index) => index,
            None => return false,
        };
        let (entry_key, entry_value) = self.entries[index].clone();
        let payload = MapItem {
            key: entry_key.clone(),
            value: entry_value,
        };
        self.on_before_delete.trigger(&payload);
        self.entries.remove(index);
        self.index.remove(key);
        for (idx, (key, _)) in self.entries.iter().enumerate().skip(index) {
            self.index.insert(key.clone(), idx);
        }
        self.on_item_deleted.trigger(&entry_key);
        true
    }

    pub fn get_key(&self, item: &V) -> Option<K>
    where
        V: PartialEq,
    {
        self.entries.iter().find_map(|(key, value)| {
            if value == item {
                Some(key.clone())
            } else {
                None
            }
        })
    }

    pub fn update(&mut self, item: &V)
    where
        V: PartialEq,
    {
        if let Some(key) = self.get_key(item) {
            self.set(key, item.clone());
        }
    }

    pub fn delete_if<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&V, &K) -> bool,
    {
        let to_remove: Vec<K> = self
            .entries
            .iter()
            .filter(|(key, value)| predicate(value, key))
            .map(|(key, _)| key.clone())
            .collect();
        for key in to_remove {
            self.delete(&key);
        }
    }

    pub fn replace_key(&mut self, old_key: &K, new_key: K, full_replace: bool) -> bool {
        let old_value = match self.index.get(old_key).copied() {
            Some(index) => self.entries[index].1.clone(),
            None => return false,
        };
        if self.index.contains_key(&new_key) && !full_replace {
            return false;
        }
        self.set_events_enabled(false);
        self.delete(old_key);
        self.set_events_enabled(true);
        self.set(new_key, old_value);
        true
    }

    pub fn dispose(&mut self) {
        self.clear();
        self.on_item_set.reset();
        self.on_item_deleted.reset();
        self.on_item_updated.reset();
        self.on_cleared.reset();
        self.on_before_delete.reset();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.index.contains_key(key)
    }
}
