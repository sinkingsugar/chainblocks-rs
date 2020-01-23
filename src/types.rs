use crate::chainblocksc::CBTypeInfo;
use crate::chainblocksc::CBTypesInfo;
use crate::chainblocksc::CBExposedTypeInfo;
use crate::chainblocksc::CBExposedTypesInfo;
use crate::chainblocksc::CBParameterInfo;
use crate::chainblocksc::CBParametersInfo;
use crate::chainblocksc::CBVar;
use crate::chainblocksc::CBInstanceData;
use crate::chainblocksc::CBString;
use crate::chainblocksc::CBSeq;
use crate::chainblocksc::CBVarPayload;
use crate::chainblocksc::CBVarPayload__bindgen_ty_1;
use crate::chainblocksc::CBVarPayload__bindgen_ty_1__bindgen_ty_2;
use crate::chainblocksc::CBContext;
use crate::chainblocksc::CBType_Int;
use crate::chainblocksc::CBType_Float;
use crate::chainblocksc::CBType_String;
use crate::chainblocksc::CBType_Seq;
use crate::chainblocksc::CBType_Bool;
use crate::length;
use crate::free;
use crate::core::Core;
use std::ffi::CString;
use std::ffi::CStr;
use std::convert::TryFrom;
use std::convert::TryInto;

pub type Context = CBContext;
pub type Var = CBVar;
pub type Type = CBTypeInfo;
pub type String = CBString;
pub type InstanceData = CBInstanceData;

unsafe impl std::marker::Sync for CBTypeInfo {
}

pub struct BaseArray<T> {
    pub carr: *mut T
}

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

pub struct ParameterInfo {
    name: CString,
    help: CString,
    types: Types,
}

impl ParameterInfo {
    fn new(name: &str,
           types: Types) -> Self {
        ParameterInfo{
            name: CString::new(name)
                .expect("CString failed."),
            help: CString::new("")
                .expect("CString failed."),
            types: types
        }
    }

    fn new1(name: &str,
           help: &str,
           types: Types) -> Self {
        ParameterInfo{
            name: CString::new(name)
                .expect("CString failed."),
            help: CString::new(help)
                .expect("CString failed."),
            types: types
        }
    }

    pub fn native(&self) -> CBParameterInfo {
        CBParameterInfo{
            name: self.name.as_ptr(),
            help: self.help.as_ptr(),
            valueTypes: self.types.carr
        }
    }
}

impl From<(&str, Types)> for ParameterInfo {
    fn from(v: (&str, Types)) -> ParameterInfo {
        ParameterInfo::new(v.0, v.1)
    }
}

impl From<(&str, &str, Types)> for ParameterInfo {
    fn from(v: (&str, &str, Types)) -> ParameterInfo {
        ParameterInfo::new1(v.0, v.1, v.2)
    }
}

pub struct Parameters {
    params: Vec<ParameterInfo>,
    pub cparams: BaseArray<CBParameterInfo>,
}

impl From<Vec<ParameterInfo>> for Parameters {
    fn from(v: Vec<ParameterInfo>) -> Parameters {
        let mut cparams = BaseArray::<CBParameterInfo>::new();
        for t in &v {
            unsafe {
                cparams.carr = Core.paramsPush
                    .unwrap()
                    (cparams.carr, &t.native());
            }
        }
        Parameters{
            params: v,
            cparams: cparams
        }
    }
}

pub mod common_type {
    use crate::chainblocksc::CBTypeInfo;
    use crate::chainblocksc::CBType_None;
    use crate::chainblocksc::CBType_Any;
    use crate::chainblocksc::CBType_String;
    use crate::chainblocksc::CBType_Int;
    use crate::chainblocksc::CBType_Float;
    use crate::chainblocksc::CBType_Seq;
    use crate::chainblocksc::CBType_Block;
    use crate::chainblocksc::CBType_Bool;
    use crate::chainblocksc::CBType_Chain;
    use crate::chainblocksc::CBTypeInfo__bindgen_ty_1;

