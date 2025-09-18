use crate::bmi::{Bmi, RefValueVec, ValueType, ValueVec};
use ffi::{BMI_FAILURE, BMI_SUCCESS};
use std::ffi::{
    CStr, CString, c_char, c_double, c_float, c_int, c_long, c_short, c_uint, c_ulong, c_ushort,
    c_void,
};
use std::slice;

fn copy_str(src: &str, out: *mut c_char) -> Option<()> {
    let Ok(c_string) = CString::new(src) else {
        return None;
    };

    let bytes = c_string.as_bytes_with_nul();
    // NOTE: not sure if this is cross platform.
    // There is nothing that would lead me to believe it wouldn't be though
    let name_buffer = unsafe { slice::from_raw_parts_mut(out as *mut u8, bytes.len()) };
    // ensure slices are the same length. otherwise, this will avoid panic
    name_buffer[..bytes.len()].copy_from_slice(bytes);
    return Some(());
}

macro_rules! data_field {
    ($value:expr) => {{
        let foo = unsafe { $value.as_mut() }.unwrap();
        unsafe { std::mem::transmute(foo.data) }
    }};
}

macro_rules! as_str_ref_or_fail {
    ($value:expr) => {{
        let c_str: &CStr = unsafe { CStr::from_ptr($value) };
        let Ok(str_slice) = c_str.to_str() else {
            return BMI_FAILURE;
        };
        str_slice
    }};
}

macro_rules! ok_or_fail {
    ($value:expr) => {{
        let Ok(value) = $value else {
            return BMI_FAILURE;
        };
        value
    }};
}

pub extern "C" fn initialize<T: Bmi>(self_: *mut ffi::Bmi, config_file: *const c_char) -> c_int {
    let config_file = as_str_ref_or_fail!(config_file);
    let data: &mut T = data_field!(&self_);
    data.initialize(config_file).bmi_result()
}

trait BmiResult {
    fn bmi_result(&self) -> c_int;
}

impl<T> BmiResult for Option<T> {
    fn bmi_result(&self) -> c_int {
        match self {
            Some(_) => BMI_SUCCESS,
            None => BMI_FAILURE,
        }
    }
}

impl<T, E> BmiResult for Result<T, E> {
    fn bmi_result(&self) -> c_int {
        match self {
            Ok(_) => BMI_SUCCESS,
            Err(_) => BMI_FAILURE,
        }
    }
}

pub extern "C" fn update<T: Bmi>(self_: *mut ffi::Bmi) -> c_int {
    let data: &mut T = data_field!(&self_);
    data.update().bmi_result()
}

pub extern "C" fn update_until<T: Bmi>(self_: *mut ffi::Bmi, then: c_double) -> c_int {
    let data: &mut T = data_field!(&self_);
    data.update_until(then).bmi_result()
}

pub extern "C" fn finalize<T: Bmi>(self_: *mut ffi::Bmi) -> c_int {
    let s = unsafe { &mut *self_ };
    let data: &mut T = data_field!(&self_);
    // NOTE: im not sure if this is semantically correct?
    let _ = data.finalize();
    {
        // drop data field
        let _ = unsafe { Box::from_raw(data as *mut T) };
    }
    s.data = std::ptr::null_mut();
    BMI_SUCCESS
}

pub extern "C" fn get_component_name<T: Bmi>(self_: *mut ffi::Bmi, name: *mut c_char) -> c_int {
    let data: &mut T = data_field!(&self_);
    copy_str(data.get_component_name(), name).bmi_result()
}

pub extern "C" fn get_input_item_count<T: Bmi>(self_: *mut ffi::Bmi, count: *mut c_int) -> c_int {
    let data: &mut T = data_field!(&self_);
    unsafe { *count = data.get_input_item_count() as c_int };
    BMI_SUCCESS
}

