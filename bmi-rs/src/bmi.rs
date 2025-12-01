use crate::errors::{BmiIndexOutOfBounds, BmiNotImplementedError};
use std::error::Error;

pub const MAX_COMPONENT_NAME: u32 = 2048;
pub const MAX_VAR_NAME: u32 = 2048;
pub const MAX_UNITS_NAME: u32 = 2048;
pub const MAX_TYPE_NAME: u32 = 2048;

/// Bmi variable grid
/// [element location](https://bmi.csdms.io/en/stable/bmi.var_funcs.html#get-var-location).
#[derive(Debug, Clone, Copy)]
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

/// Bmi
/// [grid type](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-type).
#[derive(Debug, Clone, Copy)]
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

// TODO: how to add isize and usize?
/// Represents the numeric data type of an item in a [`Bmi`] variable's array.
#[derive(Debug, Copy, Clone)]
pub enum ValueType {
    /// signed 16 bit int
    I16,
    /// unsigned 16 bit int
    U16,
    /// signed 32 bit int
    I32,
    /// unsigned 32 bit int
    U32,
    /// signed 64 bit int
    I64,
    /// unsigned 64 bit int
    U64,
    /// signed 32 bit float
    F32,
    /// signed 64 bit float
    F64,
}

impl ValueType {
    /// Return the size in bytes of the variant's analogous numeric type.
    pub fn bytes(&self) -> usize {
        match self {
            ValueType::I16 | ValueType::U16 => 2,
            ValueType::I32 | ValueType::U32 | ValueType::F32 => 4,
            ValueType::I64 | ValueType::U64 | ValueType::F64 => 8,
        }
    }
}

// NOTE: consider a more generic container type than Vec<T>, maybe Box<[T]>?
/// An owned `Vec` of a numeric type wrapped with type information.
#[derive(Debug, Clone)]
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
/// A ref to a slice of numerics wrapped with type information.
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

#[derive(Debug)]
pub enum MutPtrValues {
    I16(*mut i16), // short
    U16(*mut u16), // unsigned short
    I32(*mut i32), // usually int
    U32(*mut u32), // usually unsigned int
    I64(*mut i64), // long or usually long long
    U64(*mut u64), // unsigned long or usually unsigned long long
    F32(*mut f32), // float
    F64(*mut f64), // double
}

macro_rules! impl_from_ref_mut_slice_for_mut_ptr_values {
    ($($name:ident; $t:ty),*$(,)?) => {
        $(
        impl From<&mut [$t]> for MutPtrValues{
            fn from(v: &mut [$t]) -> Self {
                Self::$name(v.as_mut_ptr())
            }
        }
    )*
    };
}

impl_from_ref_mut_slice_for_mut_ptr_values!(
    I16;i16,
    U16;u16,
    I32;i32,
    U32;u32,
    I64;i64,
    U64;u64,
    F32;f32,
    F64;f64,
);

pub type BmiResult<T> = Result<T, Box<dyn Error>>;

macro_rules! values_at_indices {
    ($t:ty, $inds:expr, $values:expr) => {{
        let mut v = Vec::<$t>::with_capacity($inds.len());
        for i in $inds {
            if *i >= $values.len() as u32 {
                return Err(Box::new(BmiIndexOutOfBounds));
            }
            v.push($values[*i as usize]);
        }
        Ok(Values::from(v))
    }};
}

/// [CSDMS Basic Model Interface (BMI)](https://bmi.csdms.io/en/latest/index.html)
/// _like_ trait.
///
/// Types that implement this trait can be exposed over the
/// [bmi-c interface](https://github.com/csdms/bmi-c) via
/// [`register_model`].
pub trait Bmi {
    /// [`Bmi`] implementations should perform the majority of tasks that are to take place before
    /// entering the model’s time loop in this method.
    /// Exceptions to this are [`Bmi`] implementations that expose model parameters settable for
    /// configuration or calibration purposes.
    ///
    /// Code using [`Bmi`] implementations are expected to call the implementation's `initialize`
    /// member before any other [`Bmi`] trait members.
    ///
    /// See
    /// [csdms bmi `initialize`](https://bmi.csdms.io/en/stable/bmi.control_funcs.html#initialize)
    /// docs for more info.
    fn initialize(&mut self, config_file: &str) -> BmiResult<()>;

