use crate::schema::{Attribute, Meshes, Relation, SpatialStructure};

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub metadata: Option<String>,
    pub guids: Vec<String>,
    pub guids_items: Vec<u32>,
    pub max_local_id: u32,
    pub local_ids: Vec<u32>,
    pub categories: Vec<String>,
    pub meshes: Meshes,
    pub attributes: Vec<Attribute>,
    pub relations: Vec<Relation>,
    pub relations_items: Vec<i32>,
    pub guid: Option<String>,
    pub spatial_structure: Option<SpatialStructure>,
    pub unique_attributes: Vec<String>,
    pub relation_names: Vec<String>,
}

impl Model {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        metadata: Option<String>,
        guids: Vec<String>,
        guids_items: Vec<u32>,
        max_local_id: u32,
        local_ids: Vec<u32>,
        categories: Vec<String>,
        meshes: Meshes,
        attributes: Vec<Attribute>,
        relations: Vec<Relation>,
        relations_items: Vec<i32>,
        guid: Option<String>,
        spatial_structure: Option<SpatialStructure>,
        unique_attributes: Vec<String>,
        relation_names: Vec<String>,
    ) -> Self {
        Self {
            metadata,
            guids,
            guids_items,
            max_local_id,
            local_ids,
            categories,
            meshes,
            attributes,
            relations,
            relations_items,
            guid,
            spatial_structure,
            unique_attributes,
            relation_names,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            metadata: None,
            guids: Vec::new(),
            guids_items: Vec::new(),
            max_local_id: 0,
            local_ids: Vec::new(),
            categories: Vec::new(),
            meshes: Meshes::default(),
            attributes: Vec::new(),
            relations: Vec::new(),
            relations_items: Vec::new(),
            guid: None,
            spatial_structure: None,
            unique_attributes: Vec::new(),
            relation_names: Vec::new(),
        }
    }
}
