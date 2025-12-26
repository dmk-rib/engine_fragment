use std::collections::{HashMap, HashSet};

use super::types::{
    EditKey, EditRequest, EditRequestType, RawGlobalTransformData, RawItemData, RawMaterial,
    RawMetadataData, RawRelationData, RawRepresentation, RawSample, RawTransformData,
    SpatialTreeItem,
};

#[derive(Debug, Clone, PartialEq)]
pub enum EditRawData {
    Material(RawMaterial),
    Representation(RawRepresentation),
    Sample(RawSample),
    GlobalTransform(RawGlobalTransformData),
    LocalTransform(RawTransformData),
    Item(RawItemData),
    Relation(RawRelationData),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialData {
    Metadata(RawMetadataData),
    SpatialStructure(SpatialTreeItem),
}

pub fn apply_changes_to_raw_data(
    actions: &[EditRequest],
    raw_data: &mut HashMap<u32, EditRawData>,
    key: EditKey,
    filter: Option<&HashSet<u32>>,
) {
    let create_type = create_type_for_key(key);
    let update_type = update_type_for_key(key);
    let delete_type = delete_type_for_key(key);

    for action in actions {
        let action_type = action.request_type();
        if action_type == create_type || action_type == update_type {
            let local_id = match local_id_number(action) {
                Some(id) => id,
                None => continue,
            };
            if let Some(filter) = filter {
                if !filter.contains(&local_id) {
                    continue;
                }
            }
            if let Some(data) = data_for_key(action, key) {
                raw_data.insert(local_id, data);
            }
            continue;
        }
        if action_type == delete_type {
            if let Some(local_id) = local_id_number(action) {
                raw_data.remove(&local_id);
            }
        }
    }
}

pub fn apply_changes_to_special_data(
    actions: &[EditRequest],
    key: SpecialKey,
) -> Option<SpecialData> {
    let update_type = match key {
        SpecialKey::Metadata => EditRequestType::UpdateMetadata,
        SpecialKey::SpatialStructure => EditRequestType::UpdateSpatialStructure,
    };

    for action in actions.iter().rev() {
        if action.request_type() == update_type {
            return match action {
                EditRequest::UpdateMetadata(request) => {
                    Some(SpecialData::Metadata(request.data.clone()))
                }
                EditRequest::UpdateSpatialStructure(request) => {
                    Some(SpecialData::SpatialStructure(request.data.clone()))
                }
                _ => None,
            };
        }
    }
    None
}

pub fn apply_changes_to_ids(
    actions: &[EditRequest],
    ids: impl IntoIterator<Item = u32>,
    key: EditKey,
    add_created_elements: bool,
) -> Vec<u32> {
    let mut result_set: HashSet<u32> = ids.into_iter().collect();
    let delete_type = delete_type_for_key(key);
    let create_type = create_type_for_key(key);

    for action in actions {
        let action_type = action.request_type();
        if action_type == delete_type {
            if let Some(local_id) = local_id_number(action) {
                result_set.remove(&local_id);
            }
            continue;
        }
        if add_created_elements && action_type == create_type {
            if let Some(local_id) = local_id_number(action) {
                result_set.insert(local_id);
            }
        }
    }

    result_set.into_iter().collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialKey {
    Metadata,
    SpatialStructure,
}

fn create_type_for_key(key: EditKey) -> EditRequestType {
    match key {
        EditKey::Material => EditRequestType::CreateMaterial,
        EditKey::GlobalTransform => EditRequestType::CreateGlobalTransform,
        EditKey::LocalTransform => EditRequestType::CreateLocalTransform,
        EditKey::Sample => EditRequestType::CreateSample,
        EditKey::Item => EditRequestType::CreateItem,
        EditKey::Representation => EditRequestType::CreateRepresentation,
        EditKey::Relation => EditRequestType::CreateRelation,
    }
}

fn update_type_for_key(key: EditKey) -> EditRequestType {
    match key {
        EditKey::Material => EditRequestType::UpdateMaterial,
        EditKey::GlobalTransform => EditRequestType::UpdateGlobalTransform,
        EditKey::LocalTransform => EditRequestType::UpdateLocalTransform,
        EditKey::Sample => EditRequestType::UpdateSample,
        EditKey::Item => EditRequestType::UpdateItem,
        EditKey::Representation => EditRequestType::UpdateRepresentation,
        EditKey::Relation => EditRequestType::UpdateRelation,
    }
}

fn delete_type_for_key(key: EditKey) -> EditRequestType {
    match key {
        EditKey::Material => EditRequestType::DeleteMaterial,
        EditKey::GlobalTransform => EditRequestType::DeleteGlobalTransform,
        EditKey::LocalTransform => EditRequestType::DeleteLocalTransform,
        EditKey::Sample => EditRequestType::DeleteSample,
        EditKey::Item => EditRequestType::DeleteItem,
        EditKey::Representation => EditRequestType::DeleteRepresentation,
        EditKey::Relation => EditRequestType::DeleteRelation,
    }
}

fn local_id_number(action: &EditRequest) -> Option<u32> {
    match action.local_id() {
        Some(super::types::Id::Number(id)) => Some(*id),
        _ => None,
    }
}

fn data_for_key(action: &EditRequest, key: EditKey) -> Option<EditRawData> {
    match (key, action) {
        (EditKey::Material, EditRequest::CreateMaterial(request)) => {
            Some(EditRawData::Material(request.data.clone()))
        }
        (EditKey::Material, EditRequest::UpdateMaterial(request)) => {
            Some(EditRawData::Material(request.data.clone()))
        }
        (EditKey::Representation, EditRequest::CreateRepresentation(request)) => {
            Some(EditRawData::Representation(request.data.clone()))
        }
        (EditKey::Representation, EditRequest::UpdateRepresentation(request)) => {
            Some(EditRawData::Representation(request.data.clone()))
        }
        (EditKey::Sample, EditRequest::CreateSample(request)) => {
            Some(EditRawData::Sample(RawSample {
                item: id_to_u32(&request.data.item)?,
                material: id_to_u32(&request.data.material)?,
                representation: id_to_u32(&request.data.representation)?,
                local_transform: id_to_u32(&request.data.local_transform)?,
            }))
        }
        (EditKey::Sample, EditRequest::UpdateSample(request)) => {
            Some(EditRawData::Sample(RawSample {
                item: id_to_u32(&request.data.item)?,
                material: id_to_u32(&request.data.material)?,
                representation: id_to_u32(&request.data.representation)?,
                local_transform: id_to_u32(&request.data.local_transform)?,
            }))
        }
        (EditKey::GlobalTransform, EditRequest::CreateGlobalTransform(request)) => {
            Some(EditRawData::GlobalTransform(request.data.clone()))
        }
        (EditKey::GlobalTransform, EditRequest::UpdateGlobalTransform(request)) => {
            Some(EditRawData::GlobalTransform(request.data.clone()))
        }
        (EditKey::LocalTransform, EditRequest::CreateLocalTransform(request)) => {
            Some(EditRawData::LocalTransform(request.data.clone()))
        }
        (EditKey::LocalTransform, EditRequest::UpdateLocalTransform(request)) => {
            Some(EditRawData::LocalTransform(request.data.clone()))
        }
        (EditKey::Item, EditRequest::CreateItem(request)) => {
            Some(EditRawData::Item(request.data.clone()))
        }
        (EditKey::Item, EditRequest::UpdateItem(request)) => {
            Some(EditRawData::Item(request.data.clone()))
        }
        (EditKey::Relation, EditRequest::CreateRelation(request)) => {
            Some(EditRawData::Relation(request.data.clone()))
        }
        (EditKey::Relation, EditRequest::UpdateRelation(request)) => {
            Some(EditRawData::Relation(request.data.clone()))
        }
        _ => None,
    }
}

fn id_to_u32(id: &super::types::Id) -> Option<u32> {
    match id {
        super::types::Id::Number(value) => Some(*value),
        _ => None,
    }
}
