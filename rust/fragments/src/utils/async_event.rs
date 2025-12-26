use std::future::Future;
use std::pin::Pin;

use crate::utils::event::HandlerId;

pub struct AsyncEvent<T>
where
    T: Clone,
{
    pub enabled: bool,
    handlers: Vec<(
        HandlerId,
        Box<dyn FnMut(T) -> Pin<Box<dyn Future<Output = ()> + 'static>>>,
    )>,
    next_id: u64,
}

impl<T> Default for AsyncEvent<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AsyncEvent<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            enabled: true,
            handlers: Vec::new(),
            next_id: 0,
        }
    }

    /// Adds an async handler and returns its ID for removal.
    pub fn add<F, Fut>(&mut self, mut handler: F) -> HandlerId
    where
        F: FnMut(T) -> Fut + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        let id = HandlerId(self.next_id);
        self.next_id += 1;
        self.handlers
            .push((id, Box::new(move |data| Box::pin(handler(data)))));
        id
    }

    pub fn remove(&mut self, id: HandlerId) {
        self.handlers.retain(|(handler_id, _)| *handler_id != id);
    }

    pub async fn trigger(&mut self, data: &T) {
        if !self.enabled {
            return;
        }
        for (_, handler) in self.handlers.iter_mut() {
            handler(data.clone()).await;
        }
    }

    pub fn reset(&mut self) {
        self.handlers.clear();
    }
}