    /// Advance the model by a single [`get_time_step`] sized time step.
    ///
    /// See
    /// [csdms bmi `update`](https://bmi.csdms.io/en/stable/bmi.control_funcs.html#update)
    /// docs for more info.
    ///
    /// [`get_time_step`]: #tymethod.get_time_step
    fn update(&mut self) -> BmiResult<()>;

    // TODO: consider using something like Chrono instead of f64
    /// Advance the model to the time at `then`.
    /// Once called, the value returned by the [`get_current_time`] function must return the
    /// provided time to reflect that the model was updated to the requested time.
    ///
    /// See
    /// [csdms bmi `update_until`](https://bmi.csdms.io/en/stable/bmi.control_funcs.html#update-until)
    /// docs for more info.
    ///
    /// [`get_current_time`]: #tymethod.get_current_time
    fn update_until(&mut self, then: f64) -> BmiResult<()>;

    /// Perform any necessary tasks after exiting the model’s time loop.
    /// Note, the implementing type is not consumed and therefore not dropped.
    ///
    /// Code using [`Bmi`] implementations are expected to call the implementation's [`finalize`]
    /// member as the last _[`Bmi`]_ interaction with the implementing type.
    ///
    /// FFI methods that wrap [`finalize`] _should_ drop the implementing type.
    ///
    /// See
    /// [csdms bmi `finalize`](https://bmi.csdms.io/en/stable/bmi.control_funcs.html#finalize)
    /// docs for more info.
    ///
    /// [`finalize`]: #tymethod.finalize
    fn finalize(&mut self) -> BmiResult<()>;

    /* Exchange items */
    /// Return the model's name.
    ///
    /// See
    /// [csdms bmi `get_component_name`](https://bmi.csdms.io/en/stable/bmi.info_funcs.html#get-component-name)
    /// docs for more info.
    fn get_component_name(&self) -> &str;

    /// Return the number of model _input variables_ settable via [`set_value`].
    /// The count is given by the length of slice returned by [`get_input_var_names`].
    ///
    /// Note, [`Bmi`] implementations that expose model parameters settable strictly for
    /// configuration or calibration purposes should not include these parameters in their
    /// [`get_input_item_count`] count.
    ///
    /// See
    /// [csdms bmi `get_input_item_count`](https://bmi.csdms.io/en/stable/bmi.info_funcs.html#get-input-item-count)
    /// docs for more info.
    ///
    /// [`set_value`]: #tymethod.set_value
    /// [`get_input_var_names`]: #tymethod.get_input_var_names
    /// [`get_input_item_count`]: #tymethod.get_input_item_count
    fn get_input_item_count(&self) -> u32 {
        self.get_input_var_names().len() as u32
    }

    /// Return the number of _model output_ variables retrievable via [`get_value_ptr`].
    /// The count is given by the length of slice returned by [`get_output_var_names`].
    ///
    /// See
    /// [csdms bmi `get_output_item_count`](https://bmi.csdms.io/en/stable/bmi.info_funcs.html#get-output-item-count)
    /// docs for more info.
    ///
    /// [`get_value_ptr`]: #tymethod.get_value_ptr
    /// [`get_output_var_names`]: #tymethod.get_output_var_names
    fn get_output_item_count(&self) -> u32 {
        self.get_output_var_names().len() as u32
    }

