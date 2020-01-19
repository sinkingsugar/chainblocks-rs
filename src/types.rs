use crate::chainblocksc::CBTypeInfo;
use crate::chainblocksc::CBTypesInfo;
use crate::length;
use crate::free;
use crate::core::Core;

pub trait IntoType {
    fn into_type(self) -> CBTypeInfo;
}

pub trait IntoTypes {
    fn into_types(self) -> CBTypesInfo;
}

pub struct Types {
    pub ctypes: CBTypesInfo
}

impl Types {
    fn new() -> Types {
        return Types{
            ctypes: std::ptr::null_mut() as CBTypesInfo
        };
    }

    fn length(&self) -> u64 {
        return length(self.ctypes);
    }
}

impl Drop for Types {
    fn drop(&mut self) {
        free(self.ctypes);
    }
}

impl From<Vec<CBTypeInfo>> for Types {
    fn from(v: Vec<CBTypeInfo>) -> Types {
        let mut res: Types = Types::new();
        for t in &v {
            unsafe {
                res.ctypes = Core.typesPush
                    .unwrap()
                    (res.ctypes, t);
            }
        }
        return res;
    }
}
