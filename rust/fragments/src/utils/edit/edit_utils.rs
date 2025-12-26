use std::collections::{HashMap, HashSet};

use super::{
    apply_changes_to_ids, apply_changes_to_raw_data, apply_changes_to_special_data, solve_ids,
    EditKey, EditRawData, EditRequest, SpecialData, SpecialKey,
};

pub struct EditUtils;

impl EditUtils {
    pub fn solve_ids(requests: &mut [EditRequest], next_id: u32) -> Vec<u32> {
        solve_ids(requests, next_id)
    }

    pub fn apply_changes_to_raw_data(
        actions: &[EditRequest],
        raw_data: &mut HashMap<u32, EditRawData>,
        key: EditKey,
        filter: Option<&HashSet<u32>>,
    ) {
        apply_changes_to_raw_data(actions, raw_data, key, filter)
    }

    pub fn apply_changes_to_special_data(
        actions: &[EditRequest],
        key: SpecialKey,
    ) -> Option<SpecialData> {
        apply_changes_to_special_data(actions, key)
    }

    pub fn apply_changes_to_ids(
        actions: &[EditRequest],
        ids: impl IntoIterator<Item = u32>,
        key: EditKey,
        add_created_elements: bool,
    ) -> Vec<u32> {
        apply_changes_to_ids(actions, ids, key, add_created_elements)
    }
}