    /// Return the implementing model's input variable names.
    /// The length of the array is given by [`get_input_item_count`].
    ///
    /// Names are preferably in the form of
    /// [CSDMS Standard Names](https://csdms.colorado.edu/wiki/CSDMS_Standard_Names).
    ///
    /// See
    /// [csdms bmi `get_input_var_names`](https://bmi.csdms.io/en/stable/bmi.info_funcs.html#get-input-var-names)
    /// docs for more info.
    ///
    /// [`get_input_item_count`]: #tymethod.get_input_item_count
    fn get_input_var_names(&self) -> &[&str];

    /// Return the implementing model's output variable names.
    /// The length of the array is given by [`get_output_item_count`].
    ///
    /// See
    /// [csdms bmi `get_output_var_names`](https://bmi.csdms.io/en/stable/bmi.info_funcs.html#get-output-var-names)
    /// docs for more info.
    ///
    /// [`get_output_item_count`]: #tymethod.get_output_item_count
    fn get_output_var_names(&self) -> &[&str];

    /* Variable information */
    /// Return the input or output variable's grid type.
    ///
    /// See
    /// [csdms bmi `get_var_grid`](https://bmi.csdms.io/en/stable/bmi.var_funcs.html#get-var-grid)
    /// docs for more info.
    fn get_var_grid(&self, name: &str) -> BmiResult<i32>;

    /// Return a variable's inner type.
    ///
    /// See
    /// [csdms bmi `get_var_type`](https://bmi.csdms.io/en/stable/bmi.var_funcs.html#get-var-type)
    /// docs for more info.
    fn get_var_type(&self, name: &str) -> BmiResult<ValueType>;

    /// Return a variable's units.
    /// Units should be returned in lower case in
    /// [UDUNIT2 format](https://docs.unidata.ucar.edu/udunits/current/#Database).
    /// For example, `"meters"` / `"m"` or `"seconds"` / `"s"`.
    ///
    /// See
    /// [csdms bmi `get_var_units`](https://bmi.csdms.io/en/stable/bmi.var_funcs.html#get-var-units)
    /// docs for more info.
    fn get_var_units(&self, name: &str) -> BmiResult<&str>;

    /// Return the size, in bytes, of a single element of the variable.
    /// For example, a variable stored as a `Vec<f64>` has an itemsize of 8.
    ///
    /// See
    /// [csdms bmi `get_var_itemsize`](https://bmi.csdms.io/en/stable/bmi.var_funcs.html#get-var-itemsize)
    /// docs for more info.
    fn get_var_itemsize(&self, name: &str) -> BmiResult<u32> {
        Ok(self.get_var_type(name)?.bytes() as u32)
    }

    /// Return the total number of bytes used to store a variable.
    /// i.e., the number of items multiplied by the size of each item.
    ///
    /// See
    /// [csdms bmi `get_var_nbytes`](https://bmi.csdms.io/en/stable/bmi.var_funcs.html#get-var-nbytes)
    /// docs for more info.
    fn get_var_nbytes(&self, name: &str) -> BmiResult<u32>;

    /// Return a variable's grid element type.
    ///
    /// See
    /// [csdms bmi `get_var_location`](https://bmi.csdms.io/en/stable/bmi.var_funcs.html#get-var-location)
    /// docs for more info.
    fn get_var_location(&self, name: &str) -> BmiResult<Location>;

    /* Time information */
    /// Return the model's current time.
    ///
    /// See
    /// [csdms bmi `get_current_time`](https://bmi.csdms.io/en/stable/bmi.time_funcs.html#get-current-time)
    /// docs for more info.
    fn get_current_time(&self) -> f64;

    /// Return the model's simulation start time.
    /// Default: `0.0`.
    ///
    /// Note, the start time is typically `0.0`.
    ///
    /// See
    /// [csdms bmi `get_start_time`](https://bmi.csdms.io/en/stable/bmi.time_funcs.html#get-start-time)
    /// docs for more info.
    fn get_start_time(&self) -> f64 {
        0.
    }

