use std::error::Error;
use std::fmt;

pub const MAX_COMPONENT_NAME: u32 = 2048;
pub const MAX_VAR_NAME: u32 = 2048;
pub const MAX_UNITS_NAME: u32 = 2048;
pub const MAX_TYPE_NAME: u32 = 2048;

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
        match self {
            ValueType::I16 | ValueType::U16 => 2,
            ValueType::I32 | ValueType::U32 | ValueType::F32 => 4,
            ValueType::I64 | ValueType::U64 | ValueType::F64 => 8,
        }
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
        ValueVec::I16(v)
    }
}
impl From<Vec<u16>> for ValueVec {
    fn from(v: Vec<u16>) -> Self {
        ValueVec::U16(v)
    }
}
impl From<Vec<u32>> for ValueVec {
    fn from(v: Vec<u32>) -> Self {
        ValueVec::U32(v)
    }
}
impl From<Vec<i32>> for ValueVec {
    fn from(v: Vec<i32>) -> Self {
        ValueVec::I32(v)
    }
}
impl From<Vec<u64>> for ValueVec {
    fn from(v: Vec<u64>) -> Self {
        ValueVec::U64(v)
    }
}
impl From<Vec<i64>> for ValueVec {
    fn from(v: Vec<i64>) -> Self {
        ValueVec::I64(v)
    }
}
impl From<Vec<f32>> for ValueVec {
    fn from(v: Vec<f32>) -> Self {
        ValueVec::F32(v)
    }
}
impl From<Vec<f64>> for ValueVec {
    fn from(v: Vec<f64>) -> Self {
        ValueVec::F64(v)
    }
}

impl ValueVec {
    pub fn len(&self) -> usize {
        match self {
            ValueVec::I16(v) => v.len(),
            ValueVec::U16(v) => v.len(),
            ValueVec::I32(v) => v.len(),
            ValueVec::U32(v) => v.len(),
            ValueVec::I64(v) => v.len(),
            ValueVec::U64(v) => v.len(),
            ValueVec::F32(v) => v.len(),
            ValueVec::F64(v) => v.len(),
        }
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
        RefValueVec::I16(v)
    }
}
impl<'a> From<&'a Vec<u16>> for RefValueVec<'a> {
    fn from(v: &'a Vec<u16>) -> Self {
        RefValueVec::U16(v)
    }
}
impl<'a> From<&'a Vec<u32>> for RefValueVec<'a> {
    fn from(v: &'a Vec<u32>) -> Self {
        RefValueVec::U32(v)
    }
}
impl<'a> From<&'a Vec<i32>> for RefValueVec<'a> {
    fn from(v: &'a Vec<i32>) -> Self {
        RefValueVec::I32(v)
    }
}
impl<'a> From<&'a Vec<u64>> for RefValueVec<'a> {
    fn from(v: &'a Vec<u64>) -> Self {
        RefValueVec::U64(v)
    }
}
impl<'a> From<&'a Vec<i64>> for RefValueVec<'a> {
    fn from(v: &'a Vec<i64>) -> Self {
        RefValueVec::I64(v)
    }
}
impl<'a> From<&'a Vec<f32>> for RefValueVec<'a> {
    fn from(v: &'a Vec<f32>) -> Self {
        RefValueVec::F32(v)
    }
}
impl<'a> From<&'a Vec<f64>> for RefValueVec<'a> {
    fn from(v: &'a Vec<f64>) -> Self {
        RefValueVec::F64(v)
    }
}

