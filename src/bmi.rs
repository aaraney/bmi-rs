use std::error::Error;
use std::fmt;

pub const MAX_COMPONENT_NAME: u32 = 2048;
pub const MAX_VAR_NAME: u32 = 2048;
pub const MAX_UNITS_NAME: u32 = 2048;
pub const MAX_TYPE_NAME: u32 = 2048;

#[derive(Debug)]
pub enum VarType {
    Int,
    Float,
    Double,
}

// TODO: probably need to mark this non_exhaustive?
#[derive(Debug)]
pub enum Location {
    Node,
    Edge,
    Face,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Node => write!(f, "node"),
            Location::Edge => write!(f, "edge"),
            Location::Face => write!(f, "face"),
        }
    }
}

#[derive(Debug)]
pub enum GridType {
    Scalar,
    Points,
    Vector,
    Unstructured,
    StructuredQuadrilateral,
    Rectilinear,
    UniformRectilinear,
}

impl std::fmt::Display for GridType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridType::Scalar => write!(f, "scalar"),
            GridType::Points => write!(f, "points"),
            GridType::Vector => write!(f, "vector"),
            GridType::Unstructured => write!(f, "unstructured"),
            GridType::StructuredQuadrilateral => write!(f, "structured_quadrilateral"),
            GridType::Rectilinear => write!(f, "rectilinear"),
            GridType::UniformRectilinear => write!(f, "uniform_rectilinear"),
        }
    }
}

#[derive(Debug)]
pub enum ValueType {
    I16, // short
    U16, // unsigned short
    I32, // usually int
    U32, // usually unsigned int
    I64, // long or usually long long
    U64, // unsigned long or usually unsigned long long
    F32, // float
    F64, // double
}

impl ValueType {
    pub fn bytes(&self) -> usize {
        return match self {
            ValueType::I16 | ValueType::U16 => 2,
            ValueType::I32 | ValueType::U32 | ValueType::F32 => 4,
            ValueType::I64 | ValueType::U64 | ValueType::F64 => 8,
        };
    }
}

// See: https://github.com/NOAA-OWP/ngen/blob/52f43540239e202328c7c9350149f9f5b8f1f409/include/realizations/catchment/Bmi_Module_Formulation.hpp#L779
#[derive(Debug)]
pub enum ValueVec {
    I16(Vec<i16>), // short
    U16(Vec<u16>), // unsigned short
    I32(Vec<i32>), // usually int
    U32(Vec<u32>), // usually unsigned int
    I64(Vec<i64>), // long or usually long long
    U64(Vec<u64>), // unsigned long or usually unsigned long long
    F32(Vec<f32>), // float
    F64(Vec<f64>), // double
}

impl ValueVec {
    pub fn value_type(&self) -> ValueType {
        match self {
            ValueVec::I16(_) => ValueType::I16,
            ValueVec::U16(_) => ValueType::U16,
            ValueVec::I32(_) => ValueType::I32,
            ValueVec::U32(_) => ValueType::U32,
            ValueVec::I64(_) => ValueType::I64,
            ValueVec::U64(_) => ValueType::U64,
            ValueVec::F32(_) => ValueType::F32,
            ValueVec::F64(_) => ValueType::F64,
        }
    }
}

impl From<Vec<i16>> for ValueVec {
    fn from(v: Vec<i16>) -> Self {
        return ValueVec::I16(v);
    }
}
impl From<Vec<u16>> for ValueVec {
    fn from(v: Vec<u16>) -> Self {
        return ValueVec::U16(v);
    }
}
impl From<Vec<u32>> for ValueVec {
    fn from(v: Vec<u32>) -> Self {
        return ValueVec::U32(v);
    }
}
impl From<Vec<i32>> for ValueVec {
    fn from(v: Vec<i32>) -> Self {
        return ValueVec::I32(v);
    }
}
impl From<Vec<u64>> for ValueVec {
    fn from(v: Vec<u64>) -> Self {
        return ValueVec::U64(v);
    }
}
impl From<Vec<i64>> for ValueVec {
    fn from(v: Vec<i64>) -> Self {
        return ValueVec::I64(v);
    }
}
impl From<Vec<f32>> for ValueVec {
    fn from(v: Vec<f32>) -> Self {
        return ValueVec::F32(v);
    }
}
impl From<Vec<f64>> for ValueVec {
    fn from(v: Vec<f64>) -> Self {
        return ValueVec::F64(v);
    }
}