    /// Return the model's simulation end time.
    /// Default: [`f64::MAX`].
    ///
    /// Note, if an end time does not conceptually make sense, [`f64::MAX`] should be used.
    ///
    /// See
    /// [csdms bmi `get_end_time`](https://bmi.csdms.io/en/stable/bmi.time_funcs.html#get-end-time)
    /// docs for more info.
    ///
    /// [`f64::MAX`]: https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MAX
    fn get_end_time(&self) -> f64 {
        f64::MAX
    }

    /// Return the model's time unit.
    ///
    /// Units in CF conventions are recommended.
    ///
    /// e.g. `s` | `sec` | `second`, `min` | `minute`, `h` | `hr` | `hour`, or `d` | `day`.
    ///
    /// See
    /// [csdms bmi `get_time_units`](https://bmi.csdms.io/en/stable/bmi.time_funcs.html#get-time-units)
    /// docs for more info.
    fn get_time_units(&self) -> &str;

    /// Return the model's time step size in [`get_time_units`] units.
    ///
    /// See
    /// [csdms bmi `get_time_step`](https://bmi.csdms.io/en/stable/bmi.time_funcs.html#get-time-step)
    /// docs for more info.
    ///
    /// [`get_time_units`]: #tymethod.get_time_units
    fn get_time_step(&self) -> f64;

    /* Getters */
    /// Return a reference to a flattened slice of values for a given variable.
    ///
    /// Note, [`Bmi`] does not include the BMI `get_value` method in its method set.
    /// This may change in the future.
    /// Likewise, the return type of [`get_value_ptr`] may change in future versions.
    /// See discussion in [#3](https://github.com/aaraney/bmi-rs/issues/3).
    ///
    /// See
    /// [csdms bmi `get_value_ptr`](https://bmi.csdms.io/en/stable/bmi.getter_setter.html#get-value-ptr)
    /// docs for more info.
    ///
    /// [`get_value_ptr`]: #tymethod.get_value_ptr
    fn get_value_ptr(&self, name: &str) -> BmiResult<RefValues<'_>>;

    /// Return a [`MutPtrValues`] to a flattened slice of values for a given variable.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// This crate's bmi-c ffi bindings call a [`Bmi`]'s [`get_value_mut_ptr`] method when
    /// the bmi-c
    /// [`get_value_ptr`](https://bmi.csdms.io/en/stable/bmi.getter_setter.html#get-value-ptr)
    /// function pointer is called. Code calling the [`Bmi`] instance
    /// should use this method to read or write data to a variable the _model chooses to expose by
    /// pointer_. The code calling the [`Bmi`] instance must be aware of the lifetime guarantees of
    /// the returned pointer. It's recommended that [`Bmi`] instances that implement this method
    /// provide a SAFETY comment documenting the lifetime guarantees of variables exposed by their
    /// implementation.
    ///
    /// See
    /// [csdms bmi `get_value_ptr`](https://bmi.csdms.io/en/stable/bmi.getter_setter.html#get-value-ptr)
    /// docs for more info.
    ///
    /// [`get_value_mut_ptr`]: #tymethod.get_value_mut_ptr
    #[allow(unused_variables)]
    unsafe fn get_value_mut_ptr(&self, name: &str) -> BmiResult<MutPtrValues> {
        BmiNotImplementedError.into()
    }

