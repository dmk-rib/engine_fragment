use fragments::schema::{
    Attribute, Axis, AxisPartClass, BoundingBox, CircleCurve, CircleExtrusion, DoubleVector,
    FloatVector, Material, Meshes, Model, Relation, RenderedFaces, Representation,
    RepresentationClass, Sample, Shell, ShellHole, ShellProfile, ShellType, SpatialStructure,
    Stroke, Transform, Wire, WireSet,
};

#[test]
fn schema_types_construction_matches_fbs_defaults() {
    let material = Material::default();
    assert_eq!(material.rendered_faces, RenderedFaces::One);
    assert_eq!(material.stroke, Stroke::Default);

    let bbox = BoundingBox::default();
    assert_eq!(bbox.min, FloatVector::default());
    assert_eq!(bbox.max, FloatVector::default());

    let representation = Representation::default();
    assert_eq!(
        representation.representation_class,
        RepresentationClass::None
    );

    let sample = Sample::default();
    assert_eq!(sample.item, 0);
    assert_eq!(sample.local_transform, 0);

    let axis = Axis::default();
    assert!(axis.wires.is_empty());
    assert_eq!(axis.parts, Vec::<AxisPartClass>::new());

    let hole = ShellHole::default();
    assert_eq!(hole.profile_id, 0);

    let profile = ShellProfile::default();
    assert!(profile.indices.is_empty());

    let curve = CircleCurve::default();
    assert_eq!(curve.aperture, 0.0);
    assert_eq!(curve.radius, 0.0);

    let wire = Wire::default();
    assert_eq!(wire.p1, FloatVector::default());

    let wire_set = WireSet::default();
    assert!(wire_set.ps.is_empty());

    let _shell_type = ShellType::default();

    let double_vector = DoubleVector::default();
    assert_eq!(double_vector.x, 0.0);

    let transform = Transform::default();
    assert_eq!(transform.position, DoubleVector::default());

    let circle_extrusion = CircleExtrusion::default();
    assert!(circle_extrusion.radius.is_empty());

    let shell = Shell::default();
    assert!(shell.points.is_empty());
    assert_eq!(shell.shell_type, ShellType::default());

    let meshes = Meshes::default();
    assert_eq!(meshes.coordinates, Transform::default());
    assert!(meshes.samples.is_empty());

    let attribute = Attribute::default();
    assert!(attribute.data.is_empty());

    let relation = Relation::default();
    assert!(relation.data.is_empty());

    let spatial_structure = SpatialStructure::default();
    assert!(spatial_structure.children.is_empty());
    assert_eq!(spatial_structure.category, None);

    let model = Model::default();
    assert!(model.guids.is_empty());
    assert_eq!(model.meshes, Meshes::default());
}