impl ValueVec {
    pub fn len(&self) -> usize {
        return match self {
            ValueVec::I16(v) => v.len(),
            ValueVec::U16(v) => v.len(),
            ValueVec::I32(v) => v.len(),
            ValueVec::U32(v) => v.len(),
            ValueVec::I64(v) => v.len(),
            ValueVec::U64(v) => v.len(),
            ValueVec::F32(v) => v.len(),
            ValueVec::F64(v) => v.len(),
        };
    }
}

#[derive(Debug)]
pub enum RefValueVec<'a> {
    I16(&'a Vec<i16>), // short
    U16(&'a Vec<u16>), // unsigned short
    I32(&'a Vec<i32>), // usually int
    U32(&'a Vec<u32>), // usually unsigned int
    I64(&'a Vec<i64>), // long or usually long long
    U64(&'a Vec<u64>), // unsigned long or usually unsigned long long
    F32(&'a Vec<f32>), // float
    F64(&'a Vec<f64>), // double
}

impl<'a> From<&'a Vec<i16>> for RefValueVec<'a> {
    fn from(v: &'a Vec<i16>) -> Self {
        return RefValueVec::I16(v);
    }
}
impl<'a> From<&'a Vec<u16>> for RefValueVec<'a> {
    fn from(v: &'a Vec<u16>) -> Self {
        return RefValueVec::U16(v);
    }
}
impl<'a> From<&'a Vec<u32>> for RefValueVec<'a> {
    fn from(v: &'a Vec<u32>) -> Self {
        return RefValueVec::U32(v);
    }
}
impl<'a> From<&'a Vec<i32>> for RefValueVec<'a> {
    fn from(v: &'a Vec<i32>) -> Self {
        return RefValueVec::I32(v);
    }
}
impl<'a> From<&'a Vec<u64>> for RefValueVec<'a> {
    fn from(v: &'a Vec<u64>) -> Self {
        return RefValueVec::U64(v);
    }
}
impl<'a> From<&'a Vec<i64>> for RefValueVec<'a> {
    fn from(v: &'a Vec<i64>) -> Self {
        return RefValueVec::I64(v);
    }
}
impl<'a> From<&'a Vec<f32>> for RefValueVec<'a> {
    fn from(v: &'a Vec<f32>) -> Self {
        return RefValueVec::F32(v);
    }
}
impl<'a> From<&'a Vec<f64>> for RefValueVec<'a> {
    fn from(v: &'a Vec<f64>) -> Self {
        return RefValueVec::F64(v);
    }
}

impl<'a> RefValueVec<'a> {
    pub fn len(&'a self) -> usize {
        return match self {
            RefValueVec::I16(v) => v.len(),
            RefValueVec::U16(v) => v.len(),
            RefValueVec::I32(v) => v.len(),
            RefValueVec::U32(v) => v.len(),
            RefValueVec::I64(v) => v.len(),
            RefValueVec::U64(v) => v.len(),
            RefValueVec::F32(v) => v.len(),
            RefValueVec::F64(v) => v.len(),
        };
    }

    pub fn value_type(&'a self) -> ValueType {
        match self {
            RefValueVec::I16(_) => ValueType::I16,
            RefValueVec::U16(_) => ValueType::U16,
            RefValueVec::I32(_) => ValueType::I32,
            RefValueVec::U32(_) => ValueType::U32,
            RefValueVec::I64(_) => ValueType::I64,
            RefValueVec::U64(_) => ValueType::U64,
            RefValueVec::F32(_) => ValueType::F32,
            RefValueVec::F64(_) => ValueType::F64,
        }
    }
}

// TODO: revisit this.
// not sure if im happy with it or not.
#[derive(Debug)]
struct BmiNotImplementedError;

impl fmt::Display for BmiNotImplementedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not implemented")
    }
}

impl Error for BmiNotImplementedError {}

