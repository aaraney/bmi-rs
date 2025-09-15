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

fn into<T>(value: &*mut ffi::Bmi) -> &mut T
where
    T: Bmi + Sized,
{
    let foo = unsafe { value.as_mut() }.unwrap();
    unsafe { std::mem::transmute(foo.data) }
}

pub extern "C" fn initialize<T: Bmi>(self_: *mut ffi::Bmi, config_file: *const c_char) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(config_file) };
    let Ok(config_file) = c_str.to_str() else {
        return BMI_FAILURE;
    };
    let data: &mut T = into(&self_);
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
    let data: &mut T = into(&self_);
    data.update().bmi_result()
}

pub extern "C" fn update_until<T: Bmi>(self_: *mut ffi::Bmi, then: c_double) -> c_int {
    let data: &mut T = into(&self_);
    data.update_until(then).bmi_result()
}

pub extern "C" fn finalize<T: Bmi>(self_: *mut ffi::Bmi) -> c_int {
    let data: &mut T = into(&self_);
    // drop data field
    let _ = unsafe { Box::from_raw(data as *mut T) };
    return BMI_SUCCESS;
}

pub extern "C" fn get_component_name<T: Bmi>(self_: *mut ffi::Bmi, name: *mut c_char) -> c_int {
    let data: &mut T = into(&self_);
    copy_str(data.get_component_name(), name).bmi_result()
}

pub extern "C" fn get_input_item_count<T: Bmi>(self_: *mut ffi::Bmi, count: *mut c_int) -> c_int {
    let data: &mut T = into(&self_);
    unsafe { *count = data.get_input_item_count() as c_int };
    return BMI_SUCCESS;
}

pub extern "C" fn get_output_item_count<T: Bmi>(self_: *mut ffi::Bmi, count: *mut c_int) -> c_int {
    let data: &mut T = into(&self_);
    unsafe { *count = data.get_output_item_count() as c_int };
    return BMI_SUCCESS;
}

// NOTE: I not sure if the double pointer is right or not?
pub extern "C" fn get_input_var_names<T: Bmi>(
    self_: *mut ffi::Bmi,
    names: *mut *mut c_char,
) -> c_int {
    let data: &mut T = into(&self_);
    let var_names = data.get_input_var_names();
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

pub extern "C" fn get_output_var_names<T: Bmi>(
    self_: *mut ffi::Bmi,
    names: *mut *mut c_char,
) -> c_int {
    let data: &mut T = into(&self_);
    let var_names = data.get_output_var_names();
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
pub extern "C" fn get_var_grid<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    grid: *mut c_int,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);
    let Ok(grid_id) = data.get_var_grid(var_name) else {
        return BMI_FAILURE;
    };
    unsafe { *grid = grid_id };
    return BMI_FAILURE;
}

pub extern "C" fn get_var_type<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    ty: *mut c_char,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);
    let Ok(var_type) = data.get_var_type(var_name) else {
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

    copy_str(var_type, ty).bmi_result()
}
pub extern "C" fn get_var_units<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    units: *mut c_char,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);
    let Ok(var_units) = data.get_var_units(var_name) else {
        return BMI_FAILURE;
    };

    copy_str(var_units, units).bmi_result()
}
pub extern "C" fn get_var_itemsize<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    size: *mut c_int,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);
    let Ok(item_size) = data.get_var_itemsize(var_name) else {
        return BMI_FAILURE;
    };
    unsafe { *size = item_size as i32 };
    return BMI_SUCCESS;
}
pub extern "C" fn get_var_nbytes<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    nbytes: *mut c_int,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);
    let Ok(var_nbytes) = data.get_var_nbytes(var_name) else {
        return BMI_FAILURE;
    };
    unsafe { *nbytes = var_nbytes as i32 };
    return BMI_SUCCESS;
}
pub extern "C" fn get_var_location<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    location: *mut c_char,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);
    let Ok(var_location) = data.get_var_location(var_name) else {
        return BMI_FAILURE;
    };

    copy_str(var_location.to_string().as_str(), location).bmi_result()
}

