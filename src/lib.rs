#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]

extern crate ctor;    

mod chainblocksc;

use crate::chainblocksc::CBVar;
use crate::chainblocksc::CBTypesInfo;
use crate::chainblocksc::CBContext;
use crate::chainblocksc::CBlock;
use std::ffi::CString;

trait Var {
}

trait Block {
    fn name(&self) -> String;
    fn help(&self) -> String { "".to_string() }
    
    fn setup(&self) {}
    fn destroy(&self) {}

    // fn inputTypes(&self) -> CBTypesInfo;
    // fn outputTypes(&self) -> CBTypesInfo;

    fn setParam(&self, index: i32, value: CBVar) {}
    fn getParam(&self, index: i32) {}

    fn activate(&self, context: &CBContext, input: &CBVar) -> CBVar;
    fn cleanup(&self) {}
}

struct BlockWrapper<T: Default + Block> {
    header: CBlock,
    block: T,
    name: Option<CString>,
    help: Option<CString>
}

unsafe extern "C" fn cblock_name<T: Default + Block>(arg1: *mut CBlock) -> *const ::std::os::raw::c_char {
    let blk = arg1 as *mut BlockWrapper<T>;
    if !(*blk).name.is_some() {
        let name = (*blk).block.name();
        (*blk).name = Some(CString::new(name)
                           .expect("CString::new failed"));
    }
    return (*blk).name
            .as_ref()
            .unwrap()
            .as_ptr();
}

unsafe extern "C" fn cblock_help<T: Default + Block>(arg1: *mut CBlock) -> *const ::std::os::raw::c_char {
    let blk = arg1 as *mut BlockWrapper<T>;
    if !(*blk).help.is_some() {
        let help = (*blk).block.help();
        (*blk).help = Some(CString::new(help)
                           .expect("CString::new failed"));
    }
    return (*blk).help
            .as_ref()
            .unwrap()
            .as_ptr();
}

fn create<T: Default + Block>() -> BlockWrapper<T> {
    return BlockWrapper::<T>{
        header: CBlock{
            name: Some(cblock_name::<T>),
            help: Some(cblock_help::<T>),
            ..Default::default()
        },
        block: T::default(),
        name: None,
        help: None
    };
}

#[cfg(dllblock)]
mod dllblock {
    use crate::chainblocksc::CBVar;
    use crate::chainblocksc::CBCore;
    use crate::chainblocksc::chainblocksInterface;
    
    extern crate dlopen;
    use dlopen::symbor::Library;

    static mut Core: CBCore = CBCore{
        registerBlock: None,
        registerObjectType: None,
        registerEnumType: None,
        registerRunLoopCallback: None,
        unregisterRunLoopCallback: None,
        registerExitCallback: None,
        unregisterExitCallback: None,
        findVariable: None,
        throwException: None,
        suspend: None,
        cloneVar: None,
        destroyVar: None,
        freeArray: None,
        validateChain: None,
        runChain: None,
        validateBlocks: None,
        runBlocks: None,
        log: None,
        createBlock: None,
        createChain: None,
        destroyChain: None,
        createNode: None,
        destroyNode: None,
        schedule: None,
        tick: None,
        sleep: None
    };

    fn try_load_dlls() -> Option<Library> {
        return None;
    }

    #[ctor]
    fn attach() {
        let lib = Library::open_self()
            .ok()
            .or_else(try_load_dlls)
            .unwrap();
        
        let fun = unsafe{
            lib.symbol::<unsafe extern "C" fn()->CBCore>("chainblocksInterface")
        }.unwrap();

        unsafe {
            Core = fun();
        }
    }
}

#[cfg(test)]
mod dummy_block {
    use super::Block;
    use super::BlockWrapper;
    use super::create;
    use crate::chainblocksc::CBVar;
    use crate::chainblocksc::CBTypesInfo;
    use crate::chainblocksc::CBContext;
    use crate::chainblocksc::CBlock;

    #[derive(Default)]
    struct DummyBlock;
    type WDummyBlock = BlockWrapper<DummyBlock>;

    impl Block for DummyBlock {
        fn name(&self) -> String { "Dummy".to_string() }
        fn activate(&self, context: &CBContext, input: &CBVar) -> CBVar {
            return CBVar{ ..Default::default() };
        }
    }

    #[test]
    fn instanciate() {
        let blk = create::<DummyBlock>();
        assert_eq!("Dummy".to_string(), blk.block.name());
    }
}