    /// Return an owned copy of a variable’s values at the `inds` specified.
    ///
    /// Note, the default implementation copies from values via [`get_value_ptr`].
    ///
    /// See
    /// [csdms bmi `get_value_at_indices`](https://bmi.csdms.io/en/stable/bmi.getter_setter.html#get-value-at-indices)
    /// docs for more info.
    ///
    /// [`get_value_ptr`]: #tymethod.get_value_ptr
    fn get_value_at_indices(&self, name: &str, inds: &[u32]) -> BmiResult<Values> {
        match self.get_value_ptr(name)? {
            RefValues::I16(items) => values_at_indices!(i16, inds, items),
            RefValues::U16(items) => values_at_indices!(u16, inds, items),
            RefValues::I32(items) => values_at_indices!(i32, inds, items),
            RefValues::U32(items) => values_at_indices!(u32, inds, items),
            RefValues::I64(items) => values_at_indices!(i64, inds, items),
            RefValues::U64(items) => values_at_indices!(u64, inds, items),
            RefValues::F32(items) => values_at_indices!(f32, inds, items),
            RefValues::F64(items) => values_at_indices!(f64, inds, items),
        }
    }

    /* Setters */
    /// Copy values from `src` into the model's `name` variable.
    /// `src`'s [`RefValues`] variant and slice length _must_ match the analogous type _and_ length of
    /// the model's internal `name` variable.
    /// For example, if `src` is a [`RefValues::F64`] an _item_ in model's `name` variable array
    /// must be an `f64`.
    ///
    /// The type and length of a model's variable can be determined through calls to
    /// [`get_var_type`] and [`get_var_nbytes`].
    ///
    /// See
    /// [csdms bmi `set_value`](https://bmi.csdms.io/en/stable/bmi.getter_setter.html#set-value)
    /// docs for more info.
    ///
    /// [`get_var_type`]: #tymethod.get_var_type
    /// [`get_var_nbytes`]: #tymethod.get_var_nbytes
    fn set_value(&mut self, name: &str, src: RefValues) -> BmiResult<()>;

    /// Copy values from `src` into the model's `name` variable at the provided `inds` indices.
    ///
    /// See
    /// [csdms bmi `set_value_at_indices`](https://bmi.csdms.io/en/stable/bmi.getter_setter.html#set-value-at-indices)
    /// docs for more info.
    fn set_value_at_indices(&mut self, name: &str, inds: &[u32], src: RefValues) -> BmiResult<()>;

