extern crate nvpair_sys;

pub use nvpair_sys as ffi;

use std::borrow::Cow;
use std::ptr;
use ffi::*;
use std::ffi::{CStr, CString};

pub type Key<'k> = Cow<'k,str>;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnkownDataType(String),
    IllegalName(std::str::Utf8Error)
}

#[derive(Debug, PartialEq)]
pub enum PairValue {
    Boolean,
    BooleanValue(bool),
    BooleanValueArray(Vec<bool>),
}

#[derive(Debug, PartialEq)]
pub struct Pair<'k>(Key<'k>,PairValue);

#[derive(Debug, PartialEq)]
pub enum Uniqueness {
    Unknown,
    None,
    Name,
    Pair
}

fn nvpair_name_to_string(pair: *const ffi::nvpair_t) -> Result<String, Error> {
    let name_str = unsafe { CStr::from_ptr(nvpair_name(pair)) }.to_str();
    match name_str {
        Ok(n) => Ok(n.to_owned()),
        Err(e) => Err(Error::IllegalName(e))
    }
}

impl<'a> Pair<'a> {
    pub fn from_ffi(pair: *const ffi::nvpair_t) -> Result<Pair<'a>, Error> {
        let data_type = unsafe { nvpair_type(pair) };
        let name = try!(nvpair_name_to_string(pair));
        match data_type {
            data_type_t::DATA_TYPE_BOOLEAN_VALUE => {
                let mut target_bool: boolean_t = boolean_t::B_FALSE;
                assert_eq!(unsafe { nvpair_value_boolean_value(pair, &mut target_bool) }, 0);
                let bool_value = match target_bool {
                    boolean_t::B_TRUE  => true,
                    boolean_t::B_FALSE => false,
                };
                Ok(Pair(name.into(), PairValue::BooleanValue(bool_value)))
            },
            data_type_t::DATA_TYPE_BOOLEAN_ARRAY => {
                let mut n_elem: ffi::uint_t =  0;
                let mut output_ptr: *mut boolean_t =  unsafe { ::std::mem::uninitialized() };
                assert_eq!(unsafe { nvpair_value_boolean_array(pair, &mut output_ptr, &mut n_elem) }, 0);
                let output_slice = unsafe { ::std::slice::from_raw_parts(output_ptr, n_elem as usize) };

                let mut output_vec = Vec::with_capacity(output_slice.len());

                for element in output_slice {
                    let bool_value = match element {
                        &boolean_t::B_TRUE  => true,
                        &boolean_t::B_FALSE => false,
                    };
                    output_vec.push(bool_value);
                }


                Ok(Pair(name.into(), PairValue::BooleanValueArray(output_vec)))
            },
            _ => { return Err(Error::UnkownDataType(format!("{:?}", data_type))) }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct List<'l> {
    uniqueness: Uniqueness,
    pairs: Vec<Pair<'l>>
}

impl<'l> List<'l> {
    pub fn new(u: Uniqueness) -> List<'l> {
        List {
            uniqueness: u,
            pairs: Vec::new()
        }
    }

    pub fn from_ffi(nvlist: *mut ffi::nvlist_t) -> Result<List<'l>, Error> {
        let mut pairs = Vec::new();

        let mut prev: *mut nvpair_t = ptr::null_mut();
        loop {
            let pair = unsafe { nvlist_next_nvpair(nvlist, prev) };
            if pair == ptr::null_mut() {
                break;
            } else {
                pairs.push(try!(Pair::from_ffi(pair)));
            }
            prev = pair;
        }

        Ok(List {
            uniqueness: Uniqueness::Unknown,
            pairs: pairs
        })
    }

    pub fn add(&mut self, pair: Pair<'l>) -> &mut List<'l>{
        self.pairs.push(pair);
        self
    }
}

#[cfg(test)]
mod test {
    use std::ptr;
    use super::ffi::*;
    use std::ffi::{CStr, CString};
    use super::{List,Pair,PairValue,Uniqueness};

    #[test]
    fn from_ffi_one_bool() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        let name1 = CString::new("bool1").unwrap();
        assert_eq!(unsafe { nvlist_add_boolean_value(ptr, name1.as_ptr(), boolean_t::B_TRUE) }, 0);
        let nvlist = List::from_ffi(ptr).unwrap();
        assert_eq!(nvlist.pairs.len(), 1);
        let mut l2 = List::new(Uniqueness::Unknown);
        l2.add(Pair("bool1".into(), PairValue::BooleanValue(true)));
        assert_eq!(nvlist, l2);
        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn from_ffi_two_bools() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        let name1 = CString::new("bool1").unwrap();
        let name2 = CString::new("bool2").unwrap();
        assert_eq!(unsafe { nvlist_add_boolean_value(ptr, name1.as_ptr(), boolean_t::B_TRUE) }, 0);
        assert_eq!(unsafe { nvlist_add_boolean_value(ptr, name2.as_ptr(), boolean_t::B_FALSE) }, 0);
        let nvlist = List::from_ffi(ptr).unwrap();
        assert_eq!(nvlist.pairs.len(), 2);
        let mut l2 = List::new(Uniqueness::Unknown);
        l2.add(Pair("bool1".into(), PairValue::BooleanValue(true)));
        l2.add(Pair("bool2".into(), PairValue::BooleanValue(false)));
        assert_eq!(nvlist, l2);
        unsafe { nvlist_free(ptr); }
    }

    #[test]
    fn from_ffi_bool_array() {
        let mut ptr : *mut nvlist_t  = ptr::null_mut();
        assert_eq!(unsafe { nvlist_alloc(&mut ptr, 0, 0) }, 0);
        let name = CString::new("bool_array").unwrap();
        let mut array : [boolean_t; 4] = [boolean_t::B_TRUE, boolean_t::B_FALSE, boolean_t::B_FALSE, boolean_t::B_TRUE];
        assert_eq!(unsafe { nvlist_add_boolean_array(ptr, name.as_ptr(), array.as_mut_ptr(), 4) }, 0);
        let mut pair: *mut nvpair_t = ptr::null_mut();
        let next_pair = unsafe { nvlist_next_nvpair(ptr, pair) };

        let mut n_elem: uint_t =  0;
        let mut output_ptr: *mut boolean_t =  unsafe { ::std::mem::uninitialized() };
        assert_eq!(unsafe { nvpair_value_boolean_array(next_pair, &mut output_ptr, &mut n_elem) }, 0);
        let output_slice = unsafe { ::std::slice::from_raw_parts(output_ptr, n_elem as usize) };

        let nvlist = List::from_ffi(ptr).unwrap();
        assert_eq!(nvlist.pairs.len(), 1);
        let mut l2 = List::new(Uniqueness::Unknown);
        l2.add(Pair("bool_array".into(), PairValue::BooleanValueArray(vec![true, false, false, true])));
        assert_eq!(nvlist, l2);
        unsafe { nvlist_free(ptr); }
    }
}
