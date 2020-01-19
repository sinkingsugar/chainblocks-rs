use std::ffi::CString;
use crate::core::Core;
use crate::types::Types;
use crate::chainblocksc::CBVar;
use crate::chainblocksc::CBTypeInfo;
use crate::chainblocksc::CBTypesInfo;
use crate::chainblocksc::CBContext;
use crate::chainblocksc::CBlock;
use crate::chainblocksc::CBSeq;

pub trait Block {
    fn name(&self) -> &str;
    fn help(&self) -> &str { "" }
    
    fn setup(&self) {}
    fn destroy(&self) {}

    fn inputTypes(&self) -> &Types;
    fn outputTypes(&self) -> &Types;

    fn setParam(&self, _index: i32, _value: &CBVar) {}
    fn getParam(&self, _index: i32) {}

    fn activate(&self, context: &CBContext, input: &CBVar) -> CBVar;
    fn cleanup(&self) {}
}

pub struct BlockWrapper<T: Block> {
    header: CBlock,
    pub block: T,
    name: Option<CString>,
    help: Option<CString>
}

pub unsafe extern "C" fn cblock_construct<T: Default + Block>() -> *mut CBlock {
    println!("construct");
    let wrapper: Box<BlockWrapper::<T>> = Box::new(create());
    let wptr = Box::into_raw(wrapper);
    return wptr as *mut CBlock;
}

unsafe extern "C" fn cblock_name<T: Block>(arg1: *mut CBlock) -> *const ::std::os::raw::c_char {
    let blk = arg1 as *mut BlockWrapper<T>;
    let name = (*blk).block.name();
    (*blk).name = Some(CString::new(name)
                       .expect("CString::new failed"));
    return (*blk).name
        .as_ref()
        .unwrap()
        .as_ptr();
}

unsafe extern "C" fn cblock_help<T: Block>(arg1: *mut CBlock) -> *const ::std::os::raw::c_char {
    let blk = arg1 as *mut BlockWrapper<T>;
    let help = (*blk).block.help();
    (*blk).help = Some(CString::new(help)
                       .expect("CString::new failed"));
    return (*blk).help
        .as_ref()
        .unwrap()
        .as_ptr();
}

unsafe extern "C" fn cblock_inputTypes<T: Block>(arg1: *mut CBlock) -> CBTypesInfo {
    let blk = arg1 as *mut BlockWrapper<T>;
    let t = (*blk).block.inputTypes();
    return t.ctypes;
}

unsafe extern "C" fn cblock_outputTypes<T: Block>(arg1: *mut CBlock) -> CBTypesInfo {
    let blk = arg1 as *mut BlockWrapper<T>;
    let t = (*blk).block.outputTypes();
    return t.ctypes;
}

unsafe extern "C" fn cblock_setup<T: Block>(arg1: *mut CBlock) {
    println!("setup");
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.setup();
}

unsafe extern "C" fn cblock_destroy<T: Block>(arg1: *mut CBlock) {
    println!("destroy");
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.destroy();
    Box::from_raw(blk);
}

unsafe extern "C" fn cblock_activate<T: Block>(arg1: *mut CBlock,
                                                         arg2: *mut CBContext,
                                                         arg3: *const CBVar) -> CBVar {
    let blk = arg1 as *mut BlockWrapper<T>;
    return (*blk).block.activate(&(*arg2), &(*arg3));
}

unsafe extern "C" fn cblock_cleanup<T: Block>(arg1: *mut CBlock) {
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.cleanup();
}

pub fn create<T: Default + Block>() -> BlockWrapper<T> {
    return BlockWrapper::<T>{
        header: CBlock{
            name: Some(cblock_name::<T>),
            help: Some(cblock_help::<T>),
            inputTypes: Some(cblock_inputTypes::<T>),
            outputTypes: Some(cblock_outputTypes::<T>),
            setup: Some(cblock_setup::<T>),
            destroy: Some(cblock_destroy::<T>),
            activate: Some(cblock_activate::<T>),
            cleanup: Some(cblock_cleanup::<T>),
            ..Default::default()
        },
        block: T::default(),
        name: None,
        help: None
    };
}
