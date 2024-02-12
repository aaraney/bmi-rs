use crate::bmi::{Bmi, RefValueVec, ValueType, ValueVec};
use std::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uint, c_ulong, c_ushort, c_void, CStr,
    CString,
};
use std::slice;

pub const BMI_SUCCESS: c_int = 0;
pub const BMI_FAILURE: c_int = 1;

#[derive(Debug)]
#[repr(C)]
pub struct Wrapper {
    pub data: *mut c_void,

    /* Initialize, run, finalize (IRF) */
    initialize: Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int>,
    update: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    update_until: Option<unsafe extern "C" fn(*mut c_void, c_double) -> c_int>,
    finalize: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,

    /* Exchange items */
    get_component_name: Option<unsafe extern "C" fn(*mut c_void, *mut c_char) -> c_int>,
    get_input_item_count: Option<unsafe extern "C" fn(*mut c_void, *mut c_int) -> c_int>,
    get_output_item_count: Option<unsafe extern "C" fn(*mut c_void, *mut c_int) -> c_int>,
    // NOTE: I not sure if the double pointer is right or not?
    get_input_var_names: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_char) -> c_int>,
    get_output_var_names: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_char) -> c_int>,

    /* Variable information */
    get_var_grid: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_int) -> c_int>,
    get_var_type: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_char) -> c_int>,
    get_var_units: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_char) -> c_int>,
    get_var_itemsize: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_int) -> c_int>,
    get_var_nbytes: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_int) -> c_int>,
    get_var_location:
        Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_char) -> c_int>,

    /* Time information */
    get_current_time: Option<unsafe extern "C" fn(*mut c_void, *mut c_double) -> c_int>,
    get_start_time: Option<unsafe extern "C" fn(*mut c_void, *mut c_double) -> c_int>,
    get_end_time: Option<unsafe extern "C" fn(*mut c_void, *mut c_double) -> c_int>,
    get_time_units: Option<unsafe extern "C" fn(*mut c_void, *mut c_char) -> c_int>,
    get_time_step: Option<unsafe extern "C" fn(*mut c_void, *mut c_double) -> c_int>,

    /* Getters */
    get_value: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void) -> c_int>,
    get_value_ptr:
        Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut *mut c_void) -> c_int>,
    get_value_at_indices: Option<
        unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void, *mut c_int, c_int) -> c_int,
    >,

    /* Setters */
    set_value: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void) -> c_int>,
    set_value_at_indices: Option<
        unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_int, c_int, *mut c_void) -> c_int,
    >,

    /* Grid information */
    get_grid_rank: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_size: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_type: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_char) -> c_int>,

    /* Uniform rectilinear */
    get_grid_shape: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_spacing: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_double) -> c_int>,
    get_grid_origin: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_double) -> c_int>,

    /* Non-uniform rectilinear, curvilinear */
    get_grid_x: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_double) -> c_int>,
    get_grid_y: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_double) -> c_int>,
    get_grid_z: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_double) -> c_int>,

    /* Unstructured */
    get_grid_node_count: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_edge_count: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_face_count: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_edge_nodes: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_face_edges: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_face_nodes: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
    get_grid_nodes_per_face: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int) -> c_int>,
}

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

#[no_mangle]
pub extern "C" fn initialize(this: *mut c_void, config_file: *const c_char) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(config_file) };
    let Ok(config_file) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    // *mut Box<dyn Bmi>
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    match wrapper.initialize(config_file) {
        Ok(()) => return BMI_SUCCESS,
        Err(_) => return BMI_FAILURE,
    }
}

#[no_mangle]
pub extern "C" fn update(this: *mut c_void) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    match wrapper.update() {
        Ok(()) => return BMI_SUCCESS,
        Err(_) => return BMI_FAILURE,
    }
}

#[no_mangle]
pub extern "C" fn update_until(this: *mut c_void, then: c_double) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    match wrapper.update_until(then) {
        Ok(()) => return BMI_SUCCESS,
        Err(_) => return BMI_FAILURE,
    }
}

#[no_mangle]
pub extern "C" fn finalize(this: *mut c_void) -> c_int {
    let _ = unsafe { Box::<Box<dyn Bmi>>::from_raw(this as *mut Box<dyn Bmi>) };
    return BMI_SUCCESS;
}

