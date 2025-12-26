use crate::schema::{CircleExtrusion, Material, Representation, Sample, Shell, Transform};

#[derive(Debug, Clone, PartialEq)]
pub struct Meshes {
    pub coordinates: Transform,
    pub meshes_items: Vec<u32>,
    pub samples: Vec<Sample>,
    pub representations: Vec<Representation>,
    pub materials: Vec<Material>,
    pub circle_extrusions: Vec<CircleExtrusion>,
    pub shells: Vec<Shell>,
    pub local_transforms: Vec<Transform>,
    pub global_transforms: Vec<Transform>,
    pub material_ids: Vec<u32>,
    pub representation_ids: Vec<u32>,
    pub sample_ids: Vec<u32>,
    pub local_transform_ids: Vec<u32>,
    pub global_transform_ids: Vec<u32>,
}

impl Meshes {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        coordinates: Transform,
        meshes_items: Vec<u32>,
        samples: Vec<Sample>,
        representations: Vec<Representation>,
        materials: Vec<Material>,
        circle_extrusions: Vec<CircleExtrusion>,
        shells: Vec<Shell>,
        local_transforms: Vec<Transform>,
        global_transforms: Vec<Transform>,
        material_ids: Vec<u32>,
        representation_ids: Vec<u32>,
        sample_ids: Vec<u32>,
        local_transform_ids: Vec<u32>,
        global_transform_ids: Vec<u32>,
    ) -> Self {
        Self {
            coordinates,
            meshes_items,
            samples,
            representations,
            materials,
            circle_extrusions,
            shells,
            local_transforms,
            global_transforms,
            material_ids,
            representation_ids,
            sample_ids,
            local_transform_ids,
            global_transform_ids,
        }
    }
}

impl Default for Meshes {
    fn default() -> Self {
        Self {
            coordinates: Transform::default(),
            meshes_items: Vec::new(),
            samples: Vec::new(),
            representations: Vec::new(),
            materials: Vec::new(),
            circle_extrusions: Vec::new(),
            shells: Vec::new(),
            local_transforms: Vec::new(),
            global_transforms: Vec::new(),
            material_ids: Vec::new(),
            representation_ids: Vec::new(),
            sample_ids: Vec::new(),
            local_transform_ids: Vec::new(),
            global_transform_ids: Vec::new(),
        }
    }
}