impl<'a> RefValueVec<'a> {
    pub fn len(&'a self) -> usize {
        match self {
            RefValueVec::I16(v) => v.len(),
            RefValueVec::U16(v) => v.len(),
            RefValueVec::I32(v) => v.len(),
            RefValueVec::U32(v) => v.len(),
            RefValueVec::I64(v) => v.len(),
            RefValueVec::U64(v) => v.len(),
            RefValueVec::F32(v) => v.len(),
            RefValueVec::F64(v) => v.len(),
        }
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

pub type BmiResult<T> = Result<T, Box<dyn Error>>;

// TODO: Add docs
pub trait Bmi {
    /* Initialize, run, finalize (IRF) */
    fn initialize(&mut self, config_file: &str) -> BmiResult<()>;
    fn update(&mut self) -> BmiResult<()>;
    fn update_until(&mut self, then: f64) -> BmiResult<()>;
    // TODO: I think the semantics will need to be different in the trait.
    // I don't think self should be dropped here.
    // This is just finalization, self will be dropped by the C wrapper.
    // I think this should take just self?
    fn finalize(&mut self) -> BmiResult<()>;

    /* Exchange items */
    fn get_component_name(&self) -> &str;
    fn get_input_item_count(&self) -> u32;
    fn get_output_item_count(&self) -> u32;
    fn get_input_var_names(&self) -> Vec<&str>;
    fn get_output_var_names(&self) -> Vec<&str>;

    /* Variable information */
    // should this be an option or a Result?
    fn get_var_grid(&self, name: &str) -> BmiResult<i32>;
    // TODO: we could / should return an enum type here?
    // would need to be non-exhaustive
    fn get_var_type(&self, name: &str) -> BmiResult<ValueType>;
    fn get_var_units(&self, name: &str) -> BmiResult<&str>;
    fn get_var_itemsize(&self, name: &str) -> BmiResult<u32> {
        Ok(self.get_var_type(name)?.bytes() as u32)
    }
    }
    fn get_var_location(&self, name: &str) -> BmiResult<Location>;

    /* Time information */
    fn get_current_time(&self) -> f64;
    fn get_start_time(&self) -> f64;
    fn get_end_time(&self) -> f64;
    fn get_time_units(&self) -> &str;
    fn get_time_step(&self) -> f64;

    /* Getters */
    fn get_value_ptr(&self, name: &str) -> BmiResult<RefValues<'_>>;
    fn get_value_at_indices(&self, name: &str, inds: &[u32]) -> BmiResult<Values>;

    /* Setters */
    // NOTE: not sure if 'src' for set_value should be &Values or RefValues
    fn set_value(&mut self, name: &str, src: RefValues) -> BmiResult<()>;
    // NOTE: should this be just a 'RefValues' or '&RefValues'?
    fn set_value_at_indices(&mut self, name: &str, inds: &[u32], src: RefValues) -> BmiResult<()>;

    /* Grid information */
    fn get_grid_rank(&self, _grid: i32) -> BmiResult<u32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_size(&self, _grid: i32) -> BmiResult<u32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_type(&self, _grid: i32) -> BmiResult<GridType> {
        return Err(Box::new(BmiNotImplementedError));
    }

    /* Uniform rectilinear */
    fn get_grid_shape(&self, _grid: i32) -> BmiResult<i32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_spacing(&self, _grid: i32) -> BmiResult<f64> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_origin(&self, _grid: i32) -> BmiResult<f64> {
        return Err(Box::new(BmiNotImplementedError));
    }

    /* Non-uniform rectilinear, curvilinear */
    fn get_grid_x(&self, _grid: i32) -> BmiResult<f64> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_y(&self, _grid: i32) -> BmiResult<f64> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_z(&self, _grid: i32) -> BmiResult<f64> {
        return Err(Box::new(BmiNotImplementedError));
    }

    /* Unstructured */
    fn get_grid_node_count(&self, _grid: i32) -> BmiResult<i32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_edge_count(&self, _grid: i32) -> BmiResult<i32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_face_count(&self, _grid: i32) -> BmiResult<i32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_edge_nodes(&self, _grid: i32) -> BmiResult<i32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_face_edges(&self, _grid: i32) -> BmiResult<i32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_face_nodes(&self, _grid: i32) -> BmiResult<i32> {
        return Err(Box::new(BmiNotImplementedError));
    }
    fn get_grid_nodes_per_face(&self, _grid: i32) -> BmiResult<i32> {
        return Err(Box::new(BmiNotImplementedError));
    }
}

// you should now be able to do something like:
// pub extern "C" fn register_bmi_cfe(model: *mut Wrapper) -> *mut Wrapper {
//     let wrapper: &mut Wrapper = unsafe { &mut *model };
//     // this is temporary, wont be in final api
//     wrapper.setup();
//     wrapper.setup();

//     let my_model: Box<Box<dyn Bmi>> = Box::new(Box::new(some_model::new()));
//     let my_model = Box::into_raw(my_model);
//     wrapper.data = Some(model as *mut c_void);
//     return wrapper;
// }

// Bmi* register_bmi(Bmi *model);
// https://github.com/NOAA-OWP/ngen/wiki/BMI-C#additional-bootstrapping-function-needed
pub fn register_model<T: Bmi>(handle: *mut ffi::Bmi, model: T) {
    assert!(!handle.is_null(), "pointer is null");
    let handle: &mut ffi::Bmi = unsafe { handle.as_mut() }.unwrap();
    setup_fn_ptrs::<T>(handle);

    let data: Box<T> = Box::new(model);
    let data = Box::into_raw(data);
    handle.data = data as *mut std::ffi::c_void;
}

fn setup_fn_ptrs<T: Bmi>(handle: &mut ffi::Bmi) {
    handle.initialize = Some(crate::wrapper::initialize::<T>);
    handle.update = Some(crate::wrapper::update::<T>);
    handle.update_until = Some(crate::wrapper::update_until::<T>);
    handle.finalize = Some(crate::wrapper::finalize::<T>);
    handle.get_component_name = Some(crate::wrapper::get_component_name::<T>);
    handle.get_input_item_count = Some(crate::wrapper::get_input_item_count::<T>);
    handle.get_output_item_count = Some(crate::wrapper::get_output_item_count::<T>);
    handle.get_input_var_names = Some(crate::wrapper::get_input_var_names::<T>);
    handle.get_output_var_names = Some(crate::wrapper::get_output_var_names::<T>);
    handle.get_var_grid = Some(crate::wrapper::get_var_grid::<T>);
    handle.get_var_type = Some(crate::wrapper::get_var_type::<T>);
    handle.get_var_units = Some(crate::wrapper::get_var_units::<T>);
    handle.get_var_itemsize = Some(crate::wrapper::get_var_itemsize::<T>);
    handle.get_var_nbytes = Some(crate::wrapper::get_var_nbytes::<T>);
    handle.get_var_location = Some(crate::wrapper::get_var_location::<T>);
    handle.get_current_time = Some(crate::wrapper::get_current_time::<T>);
    handle.get_start_time = Some(crate::wrapper::get_start_time::<T>);
    handle.get_end_time = Some(crate::wrapper::get_end_time::<T>);
    handle.get_time_units = Some(crate::wrapper::get_time_units::<T>);
    handle.get_time_step = Some(crate::wrapper::get_time_step::<T>);
    handle.get_value = Some(crate::wrapper::get_value::<T>);
    handle.get_value_ptr = Some(crate::wrapper::get_value_ptr::<T>);
    handle.get_value_at_indices = Some(crate::wrapper::get_value_at_indices::<T>);
    handle.set_value = Some(crate::wrapper::set_value::<T>);
    handle.set_value_at_indices = Some(crate::wrapper::set_value_at_indices::<T>);
    handle.get_grid_rank = Some(crate::wrapper::get_grid_rank::<T>);
    handle.get_grid_size = Some(crate::wrapper::get_grid_size::<T>);
    handle.get_grid_type = Some(crate::wrapper::get_grid_type::<T>);
    handle.get_grid_shape = Some(crate::wrapper::get_grid_shape::<T>);
    handle.get_grid_spacing = Some(crate::wrapper::get_grid_spacing::<T>);
    handle.get_grid_origin = Some(crate::wrapper::get_grid_origin::<T>);
    handle.get_grid_x = Some(crate::wrapper::get_grid_x::<T>);
    handle.get_grid_y = Some(crate::wrapper::get_grid_y::<T>);
    handle.get_grid_z = Some(crate::wrapper::get_grid_z::<T>);
    handle.get_grid_node_count = Some(crate::wrapper::get_grid_node_count::<T>);
    handle.get_grid_edge_count = Some(crate::wrapper::get_grid_edge_count::<T>);
    handle.get_grid_face_count = Some(crate::wrapper::get_grid_face_count::<T>);
    handle.get_grid_edge_nodes = Some(crate::wrapper::get_grid_edge_nodes::<T>);
    handle.get_grid_face_edges = Some(crate::wrapper::get_grid_face_edges::<T>);
    handle.get_grid_face_nodes = Some(crate::wrapper::get_grid_face_nodes::<T>);
    handle.get_grid_nodes_per_face = Some(crate::wrapper::get_grid_nodes_per_face::<T>);
}