#[no_mangle]
pub extern "C" fn get_component_name(this: *mut c_void, name: *mut c_char) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };

    let Some(_) = copy_str(wrapper.get_component_name(), name) else {
        return BMI_FAILURE;
    };
    return BMI_SUCCESS;
}

#[no_mangle]
pub extern "C" fn get_input_item_count(this: *mut c_void, count: *mut c_int) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    unsafe {
        *count = wrapper.get_input_item_count() as c_int;
    }
    return BMI_SUCCESS;
}

#[no_mangle]
pub extern "C" fn get_output_item_count(this: *mut c_void, count: *mut c_int) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    unsafe {
        *count = wrapper.get_output_item_count() as c_int;
    }
    return BMI_SUCCESS;
}

// NOTE: I not sure if the double pointer is right or not?
#[no_mangle]
pub extern "C" fn get_input_var_names(this: *mut c_void, names: *mut *mut c_char) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let var_names = wrapper.get_input_var_names();
    let c_var_names: Vec<CString> = var_names
        .iter()
        .map(|item: &&str| -> CString {
            return CString::new(*item).expect("CString::new failed in get_input_var_names");
        })
        .collect();

    // NOTE: I think this is safe?
    let name_buffer = unsafe { slice::from_raw_parts_mut(names as *mut *mut u8, var_names.len()) };

    for (c_name, name_buff) in std::iter::zip(c_var_names, name_buffer) {
        let bytes = c_name.as_bytes_with_nul();
        // NOTE: I think we can make the slice _as large_ as the bytes buffer
        // (accounting for the null terminator, of course)
        let buff = unsafe { slice::from_raw_parts_mut(*name_buff, bytes.len()) };
        // let buff = unsafe { slice::from_raw_parts_mut(*name_buff, MAX_VAR_NAME) };
        // ensure slices are the same length. otherwise, this will avoid panic
        buff.copy_from_slice(bytes);
        // buff[..bytes.len()].copy_from_slice(bytes);
    }
    return BMI_SUCCESS;
}

#[no_mangle]
pub extern "C" fn get_output_var_names(this: *mut c_void, names: *mut *mut c_char) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let var_names = wrapper.get_output_var_names();
    let c_var_names: Vec<CString> = var_names
        .iter()
        .map(|item: &&str| -> CString {
            return CString::new(*item).expect("CString::new failed in get_input_var_names");
        })
        .collect();

    // NOTE: I think this is safe?
    let name_buffer = unsafe { slice::from_raw_parts_mut(names as *mut *mut u8, var_names.len()) };

    for (c_name, name_buff) in std::iter::zip(c_var_names, name_buffer) {
        let bytes = c_name.as_bytes_with_nul();
        // NOTE: I think we can make the slice _as large_ as the bytes buffer
        // (accounting for the null terminator, of course)
        let buff = unsafe { slice::from_raw_parts_mut(*name_buff, bytes.len()) };
        // let buff = unsafe { slice::from_raw_parts_mut(*name_buff, MAX_VAR_NAME) };
        // ensure slices are the same length. otherwise, this will avoid panic
        buff.copy_from_slice(bytes);
        // buff[..bytes.len()].copy_from_slice(bytes);
    }
    return BMI_SUCCESS;
}

