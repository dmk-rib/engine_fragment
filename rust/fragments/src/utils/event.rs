use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandlerId(pub u64);

impl fmt::Debug for HandlerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandlerId({})", self.0)
    }
}

pub struct Event<T> {
    pub enabled: bool,
    handlers: Vec<(HandlerId, Box<dyn FnMut(&T)>)>,
    next_id: u64,
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Event<T> {
    pub fn new() -> Self {
        Self {
            enabled: true,
            handlers: Vec::new(),
            next_id: 0,
        }
    }

    /// Adds a handler and returns its ID for removal.
    ///
    /// TypeScript `Event.add` returns void; in Rust we return a `HandlerId` so
    /// callers can remove handlers deterministically.
    pub fn add<F>(&mut self, handler: F) -> HandlerId
    where
        F: FnMut(&T) + 'static,
    {
        let id = HandlerId(self.next_id);
        self.next_id += 1;
        self.handlers.push((id, Box::new(handler)));
        id
    }

    /// Removes a handler by ID.
    pub fn remove(&mut self, id: HandlerId) {
        self.handlers.retain(|(handler_id, _)| *handler_id != id);
    }

    pub fn trigger(&mut self, data: &T) {
        if !self.enabled {
            return;
        }
        for (_, handler) in self.handlers.iter_mut() {
            handler(data);
        }
    }

    pub fn reset(&mut self) {
        self.handlers.clear();
    }
}
