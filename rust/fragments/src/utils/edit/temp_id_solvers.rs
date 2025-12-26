use std::collections::HashMap;

use super::types::{EditRequest, Id, RawGlobalTransformData, SampleRequestData};

pub fn solve_gt_temp_id(
    sample: &mut RawGlobalTransformData,
    temp_ids_to_local_ids: &HashMap<String, u32>,
) {
    if let Id::Text(text) = &sample.item_id {
        let local_id = temp_ids_to_local_ids
            .get(text)
            .unwrap_or_else(|| panic!("Malformed request: temp id {text} not found"));
        sample.item_id = Id::Number(*local_id);
    }
}

pub fn solve_sample_temp_id(
    sample: &mut SampleRequestData,
    temp_ids_to_local_ids: &HashMap<String, u32>,
) {
    for id in [
        &mut sample.item,
        &mut sample.material,
        &mut sample.representation,
        &mut sample.local_transform,
    ] {
        if let Id::Text(text) = id {
            let local_id = temp_ids_to_local_ids
                .get(text)
                .unwrap_or_else(|| panic!("Malformed request: temp id {text} not found"));
            *id = Id::Number(*local_id);
        }
    }
}

pub fn solve_local_id_temp_id(
    request: &mut EditRequest,
    temp_ids_to_local_ids: &HashMap<String, u32>,
) {
    if let Some(id) = request.local_id_mut() {
        if let Id::Text(text) = id {
            let local_id = temp_ids_to_local_ids
                .get(text)
                .unwrap_or_else(|| panic!("Malformed request: temp id {text} not found"));
            *id = Id::Number(*local_id);
        }
    }
}