/* Variable information */
#[no_mangle]
unsafe extern "C" fn get_var_grid(
    this: *mut c_void,
    name: *const c_char,
    grid: *mut c_int,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_id) = wrapper.get_var_grid(var_name) else {
        return BMI_FAILURE;
    };
    *grid = grid_id;
    return BMI_FAILURE;
}
#[no_mangle]
unsafe extern "C" fn get_var_type(
    this: *mut c_void,
    name: *const c_char,
    ty: *mut c_char,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(var_type) = wrapper.get_var_type(var_name) else {
        return BMI_FAILURE;
    };

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

    let Some(_) = copy_str(var_type, ty) else {
        return BMI_FAILURE;
    };
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_var_units(
    this: *mut c_void,
    name: *const c_char,
    units: *mut c_char,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(var_units) = wrapper.get_var_units(var_name) else {
        return BMI_FAILURE;
    };

    let Some(_) = copy_str(var_units, units) else {
        return BMI_FAILURE;
    };
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_var_itemsize(
    this: *mut c_void,
    name: *const c_char,
    size: *mut c_int,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(item_size) = wrapper.get_var_itemsize(var_name) else {
        return BMI_FAILURE;
    };
    *size = item_size as i32;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_var_nbytes(
    this: *mut c_void,
    name: *const c_char,
    nbytes: *mut c_int,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(var_nbytes) = wrapper.get_var_nbytes(var_name) else {
        return BMI_FAILURE;
    };
    *nbytes = var_nbytes as i32;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_var_location(
    this: *mut c_void,
    name: *const c_char,
    location: *mut c_char,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };

    let Ok(var_location) = wrapper.get_var_location(var_name) else {
        return BMI_FAILURE;
    };

    let Some(_) = copy_str(var_location.to_string().as_str(), location) else {
        return BMI_FAILURE;
    };
    return BMI_SUCCESS;
}

/* Time information */
#[no_mangle]
unsafe extern "C" fn get_current_time(this: *mut c_void, time: *mut c_double) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    *time = wrapper.get_current_time();
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_start_time(this: *mut c_void, time: *mut c_double) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    *time = wrapper.get_start_time();
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_end_time(this: *mut c_void, time: *mut c_double) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    *time = wrapper.get_end_time();
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_time_units(this: *mut c_void, units: *mut c_char) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Some(()) = copy_str(wrapper.get_time_units(), units) else {
        return BMI_FAILURE;
    };
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_time_step(this: *mut c_void, time_step: *mut c_double) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    *time_step = wrapper.get_time_step();
    return BMI_SUCCESS;
}

// TODO:
// /* Getters */
#[no_mangle]
unsafe extern "C" fn get_value(this: *mut c_void, name: *const c_char, dest: *mut c_void) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(value) = wrapper.get_value(var_name) else {
        return BMI_FAILURE;
    };

    match value {
        // short
        ValueVec::I16(v) => {
            // NOTE: I think this is safe?
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_short, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // unsigned short
        ValueVec::U16(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_ushort, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // usually int
        ValueVec::I32(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_int, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // usually unsigned int
        ValueVec::U32(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_uint, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // long or usually long long
        ValueVec::I64(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_long, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // unsigned long or usually unsigned long long
        ValueVec::U64(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_ulong, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // float
        ValueVec::F32(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_float, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // double
        ValueVec::F64(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_double, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
    }
    return BMI_SUCCESS;
}
// // NOTE: I think the double pntr is right?
#[no_mangle]
unsafe extern "C" fn get_value_ptr(
    this: *mut c_void,
    name: *const c_char,
    dest: *mut *mut c_void,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(value_ptr) = wrapper.get_value_ptr(var_name) else {
        return BMI_FAILURE;
    };

    unsafe {
        let src = match value_ptr {
            RefValueVec::I16(v) => v.as_ptr() as *mut c_void,
            RefValueVec::U16(v) => v.as_ptr() as *mut c_void,
            RefValueVec::I32(v) => v.as_ptr() as *mut c_void,
            RefValueVec::U32(v) => v.as_ptr() as *mut c_void,
            RefValueVec::I64(v) => v.as_ptr() as *mut c_void,
            RefValueVec::U64(v) => v.as_ptr() as *mut c_void,
            RefValueVec::F32(v) => v.as_ptr() as *mut c_void,
            RefValueVec::F64(v) => v.as_ptr() as *mut c_void,
        };
        *dest = src;
    }
    return BMI_SUCCESS;
}

#[no_mangle]
unsafe extern "C" fn get_value_at_indices(
    this: *mut c_void,
    name: *const c_char,
    dest: *mut c_void,
    inds: *mut c_int,
    count: c_int,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

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

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(value) = wrapper.get_value_at_indices(var_name, &var_ids) else {
        return BMI_FAILURE;
    };

    // NOTE: not sure if this should be, value.len() <= count or ==
    // we really should only panic if there are move values than space in dest
    assert_eq!(value.len(), count);

    match value {
        // short
        ValueVec::I16(v) => {
            // NOTE: I think this is safe?
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_short, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // unsigned short
        ValueVec::U16(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_ushort, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // usually int
        ValueVec::I32(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_int, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // usually unsigned int
        ValueVec::U32(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_uint, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // long or usually long long
        ValueVec::I64(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_long, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // unsigned long or usually unsigned long long
        ValueVec::U64(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_ulong, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // float
        ValueVec::F32(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_float, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
        // double
        ValueVec::F64(v) => {
            let value_slice = unsafe { slice::from_raw_parts_mut(dest as *mut c_double, v.len()) };
            value_slice.copy_from_slice(v.as_slice());
        }
    }
    return BMI_SUCCESS;
}

// /* Setters */
#[no_mangle]
unsafe extern "C" fn set_value(
    this: *mut c_void,
    name: *const c_char,
    value: *mut c_void,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(var_type) = wrapper.get_var_type(var_name) else {
        return BMI_FAILURE;
    };

    let Ok(var_nbytes) = wrapper.get_var_nbytes(var_name) else {
        return BMI_FAILURE;
    };
    let len = var_nbytes as usize / var_type.bytes();

    let res = match var_type {
        ValueType::I16 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i16, len) };
            let src: Vec<i16> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            wrapper.set_value(var_name, &src)
        }
        ValueType::U16 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u16, len) };
            let src: Vec<u16> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            wrapper.set_value(var_name, &src)
        }
        ValueType::I32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i32, len) };
            let src: Vec<i32> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            wrapper.set_value(var_name, &src)
        }
        ValueType::U32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u32, len) };
            let src: Vec<u32> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            wrapper.set_value(var_name, &src)
        }
        ValueType::I64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i64, len) };
            let src: Vec<i64> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            wrapper.set_value(var_name, &src)
        }
        ValueType::U64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u64, len) };
            let src: Vec<u64> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            wrapper.set_value(var_name, &src)
        }
        ValueType::F32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut f32, len) };
            let src: Vec<f32> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            wrapper.set_value(var_name, &src)
        }
        ValueType::F64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut f64, len) };
            let src: Vec<f64> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            wrapper.set_value(var_name, &src)
        }
    };
    if res.is_err() {
        return BMI_FAILURE;
    }

    return BMI_SUCCESS;
}

