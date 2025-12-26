#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RelationMapping {
    pub for_relating: &'static str,
    pub for_related: &'static str,
}

pub const IFC_RELATIONS_MAP: &[(u32, RelationMapping)] = &[
    (
        3818125796u32,
        RelationMapping {
            for_relating: "HasSurfaceFeatures",
            for_related: "AdheresToElement",
        },
    ),
    (
        1441486842u32,
        RelationMapping {
            for_relating: "Positions",
            for_related: "PositionedRelativeTo",
        },
    ),
    (
        1033248425u32,
        RelationMapping {
            for_relating: "AssociatedTo",
            for_related: "HasAssociations",
        },
    ),
    (
        1521410863u32,
        RelationMapping {
            for_relating: "BoundedBy",
            for_related: "ProvidesBoundaries",
        },
    ),
    (
        3523091289u32,
        RelationMapping {
            for_relating: "BoundedBy",
            for_related: "ProvidesBoundaries",
        },
    ),
    (
        427948657u32,
        RelationMapping {
            for_relating: "InterferesElements",
            for_related: "IsInterferedByElements",
        },
    ),
    (
        307848117u32,
        RelationMapping {
            for_relating: "Defines",
            for_related: "IsDefinedBy",
        },
    ),
    (
        1462361463u32,
        RelationMapping {
            for_relating: "Declares",
            for_related: "IsDeclaredBy",
        },
    ),
    (
        2565941209u32,
        RelationMapping {
            for_relating: "Declares",
            for_related: "HasContext",
        },
    ),
    (
        1027710054u32,
        RelationMapping {
            for_relating: "IsGroupedBy",
            for_related: "HasAssignments",
        },
    ),
    (
        2863920197u32,
        RelationMapping {
            for_relating: "Controls",
            for_related: "HasAssignments",
        },
    ),
    (
        160246688u32,
        RelationMapping {
            for_relating: "IsDecomposedBy",
            for_related: "Decomposes",
        },
    ),
    (
        1401173127u32,
        RelationMapping {
            for_relating: "HasOpenings",
            for_related: "VoidsElements",
        },
    ),
    (
        3451746338u32,
        RelationMapping {
            for_relating: "BoundedBy",
            for_related: "ProvidesBoundaries",
        },
    ),
    (
        366585022u32,
        RelationMapping {
            for_relating: "ServicesBuildings",
            for_related: "ServicedBySystems",
        },
    ),
    (
        4122056220u32,
        RelationMapping {
            for_relating: "IsPredecessorTo",
            for_related: "IsSuccessorFrom",
        },
    ),
    (
        1058617721u32,
        RelationMapping {
            for_relating: "ReferencedBy",
            for_related: "HasAssignments",
        },
    ),
    (
        1245217292u32,
        RelationMapping {
            for_relating: "ReferencesElements",
            for_related: "ReferencedInStructures",
        },
    ),
    (
        750771296u32,
        RelationMapping {
            for_relating: "HasProjections",
            for_related: "ProjectsElements",
        },
    ),
    (
        202636808u32,
        RelationMapping {
            for_relating: "DefinesOccurrence",
            for_related: "IsDefinedBy",
        },
    ),
    (
        2051452291u32,
        RelationMapping {
            for_relating: "IsActingUpon",
            for_related: "HasAssignments",
        },
    ),
    (
        3268803585u32,
        RelationMapping {
            for_relating: "IsNestedBy",
            for_related: "Nests",
        },
    ),
    (
        4189434867u32,
        RelationMapping {
            for_relating: "HasInteractionReqsTo",
            for_related: "HasInteractionReqsFrom",
        },
    ),
    (
        279856033u32,
        RelationMapping {
            for_relating: "HasControlElements",
            for_related: "AssignedToFlowElement",
        },
    ),
    (
        3940055652u32,
        RelationMapping {
            for_relating: "HasFillings",
            for_related: "FillsVoids",
        },
    ),
    (
        781010003u32,
        RelationMapping {
            for_relating: "Types",
            for_related: "IsTypedBy",
        },
    ),
    (
        4186316022u32,
        RelationMapping {
            for_relating: "DefinesOccurrence",
            for_related: "IsDefinedBy",
        },
    ),
    (
        693640335u32,
        RelationMapping {
            for_relating: "DefinesOccurrence",
            for_related: "IsDefinedBy",
        },
    ),
    (
        2551354335u32,
        RelationMapping {
            for_relating: "IsDecomposedBy",
            for_related: "Decomposes",
        },
    ),
    (
        2802773753u32,
        RelationMapping {
            for_relating: "HasCoverings",
            for_related: "CoversSpaces",
        },
    ),
    (
        886880790u32,
        RelationMapping {
            for_relating: "HasCoverings",
            for_related: "CoversElements",
        },
    ),
    (
        3242617779u32,
        RelationMapping {
            for_relating: "ContainsElements",
            for_related: "ContainedInStructure",
        },
    ),
    (
        3678494232u32,
        RelationMapping {
            for_relating: "ConnectedTo",
            for_related: "ConnectedFrom",
        },
    ),
    (
        504942748u32,
        RelationMapping {
            for_relating: "ConnectedBy",
            for_related: "ConnectsStructuralMembers",
        },
    ),
    (
        1638771189u32,
        RelationMapping {
            for_relating: "ConnectedBy",
            for_related: "ConnectsStructuralMembers",
        },
    ),
    (
        3912681535u32,
        RelationMapping {
            for_relating: "HasStructuralMember",
            for_related: "ReferencesElement",
        },
    ),
    (
        2127690289u32,
        RelationMapping {
            for_relating: "AssignedStructuralActivity",
            for_related: "AssignedToStructuralItem",
        },
    ),
    (
        3190031847u32,
        RelationMapping {
            for_relating: "ConnectedTo",
            for_related: "ConnectedFrom",
        },
    ),
    (
        4201705270u32,
        RelationMapping {
            for_relating: "ContainedIn",
            for_related: "HasPorts",
        },
    ),
    (
        3945020480u32,
        RelationMapping {
            for_relating: "ConnectedTo",
            for_related: "ConnectedFrom",
        },
    ),
    (
        1204542856u32,
        RelationMapping {
            for_relating: "ConnectedTo",
            for_related: "ConnectedFrom",
        },
    ),
    (
        826625072u32,
        RelationMapping {
            for_relating: "ConnectedTo",
            for_related: "ConnectedFrom",
        },
    ),
    (
        2851387026u32,
        RelationMapping {
            for_relating: "AssociatedTo",
            for_related: "HasAssociations",
        },
    ),
    (
        2655215786u32,
        RelationMapping {
            for_relating: "AssociatedTo",
            for_related: "HasAssociations",
        },
    ),
    (
        3840914261u32,
        RelationMapping {
            for_relating: "LibraryInfoForObjects",
            for_related: "HasAssociations",
        },
    ),
    (
        982818633u32,
        RelationMapping {
            for_relating: "DocumentInfoForObjects",
            for_related: "DocumentRefForObjects",
        },
    ),
    (
        2728634034u32,
        RelationMapping {
            for_relating: "AssociatedTo",
            for_related: "HasAssociations",
        },
    ),
    (
        919958153u32,
        RelationMapping {
            for_relating: "HasReferences",
            for_related: "ClassificationRefForObjects",
        },
    ),
    (
        4095574036u32,
        RelationMapping {
            for_relating: "ApprovedObjects",
            for_related: "HasAssociations",
        },
    ),
    (
        1327628568u32,
        RelationMapping {
            for_relating: "AssociatedTo",
            for_related: "HasAssociations",
        },
    ),
    (
        1865459582u32,
        RelationMapping {
            for_relating: "AssociatedTo",
            for_related: "HasAssociations",
        },
    ),
    (
        205026976u32,
        RelationMapping {
            for_relating: "ResourceOf",
            for_related: "HasAssignments",
        },
    ),
    (
        3372526763u32,
        RelationMapping {
            for_relating: "ReferencedBy",
            for_related: "HasAssignments",
        },
    ),
    (
        2857406711u32,
        RelationMapping {
            for_relating: "ReferencedBy",
            for_related: "HasAssignments",
        },
    ),
    (
        4278684876u32,
        RelationMapping {
            for_relating: "OperatesOn",
            for_related: "HasAssignments",
        },
    ),
    (
        1307041759u32,
        RelationMapping {
            for_relating: "IsGroupedBy",
            for_related: "HasAssignments",
        },
    ),
    (
        2495723537u32,
        RelationMapping {
            for_relating: "Controls",
            for_related: "HasAssignments",
        },
    ),
    (
        1683148259u32,
        RelationMapping {
            for_relating: "IsActingUpon",
            for_related: "HasAssignments",
        },
    ),
    (
        3939117080u32,
        RelationMapping {
            for_relating: "ReferencedBy",
            for_related: "HasAssignments",
        },
    ),
];

pub fn relation_mapping(entity_id: u32) -> Option<RelationMapping> {
    IFC_RELATIONS_MAP.iter().find_map(|(key, value)| {
        if *key == entity_id {
            Some(*value)
        } else {
            None
        }
    })
}