/* Time information */
pub extern "C" fn get_current_time<T: Bmi>(self_: *mut ffi::Bmi, time: *mut c_double) -> c_int {
    let data: &mut T = into(&self_);
    unsafe { *time = data.get_current_time() };
    return BMI_SUCCESS;
}
pub extern "C" fn get_start_time<T: Bmi>(self_: *mut ffi::Bmi, time: *mut c_double) -> c_int {
    let data: &mut T = into(&self_);
    unsafe { *time = data.get_start_time() };
    return BMI_SUCCESS;
}
pub extern "C" fn get_end_time<T: Bmi>(self_: *mut ffi::Bmi, time: *mut c_double) -> c_int {
    let data: &mut T = into(&self_);
    unsafe { *time = data.get_end_time() };
    return BMI_SUCCESS;
}
pub extern "C" fn get_time_units<T: Bmi>(self_: *mut ffi::Bmi, units: *mut c_char) -> c_int {
    let data: &mut T = into(&self_);
    copy_str(data.get_time_units(), units).bmi_result()
}
pub extern "C" fn get_time_step<T: Bmi>(self_: *mut ffi::Bmi, time_step: *mut c_double) -> c_int {
    let data: &mut T = into(&self_);
    unsafe { *time_step = data.get_time_step() };
    return BMI_SUCCESS;
}

// TODO:
// /* Getters */
pub extern "C" fn get_value<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    dest: *mut c_void,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);
    let Ok(value) = data.get_value(var_name) else {
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
pub extern "C" fn get_value_ptr<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    dest: *mut *mut c_void,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);

    let Ok(value_ptr) = data.get_value_ptr(var_name) else {
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

pub extern "C" fn get_value_at_indices<T: Bmi>(
    self_: *mut ffi::Bmi,
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

    let data: &mut T = into(&self_);
    let Ok(value) = data.get_value_at_indices(var_name, &var_ids) else {
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
pub extern "C" fn set_value<T: Bmi>(
    self_: *mut ffi::Bmi,
    name: *const c_char,
    value: *mut c_void,
) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let Ok(var_name) = c_str.to_str() else {
        return BMI_FAILURE;
    };

    let data: &mut T = into(&self_);
    let Ok(var_type) = data.get_var_type(var_name) else {
        return BMI_FAILURE;
    };

    let Ok(var_nbytes) = data.get_var_nbytes(var_name) else {
        return BMI_FAILURE;
    };
    let len = var_nbytes as usize / var_type.bytes();

    let res = match var_type {
        ValueType::I16 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i16, len) };
            let src: Vec<i16> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            data.set_value(var_name, &src)
        }
        ValueType::U16 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u16, len) };
            let src: Vec<u16> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            data.set_value(var_name, &src)
        }
        ValueType::I32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i32, len) };
            let src: Vec<i32> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            data.set_value(var_name, &src)
        }
        ValueType::U32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u32, len) };
            let src: Vec<u32> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            data.set_value(var_name, &src)
        }
        ValueType::I64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut i64, len) };
            let src: Vec<i64> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            data.set_value(var_name, &src)
        }
        ValueType::U64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut u64, len) };
            let src: Vec<u64> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            data.set_value(var_name, &src)
        }
        ValueType::F32 => {
            let src = unsafe { slice::from_raw_parts(value as *mut f32, len) };
            let src: Vec<f32> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            data.set_value(var_name, &src)
        }
        ValueType::F64 => {
            let src = unsafe { slice::from_raw_parts(value as *mut f64, len) };
            let src: Vec<f64> = src.iter().cloned().collect();
            let src: ValueVec = src.into();
            data.set_value(var_name, &src)
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

    let data: &mut T = into(&self_);
    let Ok(var_type) = data.get_var_type(var_name) else {
        return BMI_FAILURE;
    };

    let res = match var_type {
        ValueType::I16 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i16, count) };
            let src: Vec<i16> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            data.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::U16 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u16, count) };
            let src: Vec<u16> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            data.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::I32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i32, count) };
            let src: Vec<i32> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            data.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::U32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u32, count) };
            let src: Vec<u32> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            data.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::I64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut i64, count) };
            let src: Vec<i64> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            data.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::U64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut u64, count) };
            let src: Vec<u64> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            data.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::F32 => {
            let src = unsafe { slice::from_raw_parts(src as *mut f32, count) };
            let src: Vec<f32> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            data.set_value_at_indices(var_name, &var_ids, &src)
        }
        ValueType::F64 => {
            let src = unsafe { slice::from_raw_parts(src as *mut f64, count) };
            let src: Vec<f64> = src.iter().cloned().collect();
            let src: RefValueVec = (&src).into();
            data.set_value_at_indices(var_name, &var_ids, &src)
        }
    };
    res.bmi_result()
}