pub extern "C" fn get_output_item_count<T: Bmi>(self_: *mut ffi::Bmi, count: *mut c_int) -> c_int {
    let data: &mut T = data_field!(&self_);
    unsafe { *count = data.get_output_item_count() as c_int };
    BMI_SUCCESS
}

// NOTE: I not sure if the double pointer is right or not?
pub extern "C" fn get_input_var_names<T: Bmi>(
    self_: *mut ffi::Bmi,
    names: *mut *mut c_char,
) -> c_int {
    let data: &mut T = data_field!(&self_);
    let var_names = data.get_input_var_names();

    let name_buffer = unsafe { slice::from_raw_parts_mut(names as *mut *mut u8, var_names.len()) };
    for (var_name, buffer) in std::iter::zip(var_names, name_buffer) {
        // Safety: for each var, add extra byte to account for null character
        let buffer = unsafe { slice::from_raw_parts_mut(*buffer as *mut u8, var_name.len() + 1) };
        buffer[..var_name.len()].copy_from_slice(var_name.as_bytes());
        buffer[var_name.len()] = 0;
    }
    BMI_SUCCESS
}

pub extern "C" fn get_output_var_names<T: Bmi>(
    self_: *mut ffi::Bmi,
    names: *mut *mut c_char,
) -> c_int {
    let data: &mut T = data_field!(&self_);
    let var_names = data.get_output_var_names();
    let name_buffer = unsafe { slice::from_raw_parts_mut(names as *mut *mut u8, var_names.len()) };
    for (var_name, buffer) in std::iter::zip(var_names, name_buffer) {
        // Safety: for each var, add extra byte to account for null character
        let buffer = unsafe { slice::from_raw_parts_mut(*buffer as *mut u8, var_name.len() + 1) };
        buffer[..var_name.len()].copy_from_slice(var_name.as_bytes());
        buffer[var_name.len()] = 0;
    }
    BMI_SUCCESS
}

/* Variable information */
pub extern "C" fn get_var_grid<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    grid: *mut c_int,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);
    let data: &mut T = data_field!(&self_);
    let grid_id = ok_or_fail!(data.get_var_grid(var_name));
    unsafe { *grid = grid_id };
    return BMI_FAILURE;
}

pub extern "C" fn get_var_type<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    ty: *mut c_char,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);
    let data: &mut T = data_field!(&self_);
    let var_type = ok_or_fail!(data.get_var_type(var_name));

    let var_type = match var_type {
        ValueType::I16 => "short",
        ValueType::U16 => "unsigned short",
        ValueType::I32 => "int",
        ValueType::U32 => "unsigned int",
        ValueType::I64 => "long",          // or long long
        ValueType::U64 => "unsigned long", // or usually unsigned long long
        ValueType::F32 => "float",
        ValueType::F64 => "double",
    };

    copy_str(var_type, ty).bmi_result()
}
pub extern "C" fn get_var_units<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    units: *mut c_char,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);
    let data: &mut T = data_field!(&self_);
    let var_units = ok_or_fail!(data.get_var_units(var_name));
    copy_str(var_units, units).bmi_result()
}
pub extern "C" fn get_var_itemsize<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    size: *mut c_int,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);
    let data: &mut T = data_field!(&self_);
    let item_size = ok_or_fail!(data.get_var_itemsize(var_name));
    unsafe { *size = item_size as i32 };
    BMI_SUCCESS
}
pub extern "C" fn get_var_nbytes<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    nbytes: *mut c_int,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);
    let data: &mut T = data_field!(&self_);
    let var_nbytes = ok_or_fail!(data.get_var_nbytes(var_name));
    unsafe { *nbytes = var_nbytes as i32 };
    BMI_SUCCESS
}
pub extern "C" fn get_var_location<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    location: *mut c_char,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);
    let data: &mut T = data_field!(&self_);
    let var_location = ok_or_fail!(data.get_var_location(var_name));
    copy_str(var_location.to_string().as_str(), location).bmi_result()
}