    const fn base_info() -> CBTypeInfo {
        CBTypeInfo{
            basicType: CBType_None,
            __bindgen_anon_1: CBTypeInfo__bindgen_ty_1{
                seqType: std::ptr::null_mut()
            }
        }
    }

    const fn make_none() -> CBTypeInfo {
        let mut res = base_info();
        res.basicType = CBType_None;
        res
    }

    pub static none: CBTypeInfo = make_none();

    const fn make_any() -> CBTypeInfo {
        let mut res = base_info();
        res.basicType = CBType_Any;
        res
    }

    pub static any: CBTypeInfo = make_any();

    macro_rules! cbtype {
        ($fname:ident, $type:expr, $name:ident, $names:ident) => {    
            const fn $fname() -> CBTypeInfo {
                let mut res = base_info();
                res.basicType = $type;
                res
            }

            pub static $name: CBTypeInfo = $fname();

            pub static $names: CBTypeInfo = CBTypeInfo{
                basicType: CBType_Seq,
                __bindgen_anon_1: CBTypeInfo__bindgen_ty_1{
                    seqType: (&$name) as *const CBTypeInfo as *mut CBTypeInfo
                }
            };
        }
    }

    cbtype!(make_string, CBType_String, string, strings);
    cbtype!(make_int, CBType_Int, int, ints);
    cbtype!(make_float, CBType_Float, float, floats);
    cbtype!(make_bool, CBType_Bool, bool, bools);
    cbtype!(make_block, CBType_Block, block, blocks);
    cbtype!(make_chain, CBType_Chain, chain, chains);
}

#[repr(transparent)] // force it same size of original
pub struct OwnedVar(pub Var);

impl Drop for OwnedVar {
    #[inline(always)]
    fn drop(&mut self) {
        match self.0.valueType {
            CBType_Seq => unsafe { free(self.0.payload.__bindgen_anon_1.seqValue); }
            CBType_String => unsafe {
                let p = self.0.payload.__bindgen_anon_1.__bindgen_anon_2.stringValue as *mut i8;
                let s = CString::from_raw(p);
                drop(s);
            }
            _ => {}
        }       
    }
}

impl From<()> for Var {
    #[inline(always)]
    fn from(_: ()) -> Self {
        CBVar::default()
    }
}

impl From<()> for OwnedVar {
    #[inline(always)]
    fn from(_: ()) -> Self {
       OwnedVar(Var::from(()))
    }
}

macro_rules! var_from {
    ($type:ident, $varfield:ident, $cbtype:expr) => {
        impl From<$type> for Var {
            #[inline(always)]
            fn from(v: $type) -> Self {
                CBVar{
                    valueType: $cbtype,
                    payload: CBVarPayload{
                        __bindgen_anon_1: CBVarPayload__bindgen_ty_1{
                            $varfield: v
                        }
                    },
                    ..Default::default()
                }
            }
        }

        impl From<$type> for OwnedVar {
            #[inline(always)]
            fn from(v: $type) -> Self {
                OwnedVar(Var::from(v))
            }
        }
    }
}

var_from!(bool, boolValue, CBType_Bool);
var_from!(i64, intValue, CBType_Int);
var_from!(f64, floatValue, CBType_Float);

impl From<CBString> for Var {
    #[inline(always)]
    fn from(v: CBString) -> Self {
        CBVar{
            valueType: CBType_String,
            payload: CBVarPayload{
                __bindgen_anon_1: CBVarPayload__bindgen_ty_1{
                    __bindgen_anon_2: CBVarPayload__bindgen_ty_1__bindgen_ty_2{
                        stringValue: v,
                        stackPosition: 0
                    }
                }
            },
            ..Default::default()
        }
    }
}

impl From<&CStr> for OwnedVar {
    #[inline(always)]
    fn from(v: &CStr) -> Self {
        let s = v.to_str().unwrap();
        let cstring = CString::new(s).unwrap();
        let res = CBVar{
            valueType: CBType_String,
            payload: CBVarPayload{
                __bindgen_anon_1: CBVarPayload__bindgen_ty_1{
                    __bindgen_anon_2: CBVarPayload__bindgen_ty_1__bindgen_ty_2{
                        stringValue: cstring.into_raw(),
                        stackPosition: 0
                    }
                }
            },
            ..Default::default()
        };
        OwnedVar(res)
    }
}