// TODO: Add docs
pub trait Bmi {
    /* Initialize, run, finalize (IRF) */
    fn initialize(&mut self, config_file: &str) -> Result<(), Box<dyn Error>>;
    fn update(&mut self) -> Result<(), Box<dyn Error>>;
    fn update_until(&mut self, then: f64) -> Result<(), Box<dyn Error>>;
    // TODO: I think the semantics will need to be different in the trait.
    // I don't think self should be dropped here.
    // This is just finalization, self will be dropped by the C wrapper.
    fn finalize(&mut self) -> Result<(), Box<dyn Error>>;

    /* Exchange items */
    fn get_component_name(&self) -> &str;
    fn get_input_item_count(&self) -> u32;
    fn get_output_item_count(&self) -> u32;
    fn get_input_var_names(&self) -> Vec<&str>;
    fn get_output_var_names(&self) -> Vec<&str>;

    /* Variable information */
    // should this be an option or a Result?
    fn get_var_grid(&self, name: &str) -> Result<i32, Box<dyn Error>>;
    // TODO: we could / should return an enum type here?
    // would need to be non-exhaustive
    fn get_var_type(&self, name: &str) -> Result<ValueType, Box<dyn Error>>;
    fn get_var_units(&self, name: &str) -> Result<&str, Box<dyn Error>>;
    fn get_var_itemsize(&self, name: &str) -> Result<u32, Box<dyn Error>> {
        return match self.get_var_type(name) {
            Ok(ty) => Ok(ty.bytes() as u32),
            Err(e) => Err(e),
        };
    }
    fn get_var_nbytes(&self, name: &str) -> Result<u32, Box<dyn Error>>;
    fn get_var_location(&self, name: &str) -> Result<Location, Box<dyn Error>>;

    /* Time information */
    fn get_current_time(&self) -> f64;
    fn get_start_time(&self) -> f64;
    fn get_end_time(&self) -> f64;
    fn get_time_units(&self) -> &str;
    fn get_time_step(&self) -> f64;

    /* Getters */
    fn get_value(&self, name: &str) -> Result<ValueVec, Box<dyn Error>>;
    // NOTE: not sure if we can use &ValueVec instead. I dont think so
    fn get_value_ptr(&self, name: &str) -> Result<RefValueVec, Box<dyn Error>>;
    fn get_value_at_indices(&self, name: &str, inds: &Vec<u32>)
        -> Result<ValueVec, Box<dyn Error>>;

    /* Setters */
    fn set_value(&mut self, name: &str, src: &ValueVec) -> Result<(), Box<dyn Error>>;
    fn set_value_at_indices(
        &mut self,
        name: &str,
        inds: &Vec<u32>,
        src: &RefValueVec,
    ) -> Result<(), Box<dyn Error>>;

    /* Grid information */
    fn get_grid_rank(&self, _grid: i32) -> Result<u32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    // TODO: can you have negative size grids? not sure if this should just be a i32
    fn get_grid_size(&self, _grid: i32) -> Result<u32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    // TODO: _should_ this be an enum too?
    // fn get_grid_type(&self, grid: usize) -> &str;
    fn get_grid_type(&self, _grid: i32) -> Result<GridType, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }

    /* Uniform rectilinear */
    fn get_grid_shape(&self, _grid: i32) -> Result<i32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_spacing(&self, _grid: i32) -> Result<f64, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_origin(&self, _grid: i32) -> Result<f64, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }

    /* Non-uniform rectilinear, curvilinear */
    fn get_grid_x(&self, _grid: i32) -> Result<f64, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_y(&self, _grid: i32) -> Result<f64, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_z(&self, _grid: i32) -> Result<f64, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }

    /* Unstructured */
    fn get_grid_node_count(&self, _grid: i32) -> Result<i32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_edge_count(&self, _grid: i32) -> Result<i32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_face_count(&self, _grid: i32) -> Result<i32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_edge_nodes(&self, _grid: i32) -> Result<i32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_face_edges(&self, _grid: i32) -> Result<i32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_face_nodes(&self, _grid: i32) -> Result<i32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_nodes_per_face(&self, _grid: i32) -> Result<i32, Box<dyn Error>> {
        return Err(Box::new(BmiNotImplementedError));
    }
}
