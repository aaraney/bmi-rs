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

// NOTE: use a more generic container type than Vec<T>
#[derive(Debug)]
pub enum Values {
    I16(Vec<i16>), // short
    U16(Vec<u16>), // unsigned short
    I32(Vec<i32>), // usually int
    U32(Vec<u32>), // usually unsigned int
    I64(Vec<i64>), // long or usually long long
    U64(Vec<u64>), // unsigned long or usually unsigned long long
    F32(Vec<f32>), // float
    F64(Vec<f64>), // double
}

impl<'a> From<&'a Values> for RefValues<'a> {
    fn from(value: &'a Values) -> Self {
        match value {
            Values::I16(items) => RefValues::I16(&items),
            Values::U16(items) => RefValues::U16(&items),
            Values::I32(items) => RefValues::I32(&items),
            Values::U32(items) => RefValues::U32(&items),
            Values::I64(items) => RefValues::I64(&items),
            Values::U64(items) => RefValues::U64(&items),
            Values::F32(items) => RefValues::F32(&items),
            Values::F64(items) => RefValues::F64(&items),
        }
    }
}

macro_rules! impl_value_type {
    ($t:ty; $($name:ident),*$(,)?) => {
        impl $t {
            pub fn value_type(&self) -> ValueType {
                match self {
                    $(Self::$name(_) => ValueType::$name,)*
                }
            }
        }
    };
}

macro_rules! impl_len {
    ($t:ty; $($name:ident),*$(,)?) => {
        impl $t {
            pub fn len(&self) -> usize {
                match self {
                    $(Self::$name(v) => v.len(),)*
                }
            }
        }
    };
}

macro_rules! impl_from_vec_for_values {
    ($($name:ident; $t:ty),*$(,)?) => {
        $(
        impl From<Vec<$t>> for Values {
            fn from(v: Vec<$t>) -> Self {
                Values::$name(v)
            }
        }
    )*
    };
}

impl_from_vec_for_values!(
    I16;i16,
    U16;u16,
    I32;i32,
    U32;u32,
    I64;i64,
    U64;u64,
    F32;f32,
    F64;f64,
);
impl_value_type!(Values; I16, U16, I32, U32, I64, U64, F32, F64,);
impl_len!(Values; I16, U16, I32, U32, I64, U64, F32, F64,);

// See: https://github.com/NOAA-OWP/ngen/blob/52f43540239e202328c7c9350149f9f5b8f1f409/include/realizations/catchment/Bmi_Module_Formulation.hpp#L779
#[derive(Debug)]
pub enum RefValues<'a> {
    I16(&'a [i16]), // short
    U16(&'a [u16]), // unsigned short
    I32(&'a [i32]), // usually int
    U32(&'a [u32]), // usually unsigned int
    I64(&'a [i64]), // long or usually long long
    U64(&'a [u64]), // unsigned long or usually unsigned long long
    F32(&'a [f32]), // float
    F64(&'a [f64]), // double
}

macro_rules! impl_from_ref_t_for_ref_values {
    ($container:ident; $($name:ident; $t:ty),*$(,)?) => {
    $(
        impl<'a> From<&'a $container<$t>> for RefValues<'a> {
            fn from(v: &'a Vec<$t>) -> Self {
                RefValues::$name(v)
            }
        }
    )*
    };
    ($($name:ident; $t:ty),*$(,)?) => {
    $(
        impl<'a> From<&'a [$t]> for RefValues<'a> {
            fn from(v: &'a [$t]) -> Self {
                RefValues::$name(v)
            }
        }
    )*
    };
}
impl_from_ref_t_for_ref_values!(
    Vec;
    I16;i16,
    U16;u16,
    I32;i32,
    U32;u32,
    I64;i64,
    U64;u64,
    F32;f32,
    F64;f64,
);

impl_from_ref_t_for_ref_values!(
    I16;i16,
    U16;u16,
    I32;i32,
    U32;u32,
    I64;i64,
    U64;u64,
    F32;f32,
    F64;f64,
);

impl_len!(RefValues<'_>; I16, U16, I32, U32, I64, U64, F32, F64,);
impl_value_type!(RefValues<'_>; I16, U16, I32, U32, I64, U64, F32, F64,);

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
    fn get_input_var_names(&self) -> &[&str];
    fn get_output_var_names(&self) -> &[&str];

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
    fn get_var_nbytes(&self, name: &str) -> BmiResult<u32> {
        let itemsize = self.get_var_itemsize(name)?;
        let values = self.get_value_ptr(name)?;
        Ok(values.len() as u32 * itemsize)
    }
    fn get_var_location(&self, name: &str) -> BmiResult<Location>;

    /* Time information */
    fn get_current_time(&self) -> f64;
    fn get_start_time(&self) -> f64;
    fn get_end_time(&self) -> f64;
    fn get_time_units(&self) -> &str;
    fn get_time_step(&self) -> f64;

    /* Getters */
    fn get_value(&self, name: &str) -> BmiResult<Values>;
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