/* Time information */
pub extern "C" fn get_current_time<T: Bmi>(self_: *mut ffi::Bmi, time: *mut c_double) -> c_int {
    let data: &mut T = data_field!(&self_);
    unsafe { *time = data.get_current_time() };
    BMI_SUCCESS
}
pub extern "C" fn get_start_time<T: Bmi>(self_: *mut ffi::Bmi, time: *mut c_double) -> c_int {
    let data: &mut T = data_field!(&self_);
    unsafe { *time = data.get_start_time() };
    BMI_SUCCESS
}
pub extern "C" fn get_end_time<T: Bmi>(self_: *mut ffi::Bmi, time: *mut c_double) -> c_int {
    let data: &mut T = data_field!(&self_);
    unsafe { *time = data.get_end_time() };
    BMI_SUCCESS
}
pub extern "C" fn get_time_units<T: Bmi>(self_: *mut ffi::Bmi, units: *mut c_char) -> c_int {
    let data: &mut T = data_field!(&self_);
    copy_str(data.get_time_units(), units).bmi_result()
}
pub extern "C" fn get_time_step<T: Bmi>(self_: *mut ffi::Bmi, time_step: *mut c_double) -> c_int {
    let data: &mut T = data_field!(&self_);
    unsafe { *time_step = data.get_time_step() };
    BMI_SUCCESS
}

macro_rules! copy_from_slice {
    ($dest:ident, $value:expr, $ctype: ty) => {{
        let value_slice = unsafe { slice::from_raw_parts_mut($dest as *mut $ctype, $value.len()) };
        value_slice.copy_from_slice($value);
    }};
}

// /* Getters */
pub extern "C" fn get_value<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    dest: *mut c_void,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);
    let data: &mut T = data_field!(&self_);

    // NOTE: no need to clone vec on rust side, we can just copy into the provided dest ptr.
    let value = ok_or_fail!(data.get_value_ptr(var_name));

    match value {
        RefValues::I16(v) => copy_from_slice!(dest, v, c_short),
        RefValues::U16(v) => copy_from_slice!(dest, v, c_ushort),
        RefValues::I32(v) => copy_from_slice!(dest, v, c_int),
        RefValues::U32(v) => copy_from_slice!(dest, v, c_uint),
        RefValues::I64(v) => copy_from_slice!(dest, v, c_long),
        RefValues::U64(v) => copy_from_slice!(dest, v, c_ulong),
        RefValues::F32(v) => copy_from_slice!(dest, v, c_float),
        RefValues::F64(v) => copy_from_slice!(dest, v, c_double),
    }
    BMI_SUCCESS
}

pub extern "C" fn get_value_ptr<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    dest: *mut *mut c_void,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);
    let data: &mut T = data_field!(&self_);

    let value_ptr = ok_or_fail!(data.get_value_ptr(var_name));

    let src = match value_ptr {
        RefValues::I16(v) => v.as_ptr() as *mut c_void,
        RefValues::U16(v) => v.as_ptr() as *mut c_void,
        RefValues::I32(v) => v.as_ptr() as *mut c_void,
        RefValues::U32(v) => v.as_ptr() as *mut c_void,
        RefValues::I64(v) => v.as_ptr() as *mut c_void,
        RefValues::U64(v) => v.as_ptr() as *mut c_void,
        RefValues::F32(v) => v.as_ptr() as *mut c_void,
        RefValues::F64(v) => v.as_ptr() as *mut c_void,
    };
    unsafe { *dest = src };
    BMI_SUCCESS
}

