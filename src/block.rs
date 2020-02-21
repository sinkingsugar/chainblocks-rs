use crate::chainblocksc::CBContext;
use crate::chainblocksc::CBExposedTypesInfo;
use crate::chainblocksc::CBInstanceData;
use crate::chainblocksc::CBParametersInfo;
use crate::chainblocksc::CBSeq;
use crate::chainblocksc::CBTypeInfo;
use crate::chainblocksc::CBTypesInfo;
use crate::chainblocksc::CBVar;
use crate::chainblocksc::CBlock;
use crate::core::Core;
use crate::types::Context;
use crate::types::ExposedTypes;
use crate::types::InstanceData;
use crate::types::Parameters;
use crate::types::Type;
use crate::types::Types;
use crate::types::Var;
use std::ffi::CString;

pub trait Block {
    fn name(&mut self) -> &str;
    fn help(&mut self) -> &str {
        ""
    }

    fn setup(&mut self) {}
    fn destroy(&mut self) {}

    fn inputTypes(&mut self) -> &Types;
    fn outputTypes(&mut self) -> &Types;

    fn exposedVariables(&mut self) -> Option<&ExposedTypes> {
        None
    }
    fn requiredVariables(&mut self) -> Option<&ExposedTypes> {
        None
    }

    fn canCompose() -> bool {
        false
    }
    fn compose(&mut self, _data: &InstanceData) -> Type {
        Type::default()
    }

    fn parameters(&mut self) -> Option<&Parameters> {
        None
    }
    fn setParam(&mut self, _index: i32, _value: &Var) {}
    fn getParam(&mut self, _index: i32) -> Var {
        Var::default()
    }

    fn warmup(&mut self, _context: &Context) {}
    fn activate(&mut self, context: &Context, input: &Var) -> Var;
    fn cleanup(&mut self) {}
}

#[repr(C)]
pub struct BlockWrapper<T: Block> {
    header: CBlock,
    pub block: T,
    name: Option<CString>,
    help: Option<CString>,
}

pub unsafe extern "C" fn cblock_construct<T: Default + Block>() -> *mut CBlock {
    let wrapper: Box<BlockWrapper<T>> = Box::new(create());
    let wptr = Box::into_raw(wrapper);
    return wptr as *mut CBlock;
}

unsafe extern "C" fn cblock_name<T: Block>(arg1: *mut CBlock) -> *const ::std::os::raw::c_char {
    let blk = arg1 as *mut BlockWrapper<T>;
    let name = (*blk).block.name();
    (*blk).name = Some(CString::new(name).expect("CString::new failed"));
    return (*blk).name.as_ref().unwrap().as_ptr();
}

unsafe extern "C" fn cblock_help<T: Block>(arg1: *mut CBlock) -> *const ::std::os::raw::c_char {
    let blk = arg1 as *mut BlockWrapper<T>;
    let help = (*blk).block.help();
    (*blk).help = Some(CString::new(help).expect("CString::new failed"));
    return (*blk).help.as_ref().unwrap().as_ptr();
}

unsafe extern "C" fn cblock_inputTypes<T: Block>(arg1: *mut CBlock) -> CBTypesInfo {
    let blk = arg1 as *mut BlockWrapper<T>;
    let t = (*blk).block.inputTypes();
    CBTypesInfo::from(t)
}

unsafe extern "C" fn cblock_outputTypes<T: Block>(arg1: *mut CBlock) -> CBTypesInfo {
    let blk = arg1 as *mut BlockWrapper<T>;
    let t = (*blk).block.outputTypes();
    CBTypesInfo::from(t)
}

unsafe extern "C" fn cblock_setup<T: Block>(arg1: *mut CBlock) {
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.setup();
}

unsafe extern "C" fn cblock_destroy<T: Block>(arg1: *mut CBlock) {
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.destroy();
    Box::from_raw(blk);
    drop(blk);
}

unsafe extern "C" fn cblock_warmup<T: Block>(
    arg1: *mut CBlock,
    arg2: *mut CBContext
) {
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.warmup(&(*arg2));
}

unsafe extern "C" fn cblock_activate<T: Block>(
    arg1: *mut CBlock,
    arg2: *mut CBContext,
    arg3: *const CBVar,
) -> CBVar {
    let blk = arg1 as *mut BlockWrapper<T>;
    return (*blk).block.activate(&(*arg2), &(*arg3));
}

unsafe extern "C" fn cblock_cleanup<T: Block>(arg1: *mut CBlock) {
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.cleanup();
}

unsafe extern "C" fn cblock_exposedVariables<T: Block>(arg1: *mut CBlock) -> CBExposedTypesInfo {
    let blk = arg1 as *mut BlockWrapper<T>;
    if let Some(exposed) = (*blk).block.exposedVariables() {
        CBExposedTypesInfo::from(exposed)
    } else {
        CBExposedTypesInfo::default()
    }
}

unsafe extern "C" fn cblock_requiredVariables<T: Block>(arg1: *mut CBlock) -> CBExposedTypesInfo {
    let blk = arg1 as *mut BlockWrapper<T>;
    if let Some(required) = (*blk).block.requiredVariables() {
        CBExposedTypesInfo::from(required)
    } else {
        CBExposedTypesInfo::default()
    }
}

unsafe extern "C" fn cblock_compose<T: Block>(
    arg1: *mut CBlock,
    data: CBInstanceData,
) -> CBTypeInfo {
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.compose(&data)
}

unsafe extern "C" fn cblock_parameters<T: Block>(arg1: *mut CBlock) -> CBParametersInfo {
    let blk = arg1 as *mut BlockWrapper<T>;
    if let Some(params) = (*blk).block.parameters() {
        CBParametersInfo::from(params)
    } else {
        CBParametersInfo::default()
    }
}

unsafe extern "C" fn cblock_getParam<T: Block>(
    arg1: *mut CBlock,
    arg2: ::std::os::raw::c_int,
) -> CBVar {
    let blk = arg1 as *mut BlockWrapper<T>;
    return (*blk).block.getParam(arg2);
}

unsafe extern "C" fn cblock_setParam<T: Block>(
    arg1: *mut CBlock,
    arg2: ::std::os::raw::c_int,
    arg3: CBVar,
) {
    let blk = arg1 as *mut BlockWrapper<T>;
    return (*blk).block.setParam(arg2, &arg3);
}

pub fn create<T: Default + Block>() -> BlockWrapper<T> {
    return BlockWrapper::<T> {
        header: CBlock {
            inlineBlockId: 0,
            name: Some(cblock_name::<T>),
            help: Some(cblock_help::<T>),
            inputTypes: Some(cblock_inputTypes::<T>),
            outputTypes: Some(cblock_outputTypes::<T>),
            setup: Some(cblock_setup::<T>),
            destroy: Some(cblock_destroy::<T>),
            exposedVariables: Some(cblock_exposedVariables::<T>),
            requiredVariables: Some(cblock_requiredVariables::<T>),
            compose: if T::canCompose() {
                Some(cblock_compose::<T>)
            } else {
                None
            },
            parameters: Some(cblock_parameters::<T>),
            setParam: Some(cblock_setParam::<T>),
            getParam: Some(cblock_getParam::<T>),
            warmup: Some(cblock_warmup::<T>),
            activate: Some(cblock_activate::<T>),
            cleanup: Some(cblock_cleanup::<T>),
        },
        block: T::default(),
        name: None,
        help: None,
    };
}