#[no_mangle]
unsafe extern "C" fn set_value_at_indices(
    this: *mut c_void,
    name: *const c_char,
    inds: *mut c_int,
    count: c_int,
    src: *mut c_void,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

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

    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(var_type) = wrapper.get_var_type(var_name) else {
        return BMI_FAILURE;
    };

    let res = match var_type {
        ValueType::I16 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i16, count) };
            let src: Vec<i16> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            wrapper.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::U16 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u16, count) };
            let src: Vec<u16> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            wrapper.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::I32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i32, count) };
            let src: Vec<i32> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            wrapper.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::U32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u32, count) };
            let src: Vec<u32> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            wrapper.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::I64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i64, count) };
            let src: Vec<i64> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            wrapper.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::U64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u64, count) };
            let src: Vec<u64> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            wrapper.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::F32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut f32, count) };
            let src: Vec<f32> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            wrapper.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::F64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut f64, count) };
            let src: Vec<f64> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            wrapper.set_value_at_indices(var_name, &var_ids, &src)
        }
    };
    if res.is_err() {
        return BMI_FAILURE;
    }

    return BMI_SUCCESS;
}

/* Grid information */
#[no_mangle]
unsafe extern "C" fn get_grid_rank(this: *mut c_void, grid: c_int, rank: *mut c_int) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_rank) = wrapper.get_grid_rank(grid) else {
        return BMI_FAILURE;
    };
    *rank = grid_rank as i32;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_size(this: *mut c_void, grid: c_int, size: *mut c_int) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_size) = wrapper.get_grid_size(grid) else {
        return BMI_FAILURE;
    };
    *size = grid_size as i32;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_type(this: *mut c_void, grid: c_int, ty: *mut c_char) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_type) = wrapper.get_grid_type(grid) else {
        return BMI_FAILURE;
    };
    let Some(_) = copy_str(grid_type.to_string().as_str(), ty) else {
        return BMI_FAILURE;
    };
    return BMI_SUCCESS;
}

