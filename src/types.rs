use crate::chainblocksc::CBTypeInfo;
use crate::chainblocksc::CBTypesInfo;
use crate::chainblocksc::CBExposedTypeInfo;
use crate::chainblocksc::CBExposedTypesInfo;
use crate::chainblocksc::CBParameterInfo;
use crate::length;
use crate::free;
use crate::core::Core;
use std::ffi::CString;

unsafe impl std::marker::Sync for CBTypeInfo {
}

// pub struct Types {
//     pub ctypes: CBTypesInfo
// }

pub struct BaseArray<T> {
    pub carr: *mut T
}

// impl Types {
//     fn new() -> Self {
//         return Types{
//             ctypes: std::ptr::null_mut() as CBTypesInfo
//         };
//     }

//     fn length(&self) -> u64 {
//         return length(self.ctypes);
//     }
// }

// impl Drop for Types {
//     fn drop(&mut self) {
//         free(self.ctypes);
//     }
// }

impl<T> BaseArray<T> {
    fn new() -> Self {
        return BaseArray::<T>{
            carr: std::ptr::null_mut() as *mut T
        };
    }

    fn length(&self) -> u64 {
        return length(self.carr);
    }
}

impl<T> Drop for BaseArray<T> {
    fn drop(&mut self) {
        free(self.carr);
    }
}

pub type Types = BaseArray<CBTypeInfo>;

impl From<Vec<CBTypeInfo>> for Types {
    fn from(v: Vec<CBTypeInfo>) -> Types {
        let mut res: Types = Types::new();
        for t in &v {
            unsafe {
                res.carr = Core.typesPush
                    .unwrap()
                    (res.carr, t);
            }
        }
        return res;
    }
}

pub struct ExposedInfo {
    ctype: CBTypeInfo,
    name: CString,
    help: CString,
    isMutable: bool
}

impl ExposedInfo {
    fn new(name: &str,
           ctype: CBTypeInfo) -> Self {
        ExposedInfo{
            ctype: ctype,
            name: CString::new(name)
                .expect("CString failed."),
            help: CString::new("")
                .expect("CString failed."),
            isMutable: false,
        }
    }

    pub fn native(&self) -> CBExposedTypeInfo {
        CBExposedTypeInfo{
            name: self.name.as_ptr(),
            help: self.help.as_ptr(),
            exposedType: self.ctype,
            isMutable: self.isMutable
        }
    }
}

pub type ExposedTypes = BaseArray<CBExposedTypeInfo>;

// impl From<Vec<ExposedInfo>> for ExposedTypes {
//     fn from(v: Vec<ExposedInfo>) -> Self {
//         let mut res: Types = Types::new();
//         for t in &v {
//             unsafe {
//                 res.carr = Core.typesPush
//                     .unwrap()
//                     (res.carr, t);
//             }
//         }
//         return res;
//     }
// }

pub type Parameters = BaseArray<CBParameterInfo>;

pub mod common_type {
    use crate::chainblocksc::CBTypeInfo;
    use crate::chainblocksc::CBType_None;
    use crate::chainblocksc::CBType_Any;
    use crate::chainblocksc::CBTypeInfo__bindgen_ty_1;

    pub fn none() -> CBTypeInfo {
        CBTypeInfo{
            basicType: CBType_None,
            ..Default::default()
        }
    }

    pub fn any() -> CBTypeInfo {
        CBTypeInfo{
            basicType: CBType_Any,
            ..Default::default()
        }
    }
}
