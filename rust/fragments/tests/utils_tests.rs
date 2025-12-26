use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use fragments::utils::{
    async_event::AsyncEvent,
    data_map::DataMap,
    data_set::DataSet,
    event::Event,
    ifc_category_map::ifc_category_name,
    ifc_geometries_map::{is_geometry_type, is_ifc_geometry},
    ifc_relations_map::relation_mapping,
};

#[test]
fn event_add_remove_trigger() {
    let mut event: Event<i32> = Event::new();
    let counter = Rc::new(RefCell::new(0));
    let counter_clone = Rc::clone(&counter);
    let handler_id = event.add(move |value| {
        *counter_clone.borrow_mut() += *value;
    });

    event.trigger(&2);
    event.trigger(&3);
    assert_eq!(*counter.borrow(), 5);

    event.remove(handler_id);
    event.trigger(&4);
    assert_eq!(*counter.borrow(), 5);
}

#[test]
fn data_set_guard_and_events() {
    let mut set: DataSet<i32> = DataSet::new();
    set.guard = Box::new(|value| *value % 2 == 0);

    let added = Rc::new(RefCell::new(Vec::new()));
    let added_clone = Rc::clone(&added);
    set.on_item_added.add(move |value| {
        added_clone.borrow_mut().push(*value);
    });

    set.add(vec![1, 2, 2, 4]);
    assert!(set.contains(&2));
    assert!(set.contains(&4));
    assert_eq!(set.len(), 2);
    assert_eq!(added.borrow().as_slice(), &[2, 4]);
    assert_eq!(set.get_index(&4), 1);

    assert!(set.delete(&2));
    assert_eq!(set.len(), 1);
}

#[test]
fn data_map_set_update_replace() {
    let mut map: DataMap<String, i32> = DataMap::new();
    let updates = Rc::new(RefCell::new(Vec::new()));
    let updates_clone = Rc::clone(&updates);
    map.on_item_updated.add(move |payload| {
        updates_clone
            .borrow_mut()
            .push((payload.key.clone(), payload.value));
    });

    map.set("a".to_string(), 1);
    map.set("a".to_string(), 2);
    assert_eq!(updates.borrow().as_slice(), &[("a".to_string(), 2)]);

    assert!(map.replace_key(&"a".to_string(), "b".to_string(), false));
    assert!(map.contains_key(&"b".to_string()));
    assert_eq!(map.len(), 1);
}

#[test]
fn async_event_triggers_in_order() {
    let mut event: AsyncEvent<i32> = AsyncEvent::new();
    let calls: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let calls_first = Rc::clone(&calls);
    event.add(move |value| {
        let calls_first = Rc::clone(&calls_first);
        async move {
            calls_first.borrow_mut().push(value + 1);
        }
    });
    let calls_second = Rc::clone(&calls);
    event.add(move |value| {
        let calls_second = Rc::clone(&calls_second);
        async move {
            calls_second.borrow_mut().push(value + 2);
        }
    });

    block_on(event.trigger(&1));
    assert_eq!(calls.borrow().as_slice(), &[2, 3]);
}

#[test]
fn ifc_maps_resolve_known_entries() {
    assert_eq!(ifc_category_name(950732822), Some("IFCURIREFERENCE"));

    let relation = relation_mapping(160246688).expect("expected relation mapping");
    assert_eq!(relation.for_relating, "IsDecomposedBy");
    assert_eq!(relation.for_related, "Decomposes");

    assert!(is_ifc_geometry(1123145078));
    assert!(is_geometry_type(1123145078));
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = unsafe { Waker::from_raw(raw_waker()) };
    let mut context = Context::from_waker(&waker);
    let mut future = unsafe { Pin::new_unchecked(&mut future) };
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => {}
        }
    }
}

fn raw_waker() -> RawWaker {
    fn clone(_: *const ()) -> RawWaker {
        raw_waker()
    }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}

    RawWaker::new(
        std::ptr::null(),
        &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
    )
}