/* Uniform rectilinear */
#[no_mangle]
unsafe extern "C" fn get_grid_shape(this: *mut c_void, grid: c_int, shape: *mut c_int) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_shape) = wrapper.get_grid_shape(grid) else {
        return BMI_FAILURE;
    };
    *shape = grid_shape;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_spacing(
    this: *mut c_void,
    grid: c_int,
    spacing: *mut c_double,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_spacing) = wrapper.get_grid_spacing(grid) else {
        return BMI_FAILURE;
    };
    *spacing = grid_spacing;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_origin(
    this: *mut c_void,
    grid: c_int,
    origin: *mut c_double,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_origin) = wrapper.get_grid_origin(grid) else {
        return BMI_FAILURE;
    };
    *origin = grid_origin;
    return BMI_SUCCESS;
}

/* Non-uniform rectilinear, curvilinear */
#[no_mangle]
unsafe extern "C" fn get_grid_x(this: *mut c_void, grid: c_int, x: *mut c_double) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_x) = wrapper.get_grid_x(grid) else {
        return BMI_FAILURE;
    };
    *x = grid_x;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_y(this: *mut c_void, grid: c_int, y: *mut c_double) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_y) = wrapper.get_grid_y(grid) else {
        return BMI_FAILURE;
    };
    *y = grid_y;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_z(this: *mut c_void, grid: c_int, z: *mut c_double) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_z) = wrapper.get_grid_z(grid) else {
        return BMI_FAILURE;
    };
    *z = grid_z;
    return BMI_SUCCESS;
}