    // NOTE: can we implement a default here?
    /// Return the [`GridType`] for a given grid identifier.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_type`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-type)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_type(&self, grid: i32) -> BmiResult<GridType> {
        BmiNotImplementedError.into()
    }

    /* Grid information */
    /// Return the grid
    /// [rank](https://bmi.csdms.io/en/stable/glossary.html#term-rank)
    /// for a given grid identifier.
    ///
    /// This function is needed for every
    /// [grid type](https://bmi.csdms.io/en/stable/model_grids.html#model-grids).
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_rank`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-rank)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_rank(&self, grid: i32) -> BmiResult<u32> {
        BmiNotImplementedError.into()
    }

    /// Return the total number of elements (or
    /// [nodes](https://bmi.csdms.io/en/stable/glossary.html#term-node)
    /// ) for a given grid identifier.
    ///
    /// This function is needed for every
    /// [grid type](https://bmi.csdms.io/en/stable/model_grids.html#model-grids).
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_size`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-size)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_size(&self, grid: i32) -> BmiResult<u32> {
        BmiNotImplementedError.into()
    }

    /* Uniform rectilinear */
    /// Return the dimensions of the model grid for a given a grid identifier.
    /// The length of the returned slice is [`get_grid_rank`] long.
    ///
    /// This function is used for describing all
    /// [structured grids](https://bmi.csdms.io/en/stable/model_grids.html#structured-grids).
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_shape`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-shape)
    /// docs for more info.
    ///
    /// [`get_grid_rank`]: #tymethod.get_grid_rank
    #[allow(unused_variables)]
    fn get_grid_shape(&self, grid: i32) -> BmiResult<&[u32]> {
        BmiNotImplementedError.into()
    }

    /// Return the distance between the
    /// [nodes](https://bmi.csdms.io/en/stable/glossary.html#term-node)
    /// of the model grid.
    ///
    /// This function is used for describing
    /// [uniform rectilinear](https://bmi.csdms.io/en/stable/model_grids.html#uniform-rectilinear)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_spacing`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-spacing)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_spacing(&self, grid: i32) -> BmiResult<&[f64]> {
        BmiNotImplementedError.into()
    }

    /// Return the coordinates of the lower-left corner of the model grid.
    ///
    /// This function is used for describing
    /// [uniform rectilinear](https://bmi.csdms.io/en/stable/model_grids.html#uniform-rectilinear)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_origin`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-origin)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_origin(&self, grid: i32) -> BmiResult<&[f64]> {
        BmiNotImplementedError.into()
    }

    /* Non-uniform rectilinear, curvilinear */
    /// Return locations of the grid
    /// [nodes](https://bmi.csdms.io/en/stable/glossary.html#term-node)
    /// in the first coordinate direction.
    ///
    /// The length of the resulting one-dimensional array depends on the grid type.
    ///
    /// This function is used for describing
    /// [rectilinear](https://bmi.csdms.io/en/stable/model_grids.html#rectilinear),
    /// [structured quadrilateral](https://bmi.csdms.io/en/stable/model_grids.html#structured-quad),
    /// and all
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_rank`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-x)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_x(&self, grid: i32) -> BmiResult<&[f64]> {
        BmiNotImplementedError.into()
    }

    /// Return locations of the grid
    /// [nodes](https://bmi.csdms.io/en/stable/glossary.html#term-node)
    /// in the second coordinate direction.
    ///
    /// The length of the resulting one-dimensional array depends on the grid type.
    ///
    /// This function is used for describing
    /// [rectilinear](https://bmi.csdms.io/en/stable/model_grids.html#rectilinear),
    /// [structured quadrilateral](https://bmi.csdms.io/en/stable/model_grids.html#structured-quad),
    /// and all
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_rank`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-y)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_y(&self, grid: i32) -> BmiResult<&[f64]> {
        BmiNotImplementedError.into()
    }

    /// Return locations of the grid
    /// [nodes](https://bmi.csdms.io/en/stable/glossary.html#term-node)
    /// in the third coordinate direction.
    ///
    /// The length of the resulting one-dimensional array depends on the grid type.
    ///
    /// This function is used for describing
    /// [rectilinear](https://bmi.csdms.io/en/stable/model_grids.html#rectilinear),
    /// [structured quadrilateral](https://bmi.csdms.io/en/stable/model_grids.html#structured-quad),
    /// and all
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_rank`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-z)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_z(&self, grid: i32) -> BmiResult<&[f64]> {
        BmiNotImplementedError.into()
    }

    /* Unstructured */
    /// Get the number of
    /// [nodes](https://bmi.csdms.io/en/stable/glossary.html#term-node)
    /// in the grid.
    ///
    /// This function is used for describing
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_node_count`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-node-count)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_node_count(&self, grid: i32) -> BmiResult<u32> {
        BmiNotImplementedError.into()
    }

    /// Get the number of
    /// [edges](https://bmi.csdms.io/en/stable/glossary.html#term-edge)
    /// in the grid.
    ///
    /// This function is used for describing
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_edge_count`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-edge-count)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_edge_count(&self, grid: i32) -> BmiResult<u32> {
        BmiNotImplementedError.into()
    }

    /// Get the number of
    /// [faces](https://bmi.csdms.io/en/stable/glossary.html#term-face)
    /// in the grid.
    ///
    /// This function is used for describing
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_face_count`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-face-count)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_face_count(&self, grid: i32) -> BmiResult<u32> {
        BmiNotImplementedError.into()
    }

    /// Return the edge-node connectivity.
    /// The total length of the slice is 2 * [`get_grid_edge_count`].
    ///
    /// This function is used for describing
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_edge_nodes`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-edge-nodes)
    /// docs for more info.
    ///
    /// [`get_grid_edge_count`]: #tymethod.get_grid_edge_count
    #[allow(unused_variables)]
    fn get_grid_edge_nodes(&self, grid: i32) -> BmiResult<&[u32]> {
        BmiNotImplementedError.into()
    }

    /// Return the face-edge connectivity.
    /// The length of the returned slice is the sum of the values of [`get_grid_nodes_per_face`].
    ///
    /// This function is used for describing
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_face_edges`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-face-edges)
    /// docs for more info.
    ///
    /// [`get_grid_nodes_per_face`]: #tymethod.get_grid_nodes_per_face
    #[allow(unused_variables)]
    fn get_grid_face_edges(&self, grid: i32) -> BmiResult<&[u32]> {
        BmiNotImplementedError.into()
    }

    /// Return the face-node connectivity.
    ///
    /// This function is used for describing
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_face_nodes`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-face-nodes)
    /// docs for more info.
    #[allow(unused_variables)]
    fn get_grid_face_nodes(&self, grid: i32) -> BmiResult<&[u32]> {
        BmiNotImplementedError.into()
    }

    /// Return the number of nodes for each face.
    /// The returned array has a length of [`get_grid_face_count`].
    ///
    /// This function is used for describing
    /// [unstructured](https://bmi.csdms.io/en/stable/model_grids.html#unstructured-grids)
    /// grids.
    ///
    /// Default implementation returns Err([`BmiNotImplementedError`]).
    ///
    /// See
    /// [csdms bmi `get_grid_nodes_per_face`](https://bmi.csdms.io/en/stable/bmi.grid_funcs.html#get-grid-nodes-per-face)
    /// docs for more info.
    ///
    /// [`get_grid_face_count`]: #tymethod.get_grid_face_count
    #[allow(unused_variables)]
    fn get_grid_nodes_per_face(&self, grid: i32) -> BmiResult<&[u32]> {
        BmiNotImplementedError.into()
    }
}

/// Bootstraps the `model` so it can be called through the
/// [bmi-c](https://github.com/csdms/bmi-c/blob/031c5abf0ff0e75bec7aea48a064611138a0de64/bmi.h)
/// interface.
///
/// Example:
/// ```compile_fail
/// #[unsafe(no_mangle)]
///  pub extern "C" fn register_bmi_simple(handle: *mut ffi::Bmi) -> *mut ffi::Bmi {
///      let model = Model::new();
///      bmi_rs::register_model(handle, model);
///      return handle;
///  }
///  ```
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

