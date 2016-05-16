#![allow(non_camel_case_types)]
#![allow(dead_code)]

// Ported from ZoL 0.6.5.4's libzfs/sys/nvpair.h header file.
// and using https://illumos.org/man/3nvpair


use std::os::raw::{c_char, c_int, c_uint};
pub type uint_t = c_uint;
pub type int = c_int;

#[repr(C)]
#[derive(PartialEq, Debug, Clone)]
pub enum boolean_t { B_FALSE, B_TRUE }

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum data_type_t {
	DATA_TYPE_UNKNOWN = 0,
	DATA_TYPE_BOOLEAN,
	DATA_TYPE_BYTE,
	DATA_TYPE_INT16,
	DATA_TYPE_UINT16,
	DATA_TYPE_INT32,
	DATA_TYPE_UINT32,
	DATA_TYPE_INT64,
	DATA_TYPE_UINT64,
	DATA_TYPE_STRING,
	DATA_TYPE_BYTE_ARRAY,
	DATA_TYPE_INT16_ARRAY,
	DATA_TYPE_UINT16_ARRAY,
	DATA_TYPE_INT32_ARRAY,
	DATA_TYPE_UINT32_ARRAY,
	DATA_TYPE_INT64_ARRAY,
	DATA_TYPE_UINT64_ARRAY,
	DATA_TYPE_STRING_ARRAY,
	DATA_TYPE_HRTIME,
	DATA_TYPE_NVLIST,
	DATA_TYPE_NVLIST_ARRAY,
	DATA_TYPE_BOOLEAN_VALUE,
	DATA_TYPE_INT8,
	DATA_TYPE_UINT8,
	DATA_TYPE_BOOLEAN_ARRAY,
	DATA_TYPE_INT8_ARRAY,
	DATA_TYPE_UINT8_ARRAY,
}

#[repr(C)]
pub struct nvpair_t {
	pub nvp_size:       i32,          /* size of this nvpair */
	pub nvp_name_sz:    i16,          /* length of name string */
	pub nvp_reserve:    i16,          /* not used */
	pub nvp_value_elem: i32,          /* number of elements for array types */
	pub nvp_type:        data_type_t, /* type of value */

	/* name string */
	/* aligned ptr array for string arrays */
	/* aligned array of data for value */
}

#[repr(C)]
pub struct nvlist_t {
	pub nvl_version: i32,
	pub nvl_nvflag:  u32,	/* persistent flags */
	pub nvl_priv:    u64,	/* ptr to private data if not packed */
	pub nvl_flag:    u32,
	pub nvl_pad:     i32,	/* currently not used, for alignment */
}

const NV_UNIQUE_NAME: uint_t = 1;
const NV_UNIQUE_NAME_TYPE: uint_t = 2;

#[link(name="nvpair", kind="dylib")]
extern {
    pub fn nvlist_alloc(target: *mut *mut nvlist_t, nvflag: uint_t, flag: int) -> c_int;
    pub fn nvlist_free(list: *mut nvlist_t);
    pub fn nvlist_empty(list: *mut nvlist_t) -> boolean_t;
    pub fn nvlist_exists(list: *mut nvlist_t, name: *const c_char) -> boolean_t;
    pub fn nvlist_remove(list: *mut nvlist_t, name: *const c_char, data_type: data_type_t) -> c_int;

    pub fn nvlist_next_nvpair(list: *mut nvlist_t, pair: *mut nvpair_t) -> *mut nvpair_t;
    pub fn nvpair_name(pair: *const nvpair_t) -> *const c_char;
    pub fn nvpair_type(pair: *const nvpair_t) -> data_type_t;

    pub fn nvlist_add_boolean(list: *mut nvlist_t, name: *const c_char) -> c_int;
    pub fn nvlist_lookup_boolean(list: *const nvlist_t, name: *const c_char) -> c_int;

    pub fn nvlist_add_boolean_value(list: *mut nvlist_t, name: *const c_char , val: boolean_t) -> c_int;
    pub fn nvlist_lookup_boolean_value(list: *const nvlist_t, name: *const c_char, val: *mut boolean_t) -> c_int;
    pub fn nvpair_value_boolean_value(pair: *const nvpair_t, val: *mut boolean_t) -> c_int;

    pub fn nvlist_add_boolean_array(list: *const nvlist_t, name: *const c_char, vals: *mut boolean_t, nr_elem: uint_t) -> c_int;
    pub fn nvpair_value_boolean_array(pair: *const nvpair_t, vals: *mut *mut boolean_t, nr_elem: *mut uint_t) -> c_int;
}

