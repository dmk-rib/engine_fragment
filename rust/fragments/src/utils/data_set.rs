use std::collections::HashSet;
use std::hash::Hash;

use crate::utils::event::Event;

pub struct DataSet<T>
where
    T: Eq + Hash + Clone,
{
    data: Vec<T>,
    lookup: HashSet<T>,
    pub on_updated: Event<()>,
    pub on_item_added: Event<T>,
    pub on_before_delete: Event<T>,
    pub on_item_deleted: Event<()>,
    pub on_cleared: Event<()>,
    pub guard: Box<dyn FnMut(&T) -> bool>,
}

impl<T> Default for DataSet<T>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DataSet<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            lookup: HashSet::new(),
            on_updated: Event::new(),
            on_item_added: Event::new(),
            on_before_delete: Event::new(),
            on_item_deleted: Event::new(),
            on_cleared: Event::new(),
            guard: Box::new(|_| true),
        }
    }

    pub fn with_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut data = Vec::new();
        let mut lookup = HashSet::new();
        for item in iter {
            if lookup.insert(item.clone()) {
                data.push(item);
            }
        }
        Self {
            data,
            lookup,
            on_updated: Event::new(),
            on_item_added: Event::new(),
            on_before_delete: Event::new(),
            on_item_deleted: Event::new(),
            on_cleared: Event::new(),
            guard: Box::new(|_| true),
        }
    }

    pub fn set_events_enabled(&mut self, value: bool) {
        self.on_updated.enabled = value;
        self.on_item_added.enabled = value;
        self.on_item_deleted.enabled = value;
        self.on_before_delete.enabled = value;
        self.on_cleared.enabled = value;
    }

    /// Adds items from an iterator, mirroring the variadic TS `add`.
    pub fn add<I>(&mut self, values: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
    {
        for value in values {
            if self.lookup.contains(&value) {
                continue;
            }
            let is_valid = (self.guard)(&value);
            if !is_valid {
                continue;
            }
            self.lookup.insert(value.clone());
            self.data.push(value);
            if let Some(item_ref) = self.data.last() {
                self.on_item_added.trigger(item_ref);
            }
        }
        self.on_updated.trigger(&());
        self
    }

    pub fn clear(&mut self) {
        for item in &self.data {
            self.on_before_delete.trigger(item);
        }
        self.data.clear();
        self.lookup.clear();
        self.on_cleared.trigger(&());
        self.on_updated.trigger(&());
    }

    pub fn delete(&mut self, value: &T) -> bool {
        if !self.lookup.contains(value) {
            return false;
        }
        if let Some(index) = self.data.iter().position(|item| item == value) {
            let existing = &self.data[index];
            self.on_before_delete.trigger(existing);
            self.data.remove(index);
            self.lookup.remove(value);
            self.on_item_deleted.trigger(&());
            self.on_updated.trigger(&());
            return true;
        }
        false
    }

    pub fn delete_if<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&T) -> bool,
    {
        let to_remove: Vec<T> = self.data.iter().filter(|v| predicate(v)).cloned().collect();
        for value in to_remove {
            self.delete(&value);
        }
    }

    pub fn get_index(&self, item: &T) -> isize {
        self.data
            .iter()
            .position(|value| value == item)
            .map(|idx| idx as isize)
            .unwrap_or(-1)
    }

    pub fn dispose(&mut self) {
        self.clear();
        self.on_item_added.reset();
        self.on_item_deleted.reset();
        self.on_cleared.reset();
        self.on_before_delete.reset();
        self.on_updated.reset();
    }

    pub fn contains(&self, item: &T) -> bool {
        self.lookup.contains(item)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}