#[cfg(test)]
mod tests {
    use super::*;
    fn case(vs: &[u16], idx: &[u32]) -> Result<Values, Box<dyn Error>> {
        values_at_indices!(u16, idx, vs)
    }
    #[test]
    fn test_empty() {
        let vs: [u16; 0] = [];
        let inds: [u32; 0] = [];
        assert!(case(&vs, &inds).is_ok());

        let vs: [u16; 1] = [42];
        let inds: [u32; 0] = [];
        assert!(case(&vs, &inds).is_ok());
    }

    #[test]
    fn test_one() {
        let vs: [u16; 1] = [42];
        let inds: [u32; 1] = [0];
        match case(&vs, &inds) {
            Ok(values) => match values {
                Values::U16(values) => {
                    let i = inds[0] as usize;
                    assert_eq!(values[i], vs[i]);
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_many() {
        let vs: [u16; 2] = [0, 1];
        let inds: [u32; 2] = [0, 1];
        match case(&vs, &inds) {
            Ok(values) => match values {
                Values::U16(values) => {
                    for i in &inds {
                        let i = *i as usize;
                        assert_eq!(values[i], vs[i]);
                    }
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_out_of_bounds() {
        let vs: [u16; 0] = [];
        let inds: [u32; 1] = [1];
        match case(&vs, &inds) {
            Err(err) => {
                assert!(err.is::<crate::errors::BmiIndexOutOfBounds>());
            }
            _ => assert!(false),
        }
    }
}