pub extern "C" fn get_value_at_indices<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    dest: *mut c_void,
    inds: *mut c_int,
    count: c_int,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);

    if count < 0 {
        return BMI_FAILURE;
    }
    let count = count as usize;

    let var_ids: Option<Vec<u32>> = unsafe { slice::from_raw_parts(inds, count) }
        .iter()
        .map(|item: &i32| -> Option<u32> {
            return match *item > -1 {
                true => Some(*item as u32),
                false => None,
            };
        })
        .collect::<Option<Vec<u32>>>();

    let Some(var_ids) = var_ids else {
        // one or more negative index values provided
        return BMI_FAILURE;
    };

    let data: &mut T = data_field!(&self_);
    let value = ok_or_fail!(data.get_value_at_indices(var_name, &var_ids));

    // NOTE: not sure if this should be, value.len() <= count or ==
    // we really should only panic if there are move values than space in dest
    assert_eq!(value.len(), count);

    match value {
        Values::I16(v) => copy_from_slice!(dest, v.as_slice(), c_short),
        Values::U16(v) => copy_from_slice!(dest, v.as_slice(), c_ushort),
        Values::I32(v) => copy_from_slice!(dest, v.as_slice(), c_int),
        Values::U32(v) => copy_from_slice!(dest, v.as_slice(), c_uint),
        Values::I64(v) => copy_from_slice!(dest, v.as_slice(), c_long),
        Values::U64(v) => copy_from_slice!(dest, v.as_slice(), c_ulong),
        Values::F32(v) => copy_from_slice!(dest, v.as_slice(), c_float),
        Values::F64(v) => copy_from_slice!(dest, v.as_slice(), c_double),
    }
    BMI_SUCCESS
}

// /* Setters */
pub extern "C" fn set_value<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    value: *mut c_void,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);

    let data: &mut T = data_field!(&self_);
    let len = ok_or_fail!(data.get_value_ptr(var_name)).len();
    let var_type = ok_or_fail!(data.get_var_type(var_name));

    let res = match var_type {
        ValueType::I16 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i16, len) };
            data.set_value(var_name, RefValues::from(src))
        }
        ValueType::U16 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u16, len) };
            data.set_value(var_name, RefValues::from(src))
        }
        ValueType::I32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i32, len) };
            data.set_value(var_name, RefValues::from(src))
        }
        ValueType::U32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u32, len) };
            data.set_value(var_name, RefValues::from(src))
        }
        ValueType::I64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i64, len) };
            data.set_value(var_name, RefValues::from(src))
        }
        ValueType::U64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u64, len) };
            data.set_value(var_name, RefValues::from(src))
        }
        ValueType::F32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut f32, len) };
            data.set_value(var_name, RefValues::from(src))
        }
        ValueType::F64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut f64, len) };
            data.set_value(var_name, RefValues::from(src))
        }
    };
    res.bmi_result()
}

pub extern "C" fn set_value_at_indices<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    inds: *mut c_int,
    count: c_int,
    src: *mut c_void,
) -> c_int {
    let var_name = as_str_ref_or_fail!(name);

    if count < 0 {
        return BMI_FAILURE;
    }
    let count = count as usize;

    // TODO: technically this should be a Option<Vec<sizeof<c_int>>>, but im not sure how to do
    // that yet
    let var_ids: Option<Vec<u32>> = unsafe { slice::from_raw_parts(inds, count) }
        .iter()
        .map(|item: &i32| -> Option<u32> {
            return match *item > -1 {
                true => Some(*item as u32),
                false => None,
            };
        })
        .collect();

    let Some(var_ids) = var_ids else {
        // one or more negative index values provided
        return BMI_FAILURE;
    };

    let data: &mut T = data_field!(&self_);
    let var_type = ok_or_fail!(data.get_var_type(var_name));

    let res = match var_type {
        ValueType::I16 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i16, count) };
            data.set_value_at_indices(var_name, &var_ids, src.into())
        }
        ValueType::U16 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u16, count) };
            data.set_value_at_indices(var_name, &var_ids, src.into())
        }
        ValueType::I32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i32, count) };
            data.set_value_at_indices(var_name, &var_ids, src.into())
        }
        ValueType::U32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u32, count) };
            data.set_value_at_indices(var_name, &var_ids, src.into())
        }
        ValueType::I64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i64, count) };
            data.set_value_at_indices(var_name, &var_ids, src.into())
        }
        ValueType::U64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u64, count) };
            data.set_value_at_indices(var_name, &var_ids, src.into())
        }
        ValueType::F32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut f32, count) };
            data.set_value_at_indices(var_name, &var_ids, src.into())
        }
        ValueType::F64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut f64, count) };
            data.set_value_at_indices(var_name, &var_ids, src.into())
        }
    };
    res.bmi_result()
}

