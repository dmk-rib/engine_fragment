#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub elements: [f64; 16],
}

impl Matrix4 {
    pub fn identity() -> Self {
        let mut elements = [0.0; 16];
        elements[0] = 1.0;
        elements[5] = 1.0;
        elements[10] = 1.0;
        elements[15] = 1.0;
        Self { elements }
    }

    pub fn from_array(elements: [f64; 16]) -> Self {
        Self { elements }
    }

    pub fn make_rotation_x(angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_array([
            1.0, 0.0, 0.0, 0.0, 0.0, cos, -sin, 0.0, 0.0, sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn premultiply(&mut self, other: &Matrix4) {
        let mut result = [0.0; 16];
        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += other.elements[row * 4 + k] * self.elements[k * 4 + col];
                }
                result[row * 4 + col] = sum;
            }
        }
        self.elements = result;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfcProduct {
    pub object_placement: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfcObjectPlacement {
    pub relative_placement: u32,
    pub placement_rel_to: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfcAxis2Placement3D {
    pub location: u32,
    pub axis: Option<u32>,
    pub ref_direction: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfcCartesianPoint {
    pub coordinates: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfcDirection {
    pub direction_ratios: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfcUnitAssignment {
    pub units: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfcUnit {
    pub unit_type: Option<String>,
    pub name: Option<String>,
    pub prefix: Option<String>,
}

pub trait IfcApi {
    fn unit_assignment_type_id(&self) -> i32;
    fn get_line_ids_with_type(&self, model: i32, type_id: i32) -> Vec<u32>;
    fn get_unit_assignment(&self, model: i32, id: u32) -> Option<IfcUnitAssignment>;
    fn get_unit(&self, model: i32, id: u32) -> Option<IfcUnit>;
    fn get_object_placement(&self, model: i32, id: u32) -> Option<IfcObjectPlacement>;
    fn get_axis_placement(&self, model: i32, id: u32) -> Option<IfcAxis2Placement3D>;
    fn get_cartesian_point(&self, model: i32, id: u32) -> Option<IfcCartesianPoint>;
    fn get_direction(&self, model: i32, id: u32) -> Option<IfcDirection>;
}

pub struct FragmentsIfcUtils;

impl FragmentsIfcUtils {
    pub fn get_absolute_placement<A: IfcApi>(
        api: &A,
        item: &IfcProduct,
        units_factor: Option<f64>,
    ) -> Matrix4 {
        let placement_id = item.object_placement;
        let placement = api
            .get_object_placement(0, placement_id)
            .expect("placement not found");

        let mut result = Matrix4::identity();
        let units_factor = units_factor.unwrap_or_else(|| Self::get_units_factor(api));
        Self::get_absolute_placement_recursively(api, &placement, &mut result, units_factor);

        let temp_matrix = Matrix4::make_rotation_x(-std::f64::consts::FRAC_PI_2);
        result.premultiply(&temp_matrix);
        result
    }

    pub fn get_units_factor<A: IfcApi>(api: &A) -> f64 {
        let unit_assignment_ids = api.get_line_ids_with_type(0, api.unit_assignment_type_id());

        let mut result = 1.0;
        if unit_assignment_ids.is_empty() {
            return result;
        }

        for assignment_id in unit_assignment_ids {
            let assignment = match api.get_unit_assignment(0, assignment_id) {
                Some(value) => value,
                None => continue,
            };

            for unit_id in assignment.units {
                let unit = match api.get_unit(0, unit_id) {
                    Some(value) => value,
                    None => continue,
                };

                if unit.unit_type.as_deref() != Some("LENGTHUNIT") {
                    continue;
                }

                let mut factor = 1.0;
                let mut unit_value = 1.0;
                if unit.name.as_deref() == Some("METRE") {
                    unit_value = 1.0;
                }
                if unit.name.as_deref() == Some("FOOT") {
                    unit_value = 0.3048;
                }

                match unit.prefix.as_deref() {
                    Some("MILLI") => factor = 0.001,
                    Some("CENTI") => factor = 0.01,
                    Some("DECI") => factor = 0.1,
                    _ => {}
                }

                result = unit_value * factor;
            }
        }

        result
    }

    fn get_absolute_placement_recursively<A: IfcApi>(
        api: &A,
        placement: &IfcObjectPlacement,
        result: &mut Matrix4,
        units_factor: f64,
    ) {
        let relative_placement_id = placement.relative_placement;
        let relative_placement = api
            .get_axis_placement(0, relative_placement_id)
            .expect("relative placement not found");

        let location_id = relative_placement.location;
        let z_axis_ref = relative_placement.axis;
        let x_axis_ref = relative_placement.ref_direction;

        let mut pos = Vector3::new(0.0, 0.0, 0.0);
        let mut z_axis = Vector3::new(0.0, 0.0, 1.0);
        let mut x_axis = Vector3::new(1.0, 0.0, 0.0);

        if let Some(location_data) = api.get_cartesian_point(0, location_id) {
            if location_data.coordinates.len() >= 3 {
                pos.x = location_data.coordinates[0] * units_factor;
                pos.y = location_data.coordinates[1] * units_factor;
                pos.z = location_data.coordinates[2] * units_factor;
            }
        }

        if let Some(axis_id) = z_axis_ref {
            if let Some(axis_data) = api.get_direction(0, axis_id) {
                if axis_data.direction_ratios.len() >= 3 {
                    z_axis.x = axis_data.direction_ratios[0];
                    z_axis.y = axis_data.direction_ratios[1];
                    z_axis.z = axis_data.direction_ratios[2];
                }
            }
        }

        if let Some(axis_id) = x_axis_ref {
            if let Some(axis_data) = api.get_direction(0, axis_id) {
                if axis_data.direction_ratios.len() >= 3 {
                    x_axis.x = axis_data.direction_ratios[0];
                    x_axis.y = axis_data.direction_ratios[1];
                    x_axis.z = axis_data.direction_ratios[2];
                }
            }
        }

        let y_axis = z_axis.cross(&x_axis);

        let temp_matrix = Matrix4::from_array([
            x_axis.x, x_axis.y, x_axis.z, 0.0, y_axis.x, y_axis.y, y_axis.z, 0.0, z_axis.x,
            z_axis.y, z_axis.z, 0.0, pos.x, pos.y, pos.z, 1.0,
        ]);

        result.premultiply(&temp_matrix);

        if let Some(parent_id) = placement.placement_rel_to {
            if let Some(parent_placement) = api.get_object_placement(0, parent_id) {
                Self::get_absolute_placement_recursively(
                    api,
                    &parent_placement,
                    result,
                    units_factor,
                );
            }
        }
    }
}