#[cfg(test)]
mod test {
    use std::ptr;
    use super::{data_type_t, boolean_t, nvlist_t, nvpair_t, NV_UNIQUE_NAME};
    use super::{nvlist_alloc, nvlist_free};
    use super::{nvlist_empty, nvlist_remove, nvlist_exists, nvlist_next_nvpair, nvpair_type, nvpair_name};
    use super::{nvlist_add_boolean_value, nvlist_lookup_boolean_value, nvlist_add_boolean_array, nvpair_value_boolean_array};
    use std::ffi::{CStr, CString};

    #[test]
    fn alloc_free() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn test_nvlist_empty() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        assert_eq!(unsafe { nvlist_empty(ptr) }, boolean_t::B_TRUE);

        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn test_nvlist_empty_iter() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        assert_eq!(unsafe { nvlist_empty(ptr) }, boolean_t::B_TRUE);

        let mut pair: *mut nvpair_t = ptr::null_mut();
        assert_eq!(unsafe { nvlist_next_nvpair(ptr, pair) }, ptr::null_mut());

        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn test_add_bool() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        let name1 = CString::new("bool1").unwrap();
        assert_eq!(unsafe { nvlist_add_boolean_value(ptr, name1.as_ptr(), boolean_t::B_TRUE) }, 0);
        assert_eq!(unsafe { nvlist_empty(ptr) }, boolean_t::B_FALSE);
        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn test_lookup_bool() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, NV_UNIQUE_NAME, 0) }, 0);
        let name1 = CString::new("bool1").unwrap();
        assert_eq!(unsafe { nvlist_add_boolean_value(ptr, name1.as_ptr(), boolean_t::B_TRUE) }, 0);
        let mut target_bool: boolean_t = boolean_t::B_FALSE;
        assert_eq!(unsafe { nvlist_lookup_boolean_value(ptr, name1.as_ptr(), &mut target_bool) }, 0);
        assert_eq!(target_bool, boolean_t::B_TRUE);

        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn test_add_and_remove_bool() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        let name1 = CString::new("bool1").unwrap();
        assert_eq!(unsafe { nvlist_empty(ptr) }, boolean_t::B_TRUE);
        assert_eq!(unsafe { nvlist_add_boolean_value(ptr, name1.as_ptr(), boolean_t::B_TRUE) }, 0);
        assert_eq!(unsafe { nvlist_empty(ptr) }, boolean_t::B_FALSE);
        assert_eq!(unsafe { nvlist_remove(ptr, name1.as_ptr(), data_type_t::DATA_TYPE_BOOLEAN_VALUE) }, 0);
        assert_eq!(unsafe { nvlist_empty(ptr) }, boolean_t::B_TRUE);
        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn test_1_bool_iter() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        let name1 = CString::new("bool1").unwrap();
        assert_eq!(unsafe { nvlist_add_boolean_value(ptr, name1.as_ptr(), boolean_t::B_TRUE) }, 0);
        let mut pair: *mut nvpair_t = ptr::null_mut();
        let next_pair = unsafe { nvlist_next_nvpair(ptr, pair) };
        assert!(next_pair != ptr::null_mut());
        assert_eq!(unsafe { nvpair_type(next_pair) }, data_type_t::DATA_TYPE_BOOLEAN_VALUE);
        let nvpair_name = unsafe { nvpair_name(next_pair) };
        assert!(nvpair_name != ptr::null_mut());
        let nvpair_name_cstr: &CStr = unsafe { CStr::from_ptr(nvpair_name) };
        assert_eq!(nvpair_name_cstr.to_str().unwrap(), "bool1");

        assert_eq!(unsafe { nvlist_next_nvpair(ptr, next_pair) }, ptr::null_mut());

        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn from_ffi_bools_array() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, NV_UNIQUE_NAME, 0) }, 0);
        let name = CString::new("bool_array").unwrap();
        let mut array : [boolean_t; 4] = [boolean_t::B_TRUE, boolean_t::B_FALSE, boolean_t::B_FALSE, boolean_t::B_TRUE];
        assert_eq!(unsafe { nvlist_add_boolean_array(ptr, name.as_ptr(), array.as_mut_ptr(), 4) }, 0);
        let mut pair: *mut nvpair_t = ptr::null_mut();
        let next_pair = unsafe { nvlist_next_nvpair(ptr, pair) };

        let mut n_elem: super::uint_t =  0;
        let mut output_ptr: *mut boolean_t =  unsafe { ::std::mem::uninitialized() };
        assert_eq!(unsafe { nvpair_value_boolean_array(next_pair, &mut output_ptr, &mut n_elem) }, 0);
        let output_slice = unsafe { ::std::slice::from_raw_parts(output_ptr, n_elem as usize) };
        assert!(output_slice.as_ptr() != array.as_ptr());
        assert_eq!(n_elem, 4);
        assert_eq!(output_slice, &array);
        unsafe { nvlist_free(ptr); }
    }
}
