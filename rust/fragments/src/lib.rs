pub mod schema;
pub mod utils;

pub use utils::{
    async_event::AsyncEvent,
    data_map::DataMap,
    data_set::DataSet,
    edit::{
        apply_changes_to_ids, apply_changes_to_raw_data, apply_changes_to_special_data,
        edit_request_type_name, solve_gt_temp_id, solve_ids, solve_local_id_temp_id,
        solve_sample_temp_id, BaseCreateRequest, BaseEditRequest, BaseUpdateRequest,
        BufferGeometry, CreateGlobalTransformRequest, CreateItemRequest,
        CreateLocalTransformRequest, CreateMaterialRequest, CreateRelationRequest,
        CreateRepresentationRequest, CreateSampleRequest, DeleteRequest, EditKey, EditRawData,
        EditRequest, EditRequestType, ElementData, Id, ItemAttribute, ItemData, Matrix4,
        MeshLambertMaterial, NewElementData, NewElementSample, RawAxis, RawCircleCurve,
        RawCircleExtrusion, RawGeometry, RawGlobalTransformData, RawItemData, RawMaterial,
        RawMetadataData, RawRelationData, RawRepresentation, RawSample, RawShell, RawTransformData,
        ResourceRef, SampleRequestData, SpatialTreeItem, SpecialData, SpecialKey,
        UpdateGlobalTransformRequest, UpdateItemRequest, UpdateLocalTransformRequest,
        UpdateMaterialRequest, UpdateMaxLocalIdRequest, UpdateMetadataRequest,
        UpdateRelationRequest, UpdateRepresentationRequest, UpdateSampleRequest,
        UpdateSpatialStructureRequest,
    },
    event::Event,
    flatbuffers_json_converter::{get_object, FlatbuffersObject, FlatbuffersValue},
    ifc_category_map::{ifc_category_map, ifc_category_name, IFC_CATEGORY_MAP},
    ifc_geometries_map::{is_geometry_type, is_ifc_geometry, GEOMETRY_TYPES, IFC_GEOMETRIES_MAP},
    ifc_relations_map::{relation_mapping, RelationMapping, IFC_RELATIONS_MAP},
    ifc_utils::{
        FragmentsIfcUtils, IfcApi, IfcAxis2Placement3D, IfcCartesianPoint, IfcDirection,
        IfcObjectPlacement, IfcProduct, IfcUnit, IfcUnitAssignment,
    },
    shells::{
        Aabb, Edge, Face, Faces, GeomsFbUtils, Plane, Point, Points, Profile, ProfileData,
        Profiles, Vector2, Vector3,
    },
};

pub use schema::*;