/* Grid information */
pub extern "C" fn get_grid_rank<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    rank: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_rank) = data.get_grid_rank(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *rank = grid_rank as i32 };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_size<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    size: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_size) = data.get_grid_size(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *size = grid_size as i32 };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_type<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    ty: *mut c_char,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_type) = data.get_grid_type(grid) else {
        return BMI_FAILURE;
    };
    copy_str(grid_type.to_string().as_str(), ty).bmi_result()
}

/* Uniform rectilinear */
pub extern "C" fn get_grid_shape<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    shape: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_shape) = data.get_grid_shape(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *shape = grid_shape };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_spacing<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    spacing: *mut c_double,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_spacing) = data.get_grid_spacing(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *spacing = grid_spacing };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_origin<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    origin: *mut c_double,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_origin) = data.get_grid_origin(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *origin = grid_origin };
    return BMI_SUCCESS;
}

/* Non-uniform rectilinear, curvilinear */
pub extern "C" fn get_grid_x<T: Bmi>(self_: *mut ffi::Bmi, grid: c_int, x: *mut c_double) -> c_int {
    let data: &mut T = into(&self_);

    let Ok(grid_x) = data.get_grid_x(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *x = grid_x };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_y<T: Bmi>(self_: *mut ffi::Bmi, grid: c_int, y: *mut c_double) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_y) = data.get_grid_y(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *y = grid_y };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_z<T: Bmi>(self_: *mut ffi::Bmi, grid: c_int, z: *mut c_double) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_z) = data.get_grid_z(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *z = grid_z };
    return BMI_SUCCESS;
}

/* Unstructured */
pub extern "C" fn get_grid_node_count<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(node_count) = data.get_grid_node_count(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *count = node_count };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_edge_count<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(edge_count) = data.get_grid_edge_count(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *count = edge_count };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_face_count<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    count: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(face_count) = data.get_grid_face_count(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *count = face_count };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_edge_nodes<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    edge_nodes: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_edge_nodes) = data.get_grid_edge_nodes(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *edge_nodes = grid_edge_nodes };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_face_edges<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    face_edges: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_face_edges) = data.get_grid_face_edges(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *face_edges = grid_face_edges };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_face_nodes<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    face_nodes: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_face_nodes) = data.get_grid_face_nodes(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *face_nodes = grid_face_nodes };
    return BMI_SUCCESS;
}
pub extern "C" fn get_grid_nodes_per_face<T: Bmi>(
    self_: *mut ffi::Bmi,
    grid: c_int,
    nodes_per_face: *mut c_int,
) -> c_int {
    let data: &mut T = into(&self_);
    let Ok(grid_nodes_per_face) = data.get_grid_nodes_per_face(grid) else {
        return BMI_FAILURE;
    };
    unsafe { *nodes_per_face = grid_nodes_per_face };
    return BMI_SUCCESS;
}
