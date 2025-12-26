use std::collections::BTreeMap;

use crate::schema::{AxisPartClass, RenderedFaces, ShellType, Stroke};
use crate::utils::flatbuffers_json_converter::FlatbuffersValue;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Id {
    Number(u32),
    Text(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawMaterial {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub rendered_faces: RenderedFaces,
    pub stroke: Stroke,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemAttribute {
    pub value: FlatbuffersValue,
    pub attribute_type: Option<i32>,
}

pub type ItemData = BTreeMap<String, ItemAttribute>;

#[derive(Debug, Clone, PartialEq)]
pub struct RawItemData {
    pub data: ItemData,
    pub category: String,
    pub guid: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawRelationData {
    pub data: BTreeMap<String, Vec<u32>>,
}

pub type RawMetadataData = BTreeMap<String, FlatbuffersValue>;

#[derive(Debug, Clone, PartialEq)]
pub struct RawTransformData {
    pub position: Vec<f64>,
    pub x_direction: Vec<f64>,
    pub y_direction: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawGlobalTransformData {
    pub transform: RawTransformData,
    pub item_id: Id,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawSample {
    pub item: u32,
    pub material: u32,
    pub representation: u32,
    pub local_transform: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SampleRequestData {
    pub item: Id,
    pub material: Id,
    pub representation: Id,
    pub local_transform: Id,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawCircleCurve {
    pub aperture: f64,
    pub position: Vec<f64>,
    pub radius: f64,
    pub x_direction: Vec<f64>,
    pub y_direction: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawAxis {
    pub wires: Vec<Vec<f64>>,
    pub order: Vec<u32>,
    pub parts: Vec<AxisPartClass>,
    pub wire_sets: Vec<Vec<f64>>,
    pub circle_curves: Vec<RawCircleCurve>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawCircleExtrusion {
    pub radius: Vec<f64>,
    pub axes: Vec<RawAxis>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawShell {
    pub points: Vec<Vec<f64>>,
    pub profiles: BTreeMap<u32, Vec<u32>>,
    pub holes: BTreeMap<u32, Vec<Vec<u32>>>,
    pub big_profiles: BTreeMap<u32, Vec<u32>>,
    pub big_holes: BTreeMap<u32, Vec<Vec<u32>>>,
    pub shell_type: ShellType,
    pub profiles_face_ids: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawGeometry {
    Shell(RawShell),
    CircleExtrusion(RawCircleExtrusion),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawRepresentation {
    pub id: Option<u32>,
    pub bbox: Vec<f64>,
    pub representation_class: u32,
    pub geometry: Option<RawGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementData {
    pub samples: BTreeMap<u32, RawSample>,
    pub local_transforms: BTreeMap<u32, RawTransformData>,
    pub global_transforms: BTreeMap<u32, RawGlobalTransformData>,
    pub representations: BTreeMap<u32, RawRepresentation>,
    pub materials: BTreeMap<u32, RawMaterial>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4 {
    pub elements: [f64; 16],
}

#[derive(Debug, Clone, PartialEq)]
pub struct BufferGeometry;

#[derive(Debug, Clone, PartialEq)]
pub struct MeshLambertMaterial;

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceRef<T> {
    Value(T),
    Id(Id),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewElementSample {
    pub local_transform: ResourceRef<Matrix4>,
    pub representation: ResourceRef<BufferGeometry>,
    pub material: ResourceRef<MeshLambertMaterial>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewElementData {
    pub attributes: ItemData,
    pub global_transform: Matrix4,
    pub samples: Vec<NewElementSample>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditRequestType {
    CreateMaterial,
    CreateRepresentation,
    CreateSample,
    CreateGlobalTransform,
    CreateLocalTransform,
    CreateItem,
    CreateRelation,
    UpdateMaterial,
    UpdateRepresentation,
    UpdateSample,
    UpdateGlobalTransform,
    UpdateLocalTransform,
    UpdateItem,
    UpdateMaxLocalId,
    UpdateRelation,
    UpdateMetadata,
    UpdateSpatialStructure,
    DeleteMaterial,
    DeleteRepresentation,
    DeleteSample,
    DeleteGlobalTransform,
    DeleteLocalTransform,
    DeleteItem,
    DeleteRelation,
}

pub fn edit_request_type_name(request_type: EditRequestType) -> &'static str {
    match request_type {
        EditRequestType::CreateMaterial => "Create Material",
        EditRequestType::CreateRepresentation => "Create Representation",
        EditRequestType::CreateSample => "Create Sample",
        EditRequestType::CreateGlobalTransform => "Create Global Transform",
        EditRequestType::CreateLocalTransform => "Create Local Transform",
        EditRequestType::CreateItem => "Create Item",
        EditRequestType::CreateRelation => "Create Relation",
        EditRequestType::UpdateMaterial => "Update Material",
        EditRequestType::UpdateRepresentation => "Update Representation",
        EditRequestType::UpdateSample => "Update Sample",
        EditRequestType::UpdateGlobalTransform => "Update Global Transform",
        EditRequestType::UpdateLocalTransform => "Update Local Transform",
        EditRequestType::UpdateItem => "Update Item",
        EditRequestType::UpdateMaxLocalId => "Update Max Local Id",
        EditRequestType::UpdateRelation => "Update Relation",
        EditRequestType::UpdateMetadata => "Update Metadata",
        EditRequestType::UpdateSpatialStructure => "Update Spatial Structure",
        EditRequestType::DeleteMaterial => "Delete Material",
        EditRequestType::DeleteRepresentation => "Delete Representation",
        EditRequestType::DeleteSample => "Delete Sample",
        EditRequestType::DeleteGlobalTransform => "Delete Global Transform",
        EditRequestType::DeleteLocalTransform => "Delete Local Transform",
        EditRequestType::DeleteItem => "Delete Item",
        EditRequestType::DeleteRelation => "Delete Relation",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditKey {
    Material,
    GlobalTransform,
    LocalTransform,
    Sample,
    Item,
    Representation,
    Relation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpatialTreeItem {
    pub local_id: Option<u32>,
    pub category: Option<String>,
    pub children: Vec<SpatialTreeItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseEditRequest {
    pub request_type: EditRequestType,
    pub temp_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseUpdateRequest {
    pub base: BaseEditRequest,
    pub local_id: Id,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMaterialRequest {
    pub base: BaseUpdateRequest,
    pub data: RawMaterial,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateRepresentationRequest {
    pub base: BaseUpdateRequest,
    pub data: RawRepresentation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateSampleRequest {
    pub base: BaseUpdateRequest,
    pub data: SampleRequestData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateGlobalTransformRequest {
    pub base: BaseUpdateRequest,
    pub data: RawGlobalTransformData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateLocalTransformRequest {
    pub base: BaseUpdateRequest,
    pub data: RawTransformData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateItemRequest {
    pub base: BaseUpdateRequest,
    pub data: RawItemData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMaxLocalIdRequest {
    pub base: BaseUpdateRequest,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateRelationRequest {
    pub base: BaseUpdateRequest,
    pub data: RawRelationData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMetadataRequest {
    pub base: BaseUpdateRequest,
    pub data: RawMetadataData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateSpatialStructureRequest {
    pub base: BaseUpdateRequest,
    pub data: SpatialTreeItem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseCreateRequest {
    pub base: BaseEditRequest,
    pub local_id: Option<Id>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSampleRequest {
    pub base: BaseCreateRequest,
    pub data: SampleRequestData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateMaterialRequest {
    pub base: BaseCreateRequest,
    pub data: RawMaterial,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateRepresentationRequest {
    pub base: BaseCreateRequest,
    pub data: RawRepresentation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateGlobalTransformRequest {
    pub base: BaseCreateRequest,
    pub data: RawGlobalTransformData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateLocalTransformRequest {
    pub base: BaseCreateRequest,
    pub data: RawTransformData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateItemRequest {
    pub base: BaseCreateRequest,
    pub data: RawItemData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateRelationRequest {
    pub base: BaseCreateRequest,
    pub data: RawRelationData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteRequest {
    pub base: BaseUpdateRequest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EditRequest {
    CreateSample(CreateSampleRequest),
    CreateMaterial(CreateMaterialRequest),
    CreateRepresentation(CreateRepresentationRequest),
    CreateGlobalTransform(CreateGlobalTransformRequest),
    CreateLocalTransform(CreateLocalTransformRequest),
    CreateItem(CreateItemRequest),
    CreateRelation(CreateRelationRequest),
    UpdateSample(UpdateSampleRequest),
    UpdateMaterial(UpdateMaterialRequest),
    UpdateRepresentation(UpdateRepresentationRequest),
    UpdateGlobalTransform(UpdateGlobalTransformRequest),
    UpdateLocalTransform(UpdateLocalTransformRequest),
    UpdateItem(UpdateItemRequest),
    UpdateMaxLocalId(UpdateMaxLocalIdRequest),
    UpdateRelation(UpdateRelationRequest),
    UpdateMetadata(UpdateMetadataRequest),
    UpdateSpatialStructure(UpdateSpatialStructureRequest),
    DeleteMaterial(DeleteRequest),
    DeleteRepresentation(DeleteRequest),
    DeleteSample(DeleteRequest),
    DeleteGlobalTransform(DeleteRequest),
    DeleteLocalTransform(DeleteRequest),
    DeleteItem(DeleteRequest),
    DeleteRelation(DeleteRequest),
}

impl EditRequest {
    pub fn request_type(&self) -> EditRequestType {
        match self {
            EditRequest::CreateSample(_) => EditRequestType::CreateSample,
            EditRequest::CreateMaterial(_) => EditRequestType::CreateMaterial,
            EditRequest::CreateRepresentation(_) => EditRequestType::CreateRepresentation,
            EditRequest::CreateGlobalTransform(_) => EditRequestType::CreateGlobalTransform,
            EditRequest::CreateLocalTransform(_) => EditRequestType::CreateLocalTransform,
            EditRequest::CreateItem(_) => EditRequestType::CreateItem,
            EditRequest::CreateRelation(_) => EditRequestType::CreateRelation,
            EditRequest::UpdateSample(_) => EditRequestType::UpdateSample,
            EditRequest::UpdateMaterial(_) => EditRequestType::UpdateMaterial,
            EditRequest::UpdateRepresentation(_) => EditRequestType::UpdateRepresentation,
            EditRequest::UpdateGlobalTransform(_) => EditRequestType::UpdateGlobalTransform,
            EditRequest::UpdateLocalTransform(_) => EditRequestType::UpdateLocalTransform,
            EditRequest::UpdateItem(_) => EditRequestType::UpdateItem,
            EditRequest::UpdateMaxLocalId(_) => EditRequestType::UpdateMaxLocalId,
            EditRequest::UpdateRelation(_) => EditRequestType::UpdateRelation,
            EditRequest::UpdateMetadata(_) => EditRequestType::UpdateMetadata,
            EditRequest::UpdateSpatialStructure(_) => EditRequestType::UpdateSpatialStructure,
            EditRequest::DeleteMaterial(_) => EditRequestType::DeleteMaterial,
            EditRequest::DeleteRepresentation(_) => EditRequestType::DeleteRepresentation,
            EditRequest::DeleteSample(_) => EditRequestType::DeleteSample,
            EditRequest::DeleteGlobalTransform(_) => EditRequestType::DeleteGlobalTransform,
            EditRequest::DeleteLocalTransform(_) => EditRequestType::DeleteLocalTransform,
            EditRequest::DeleteItem(_) => EditRequestType::DeleteItem,
            EditRequest::DeleteRelation(_) => EditRequestType::DeleteRelation,
        }
    }

    pub fn temp_id(&self) -> Option<&str> {
        match self {
            EditRequest::CreateSample(request) => request.base.base.temp_id.as_deref(),
            EditRequest::CreateMaterial(request) => request.base.base.temp_id.as_deref(),
            EditRequest::CreateRepresentation(request) => request.base.base.temp_id.as_deref(),
            EditRequest::CreateGlobalTransform(request) => request.base.base.temp_id.as_deref(),
            EditRequest::CreateLocalTransform(request) => request.base.base.temp_id.as_deref(),
            EditRequest::CreateItem(request) => request.base.base.temp_id.as_deref(),
            EditRequest::CreateRelation(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateSample(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateMaterial(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateRepresentation(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateGlobalTransform(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateLocalTransform(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateItem(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateMaxLocalId(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateRelation(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateMetadata(request) => request.base.base.temp_id.as_deref(),
            EditRequest::UpdateSpatialStructure(request) => request.base.base.temp_id.as_deref(),
            EditRequest::DeleteMaterial(request) => request.base.base.temp_id.as_deref(),
            EditRequest::DeleteRepresentation(request) => request.base.base.temp_id.as_deref(),
            EditRequest::DeleteSample(request) => request.base.base.temp_id.as_deref(),
            EditRequest::DeleteGlobalTransform(request) => request.base.base.temp_id.as_deref(),
            EditRequest::DeleteLocalTransform(request) => request.base.base.temp_id.as_deref(),
            EditRequest::DeleteItem(request) => request.base.base.temp_id.as_deref(),
            EditRequest::DeleteRelation(request) => request.base.base.temp_id.as_deref(),
        }
    }

    pub fn local_id(&self) -> Option<&Id> {
        match self {
            EditRequest::CreateSample(request) => request.base.local_id.as_ref(),
            EditRequest::CreateMaterial(request) => request.base.local_id.as_ref(),
            EditRequest::CreateRepresentation(request) => request.base.local_id.as_ref(),
            EditRequest::CreateGlobalTransform(request) => request.base.local_id.as_ref(),
            EditRequest::CreateLocalTransform(request) => request.base.local_id.as_ref(),
            EditRequest::CreateItem(request) => request.base.local_id.as_ref(),
            EditRequest::CreateRelation(request) => request.base.local_id.as_ref(),
            EditRequest::UpdateSample(request) => Some(&request.base.local_id),
            EditRequest::UpdateMaterial(request) => Some(&request.base.local_id),
            EditRequest::UpdateRepresentation(request) => Some(&request.base.local_id),
            EditRequest::UpdateGlobalTransform(request) => Some(&request.base.local_id),
            EditRequest::UpdateLocalTransform(request) => Some(&request.base.local_id),
            EditRequest::UpdateItem(request) => Some(&request.base.local_id),
            EditRequest::UpdateMaxLocalId(request) => Some(&request.base.local_id),
            EditRequest::UpdateRelation(request) => Some(&request.base.local_id),
            EditRequest::UpdateMetadata(request) => Some(&request.base.local_id),
            EditRequest::UpdateSpatialStructure(request) => Some(&request.base.local_id),
            EditRequest::DeleteMaterial(request) => Some(&request.base.local_id),
            EditRequest::DeleteRepresentation(request) => Some(&request.base.local_id),
            EditRequest::DeleteSample(request) => Some(&request.base.local_id),
            EditRequest::DeleteGlobalTransform(request) => Some(&request.base.local_id),
            EditRequest::DeleteLocalTransform(request) => Some(&request.base.local_id),
            EditRequest::DeleteItem(request) => Some(&request.base.local_id),
            EditRequest::DeleteRelation(request) => Some(&request.base.local_id),
        }
    }

    pub fn local_id_mut(&mut self) -> Option<&mut Id> {
        match self {
            EditRequest::CreateSample(request) => request.base.local_id.as_mut(),
            EditRequest::CreateMaterial(request) => request.base.local_id.as_mut(),
            EditRequest::CreateRepresentation(request) => request.base.local_id.as_mut(),
            EditRequest::CreateGlobalTransform(request) => request.base.local_id.as_mut(),
            EditRequest::CreateLocalTransform(request) => request.base.local_id.as_mut(),
            EditRequest::CreateItem(request) => request.base.local_id.as_mut(),
            EditRequest::CreateRelation(request) => request.base.local_id.as_mut(),
            EditRequest::UpdateSample(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateMaterial(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateRepresentation(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateGlobalTransform(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateLocalTransform(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateItem(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateMaxLocalId(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateRelation(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateMetadata(request) => Some(&mut request.base.local_id),
            EditRequest::UpdateSpatialStructure(request) => Some(&mut request.base.local_id),
            EditRequest::DeleteMaterial(request) => Some(&mut request.base.local_id),
            EditRequest::DeleteRepresentation(request) => Some(&mut request.base.local_id),
            EditRequest::DeleteSample(request) => Some(&mut request.base.local_id),
            EditRequest::DeleteGlobalTransform(request) => Some(&mut request.base.local_id),
            EditRequest::DeleteLocalTransform(request) => Some(&mut request.base.local_id),
            EditRequest::DeleteItem(request) => Some(&mut request.base.local_id),
            EditRequest::DeleteRelation(request) => Some(&mut request.base.local_id),
        }
    }

    pub fn set_local_id(&mut self, id: Id) {
        match self {
            EditRequest::CreateSample(request) => request.base.local_id = Some(id),
            EditRequest::CreateMaterial(request) => request.base.local_id = Some(id),
            EditRequest::CreateRepresentation(request) => request.base.local_id = Some(id),
            EditRequest::CreateGlobalTransform(request) => request.base.local_id = Some(id),
            EditRequest::CreateLocalTransform(request) => request.base.local_id = Some(id),
            EditRequest::CreateItem(request) => request.base.local_id = Some(id),
            EditRequest::CreateRelation(request) => request.base.local_id = Some(id),
            EditRequest::UpdateSample(request) => request.base.local_id = id,
            EditRequest::UpdateMaterial(request) => request.base.local_id = id,
            EditRequest::UpdateRepresentation(request) => request.base.local_id = id,
            EditRequest::UpdateGlobalTransform(request) => request.base.local_id = id,
            EditRequest::UpdateLocalTransform(request) => request.base.local_id = id,
            EditRequest::UpdateItem(request) => request.base.local_id = id,
            EditRequest::UpdateMaxLocalId(request) => request.base.local_id = id,
            EditRequest::UpdateRelation(request) => request.base.local_id = id,
            EditRequest::UpdateMetadata(request) => request.base.local_id = id,
            EditRequest::UpdateSpatialStructure(request) => request.base.local_id = id,
            EditRequest::DeleteMaterial(request) => request.base.local_id = id,
            EditRequest::DeleteRepresentation(request) => request.base.local_id = id,
            EditRequest::DeleteSample(request) => request.base.local_id = id,
            EditRequest::DeleteGlobalTransform(request) => request.base.local_id = id,
            EditRequest::DeleteLocalTransform(request) => request.base.local_id = id,
            EditRequest::DeleteItem(request) => request.base.local_id = id,
            EditRequest::DeleteRelation(request) => request.base.local_id = id,
        }
    }

    pub fn sample_data_mut(&mut self) -> Option<&mut SampleRequestData> {
        match self {
            EditRequest::CreateSample(request) => Some(&mut request.data),
            EditRequest::UpdateSample(request) => Some(&mut request.data),
            _ => None,
        }
    }

    pub fn global_transform_data_mut(&mut self) -> Option<&mut RawGlobalTransformData> {
        match self {
            EditRequest::CreateGlobalTransform(request) => Some(&mut request.data),
            EditRequest::UpdateGlobalTransform(request) => Some(&mut request.data),
            _ => None,
        }
    }
}