macro_rules! call {
    ($method:ident($self_:ident, $in:expr, $out:ident)) => {{
        let data: &mut T = data_field!(&$self_);
        let value = ok_or_fail!(data.$method($in));
        unsafe { *$out = value };
        return BMI_SUCCESS;
    }};
    ($method:ident($self_:ident, $in:expr, $out:ident) as $cast:ty) => {{
        let data: &mut T = data_field!(&$self_);
        let value = ok_or_fail!(data.$method($in));
        unsafe { *$out = value as $cast };
        return BMI_SUCCESS;
    }};
}

/* Grid information */
pub extern "C" fn get_grid_rank<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    rank: *mut c_int,
) -> c_int {
    call!(get_grid_rank(self_, grid, rank) as i32)
}
pub extern "C" fn get_grid_size<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    size: *mut c_int,
) -> c_int {
    call!(get_grid_size(self_, grid, size) as i32)
}
pub extern "C" fn get_grid_type<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    ty: *mut c_char,
) -> c_int {
    let data: &mut T = data_field!(self_);
    let grid_type = ok_or_fail!(data.get_grid_type(grid));
    copy_str(grid_type.to_string().as_str(), ty).bmi_result()
}

/* Uniform rectilinear */
pub extern "C" fn get_grid_shape<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    shape: *mut c_int,
) -> c_int {
    call!(get_grid_shape(self_, grid, shape))
}
pub extern "C" fn get_grid_spacing<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    spacing: *mut c_double,
) -> c_int {
    call!(get_grid_spacing(self_, grid, spacing))
}
pub extern "C" fn get_grid_origin<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    origin: *mut c_double,
) -> c_int {
    call!(get_grid_origin(self_, grid, origin))
}

/* Non-uniform rectilinear, curvilinear */
pub extern "C" fn get_grid_x<T: Bmi>(self_: *mut ffi::Bmi, grid: c_int, x: *mut c_double) -> c_int {
    call!(get_grid_x(self_, grid, x))
}
pub extern "C" fn get_grid_y<T: Bmi>(self_: *mut ffi::Bmi, grid: c_int, y: *mut c_double) -> c_int {
    call!(get_grid_y(self_, grid, y))
}
pub extern "C" fn get_grid_z<T: Bmi>(self_: *mut ffi::Bmi, grid: c_int, z: *mut c_double) -> c_int {
    call!(get_grid_z(self_, grid, z))
}

/* Unstructured */
pub extern "C" fn get_grid_node_count<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    call!(get_grid_node_count(self_, grid, count))
}
pub extern "C" fn get_grid_edge_count<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    call!(get_grid_edge_count(self_, grid, count))
}
pub extern "C" fn get_grid_face_count<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    call!(get_grid_face_count(self_, grid, count))
}

pub extern "C" fn get_grid_edge_nodes<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    edge_nodes: *mut c_int,
) -> c_int {
    call!(get_grid_edge_nodes(self_, grid, edge_nodes))
}
pub extern "C" fn get_grid_face_edges<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    face_edges: *mut c_int,
) -> c_int {
    call!(get_grid_face_edges(self_, grid, face_edges))
}
pub extern "C" fn get_grid_face_nodes<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    face_nodes: *mut c_int,
) -> c_int {
    call!(get_grid_face_nodes(self_, grid, face_nodes))
}
pub extern "C" fn get_grid_nodes_per_face<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    nodes_per_face: *mut c_int,
) -> c_int {
    call!(get_grid_nodes_per_face(self_, grid, nodes_per_face))
}
