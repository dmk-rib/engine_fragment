use std::collections::HashMap;

use super::temp_id_solvers::{solve_gt_temp_id, solve_local_id_temp_id, solve_sample_temp_id};
use super::types::{EditRequest, EditRequestType, Id};

pub fn solve_ids(requests: &mut [EditRequest], mut next_id: u32) -> Vec<u32> {
    let mut temp_ids = HashMap::new();
    let mut result = Vec::new();

    for request in requests.iter_mut() {
        if request.local_id().is_some() {
            continue;
        }
        let new_id = next_id;
        next_id += 1;
        if let Some(temp_id) = request.temp_id().map(str::to_string) {
            temp_ids.insert(temp_id, new_id);
        }
        request.set_local_id(Id::Number(new_id));
        result.push(new_id);
    }

    for request in requests.iter_mut() {
        match request.request_type() {
            EditRequestType::UpdateSample | EditRequestType::CreateSample => {
                let sample = request.sample_data_mut().expect("sample request data");
                solve_sample_temp_id(sample, &temp_ids);
            }
            EditRequestType::UpdateGlobalTransform | EditRequestType::CreateGlobalTransform => {
                let data = request
                    .global_transform_data_mut()
                    .expect("global transform request data");
                solve_gt_temp_id(data, &temp_ids);
            }
            _ => {
                solve_local_id_temp_id(request, &temp_ids);
            }
        }
    }

    result
}