/* Unstructured */
#[no_mangle]
unsafe extern "C" fn get_grid_node_count(
    this: *mut c_void,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(node_count) = wrapper.get_grid_node_count(grid) else {
        return BMI_FAILURE;
    };
    *count = node_count;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_edge_count(
    this: *mut c_void,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(edge_count) = wrapper.get_grid_edge_count(grid) else {
        return BMI_FAILURE;
    };
    *count = edge_count;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_face_count(
    this: *mut c_void,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(face_count) = wrapper.get_grid_face_count(grid) else {
        return BMI_FAILURE;
    };
    *count = face_count;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_edge_nodes(
    this: *mut c_void,
    grid: c_int,
    edge_nodes: *mut c_int,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_edge_nodes) = wrapper.get_grid_edge_nodes(grid) else {
        return BMI_FAILURE;
    };
    *edge_nodes = grid_edge_nodes;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_face_edges(
    this: *mut c_void,
    grid: c_int,
    face_edges: *mut c_int,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_face_edges) = wrapper.get_grid_face_edges(grid) else {
        return BMI_FAILURE;
    };
    *face_edges = grid_face_edges;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_face_nodes(
    this: *mut c_void,
    grid: c_int,
    face_nodes: *mut c_int,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_face_nodes) = wrapper.get_grid_face_nodes(grid) else {
        return BMI_FAILURE;
    };
    *face_nodes = grid_face_nodes;
    return BMI_SUCCESS;
}
#[no_mangle]
unsafe extern "C" fn get_grid_nodes_per_face(
    this: *mut c_void,
    grid: c_int,
    nodes_per_face: *mut c_int,
) -> c_int {
    let wrapper: &mut Box<dyn Bmi> = unsafe { &mut *(this as *mut Box<dyn Bmi>) };
    let Ok(grid_nodes_per_face) = wrapper.get_grid_nodes_per_face(grid) else {
        return BMI_FAILURE;
    };
    *nodes_per_face = grid_nodes_per_face;
    return BMI_SUCCESS;
}

impl From<Box<dyn Bmi>> for Wrapper {
    fn from(bmi_impl: Box<dyn Bmi>) -> Self {
        let model = Box::new(bmi_impl);
        // after this point the caller is responsible for the boxed model.
        // it must be deallocated using Wrapper's finalize method.
        let model = Box::into_raw(model);

        return Wrapper {
            data: Some(model as *mut c_void),
            initialize: Some(initialize),
            update: Some(update),
            update_until: Some(update_until),
            finalize: Some(finalize),
            get_component_name: Some(get_component_name),
            get_input_item_count: Some(get_input_item_count),
            get_output_item_count: Some(get_output_item_count),
            get_input_var_names: Some(get_input_var_names),
            get_output_var_names: Some(get_output_var_names),
            get_var_grid: Some(get_var_grid),
            get_var_type: Some(get_var_type),
            get_var_units: Some(get_var_units),
            get_var_itemsize: Some(get_var_itemsize),
            get_var_nbytes: Some(get_var_nbytes),
            get_var_location: Some(get_var_location),
            get_current_time: Some(get_current_time),
            get_start_time: Some(get_start_time),
            get_end_time: Some(get_end_time),
            get_time_units: Some(get_time_units),
            get_time_step: Some(get_time_step),
            get_value: Some(get_value),
            get_value_ptr: Some(get_value_ptr),
            get_value_at_indices: Some(get_value_at_indices),
            set_value: Some(set_value),
            set_value_at_indices: Some(set_value_at_indices),
            get_grid_rank: Some(get_grid_rank),
            get_grid_size: Some(get_grid_size),
            get_grid_type: Some(get_grid_type),
            get_grid_shape: Some(get_grid_shape),
            get_grid_spacing: Some(get_grid_spacing),
            get_grid_origin: Some(get_grid_origin),
            get_grid_x: Some(get_grid_x),
            get_grid_y: Some(get_grid_y),
            get_grid_z: Some(get_grid_z),
            get_grid_node_count: Some(get_grid_node_count),
            get_grid_edge_count: Some(get_grid_edge_count),
            get_grid_face_count: Some(get_grid_face_count),
            get_grid_edge_nodes: Some(get_grid_edge_nodes),
            get_grid_face_edges: Some(get_grid_face_edges),
            get_grid_face_nodes: Some(get_grid_face_nodes),
            get_grid_nodes_per_face: Some(get_grid_nodes_per_face),
        };
    }
}
impl Wrapper {
    // NOTE: this will not be in final api, stil working out initialization
    pub fn setup(&mut self) {
        self.initialize = Some(initialize);
        self.update = Some(update);
        self.update_until = Some(update_until);
        self.finalize = Some(finalize);
        self.get_component_name = Some(get_component_name);
        self.get_input_item_count = Some(get_input_item_count);
        self.get_output_item_count = Some(get_output_item_count);
        self.get_input_var_names = Some(get_input_var_names);
        self.get_output_var_names = Some(get_output_var_names);
        self.get_var_grid = Some(get_var_grid);
        self.get_var_type = Some(get_var_type);
        self.get_var_units = Some(get_var_units);
        self.get_var_itemsize = Some(get_var_itemsize);
        self.get_var_nbytes = Some(get_var_nbytes);
        self.get_var_location = Some(get_var_location);
        self.get_current_time = Some(get_current_time);
        self.get_start_time = Some(get_start_time);
        self.get_end_time = Some(get_end_time);
        self.get_time_units = Some(get_time_units);
        self.get_time_step = Some(get_time_step);
        self.get_value = Some(get_value);
        self.get_value_ptr = Some(get_value_ptr);
        self.get_value_at_indices = Some(get_value_at_indices);
        self.set_value = Some(set_value);
        self.set_value_at_indices = Some(set_value_at_indices);
        self.get_grid_rank = Some(get_grid_rank);
        self.get_grid_size = Some(get_grid_size);
        self.get_grid_type = Some(get_grid_type);
        self.get_grid_shape = Some(get_grid_shape);
        self.get_grid_spacing = Some(get_grid_spacing);
        self.get_grid_origin = Some(get_grid_origin);
        self.get_grid_x = Some(get_grid_x);
        self.get_grid_y = Some(get_grid_y);
        self.get_grid_z = Some(get_grid_z);
        self.get_grid_node_count = Some(get_grid_node_count);
        self.get_grid_edge_count = Some(get_grid_edge_count);
        self.get_grid_face_count = Some(get_grid_face_count);
        self.get_grid_edge_nodes = Some(get_grid_edge_nodes);
        self.get_grid_face_edges = Some(get_grid_face_edges);
        self.get_grid_face_nodes = Some(get_grid_face_nodes);
        self.get_grid_nodes_per_face = Some(get_grid_nodes_per_face);
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