impl From<&CString> for Var {
    #[inline(always)]
    fn from(v: &CString) -> Self {
        CBVar{
            valueType: CBType_String,
            payload: CBVarPayload{
                __bindgen_anon_1: CBVarPayload__bindgen_ty_1{
                    __bindgen_anon_2: CBVarPayload__bindgen_ty_1__bindgen_ty_2{
                        stringValue: v.as_ptr(),
                        stackPosition: 0
                    }
                }
            },
            ..Default::default()
        }
    }
}

impl From<Option<&CString>> for Var {
    #[inline(always)]
    fn from(v: Option<&CString>) -> Self {
        if v.is_none() {
            Var::default()
        } else {
            Var::from(v.unwrap())
        }
    }
}

impl From<Vec<Var>> for OwnedVar {
    #[inline(always)]
    fn from(vec: Vec<Var>) -> Self {
        unsafe {
            let mut cbseq: CBSeq = std::ptr::null_mut();
            for v in vec {
                cbseq = Core.seqPush
                    .unwrap()
                    (cbseq, &v);
            }
            let res = CBVar{
                valueType: CBType_Seq,
                payload: CBVarPayload{
                    __bindgen_anon_1: CBVarPayload__bindgen_ty_1{
                        seqValue: cbseq
                    }
                },
                ..Default::default()
            };
            OwnedVar(res)
        }
    }
}

impl TryFrom<&Var> for std::string::String {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(var: &Var) -> Result<Self, Self::Error> {
        if var.valueType != CBType_String {
            Err("Expected String variable, but casting failed.")
        } else {
            unsafe {
                let cstr = CStr::from_ptr(var.payload.__bindgen_anon_1.__bindgen_anon_2.stringValue);
                Ok(std::string::String::from(cstr.to_str().unwrap()))
            }
        }
    }
}

impl TryFrom<&Var> for CString {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(var: &Var) -> Result<Self, Self::Error> {
        if var.valueType != CBType_String {
            Err("Expected String variable, but casting failed.")
        } else {
            unsafe {
                let cstr = CStr::from_ptr(var.payload.__bindgen_anon_1.__bindgen_anon_2.stringValue);
                Ok(CString::from(cstr))
            }
        }
    }
}

impl TryFrom<&Var> for i64 {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(var: &Var) -> Result<Self, Self::Error> {
        if var.valueType != CBType_Int {
            Err("Expected Int variable, but casting failed.")
        } else {
            unsafe {
                Ok(var.payload.__bindgen_anon_1.intValue)
            }
        }
    }
}

impl TryFrom<&Var> for f64 {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(var: &Var) -> Result<Self, Self::Error> {
        if var.valueType != CBType_Float {
            Err("Expected Float variable, but casting failed.")
        } else {
            unsafe {
                Ok(var.payload.__bindgen_anon_1.floatValue)
            }
        }
    }
}

impl TryFrom<&Var> for bool {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(var: &Var) -> Result<Self, Self::Error> {
        if var.valueType != CBType_Bool {
            Err("Expected Float variable, but casting failed.")
        } else {
            unsafe {
                Ok(var.payload.__bindgen_anon_1.boolValue)
            }
        }
    }
}

impl TryFrom<&Var> for Vec<Var> {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(var: &Var) -> Result<Self, Self::Error> {
        if var.valueType != CBType_Seq {
            Err("Expected Float variable, but casting failed.")
        } else {
            unsafe {
                let mut res = Vec::<Var>::new();
                let len = length(var.payload.__bindgen_anon_1.seqValue);
                for i in 0..len {
                    let var = var.payload.__bindgen_anon_1.seqValue.offset(i.try_into().unwrap());
                    res.push(*var);
                }
                Ok(res)
            }
        }
    }
}
