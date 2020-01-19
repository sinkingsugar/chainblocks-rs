#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

#[macro_use]

extern crate ctor;    

mod chainblocksc;

use crate::chainblocksc::CBVar;
use crate::chainblocksc::CBTypesInfo;
use crate::chainblocksc::CBContext;
use crate::chainblocksc::CBlock;
use crate::chainblocksc::CBCore;
use crate::chainblocksc::CBSeq;
use crate::chainblocksc::chainblocksInterface;
use std::ffi::CString;
use std::convert::TryInto;

const ABI_VERSION: u32 = 0x20200101;

extern crate dlopen;
use dlopen::symbor::Library;

fn try_load_dlls() -> Option<Library> {
    let macLib = Library::open("libcb.dylib").ok();
    if macLib.is_some() {
        return macLib;
    }
    return None;
}

#[ctor]
static Core: CBCore = {
    let exe = Library::open_self()
        .ok()
        .unwrap();

    let exefun = exe.symbol::<unsafe extern "C" fn(abi_version: u32)->CBCore>("chainblocksInterface").ok();
    if exefun.is_some() {
        let fun = exefun.unwrap();
        let core = fun(ABI_VERSION);
        if core.registerBlock.is_none() {
            panic!("Failed to aquire chainblocks interface, version not compatible.");
        }
        core
    } else {
        let lib = try_load_dlls().unwrap();
        let fun = lib.symbol::<unsafe extern "C" fn(abi_version: u32)->CBCore>("chainblocksInterface").unwrap();
        let core = fun(ABI_VERSION);
        if core.registerBlock.is_none() {
            panic!("Failed to aquire chainblocks interface, version not compatible.");
        }
        core
    }
};

#[inline(always)]
fn length(seq: CBSeq) -> u64 {
    unsafe {
        let arr = seq as *mut std::ffi::c_void;
        return Core
            .arrayLength
            .unwrap()
            (arr);
    }
}

#[inline(always)]
fn free(seq: CBSeq) {
    unsafe {
        let arr = seq as *mut std::ffi::c_void;
        Core
            .arrayFree
            .unwrap()
            (arr);
    }
}

struct Seq {
    cseq: CBSeq
}

impl Seq {
    fn new() -> Seq {
        println!("creating new Seq");
        return Seq{
            cseq: std::ptr::null_mut() as CBSeq
        };
    }

    fn length(&self) -> u64 {
        return length(self.cseq);
    }
}

impl Drop for Seq {
    fn drop(&mut self) {
        free(self.cseq);
        println!("Seq dropped!");
    }
}

struct SeqIterator {
    seq: Seq,
    count: u64
}

impl Iterator for SeqIterator {
    type Item = CBVar;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < length(self.seq.cseq) {
            unsafe {
                let item = *self.seq.cseq.offset(self.count.try_into().unwrap());
                self.count += 1;
                Some(item)
            }
        } else {
            None
        }
    }
}

impl IntoIterator for Seq {
    type Item = CBVar;
    type IntoIter = SeqIterator;

    fn into_iter(self) -> Self::IntoIter {
        return SeqIterator{
            seq: self,
            count: 0
        };
    }
}

trait Var {
}

trait Block {
    fn name(&self) -> String;
    fn help(&self) -> String { "".to_string() }
    
    fn setup(&self) {}
    fn destroy(&self) {}

    // fn inputTypes(&self) -> CBTypesInfo;
    // fn outputTypes(&self) -> CBTypesInfo;

    fn setParam(&self, _index: i32, _value: &CBVar) {}
    fn getParam(&self, _index: i32) {}

    fn activate(&self, context: &CBContext, input: &CBVar) -> CBVar;
    fn cleanup(&self) {}
}

struct BlockWrapper<T: Block> {
    header: CBlock,
    block: T,
    name: Option<CString>,
    help: Option<CString>
}

unsafe extern "C" fn cblock_construct<T: Default + Block>() -> *mut CBlock {
    println!("construct");
    let wrapper: Box<BlockWrapper::<T>> = Box::new(create());
    let wptr = Box::into_raw(wrapper);
    return wptr as *mut CBlock;
}

unsafe extern "C" fn cblock_name<T: Block>(arg1: *mut CBlock) -> *const ::std::os::raw::c_char {
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

unsafe extern "C" fn cblock_help<T: Block>(arg1: *mut CBlock) -> *const ::std::os::raw::c_char {
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
    let ctx = arg2.as_ref();
    let value = arg3.as_ref();
    return (*blk).block.activate(ctx.unwrap(), value.unwrap());
}

unsafe extern "C" fn cblock_cleanup<T: Block>(arg1: *mut CBlock) {
    let blk = arg1 as *mut BlockWrapper<T>;
    (*blk).block.cleanup();
}

fn create<T: Default + Block>() -> BlockWrapper<T> {
    return BlockWrapper::<T>{
        header: CBlock{
            name: Some(cblock_name::<T>),
            help: Some(cblock_help::<T>),
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

#[cfg(test)]
mod dummy_block {
    // run with: RUST_BACKTRACE=1 cargo test -- --nocapture
    
    use super::Block;
    use super::BlockWrapper;
    use super::create;
    use super::Seq;
    use crate::chainblocksc::CBVar;
    use crate::chainblocksc::CBTypesInfo;
    use crate::chainblocksc::CBContext;
    use crate::chainblocksc::CBlock;
    use std::ffi::CString;

    #[derive(Default)]
    struct DummyBlock;
    type WDummyBlock = BlockWrapper<DummyBlock>;

    impl Block for DummyBlock {
        fn name(&self) -> String { "Dummy".to_string() }
        fn activate(&self, _context: &CBContext, _input: &CBVar) -> CBVar { return CBVar::default(); }  
    }

    #[test]
    fn instanciate() {
        let blk = create::<DummyBlock>();
        assert_eq!("Dummy".to_string(), blk.block.name());

        let s = Seq::new();
        assert_eq!(s.length(), 0);

        let blkname = CString::new("Dummy").expect("CString failed...");
        unsafe {
            super::Core.registerBlock
                .unwrap()(
                    blkname.as_ptr(),
                    Some(super::cblock_construct::<DummyBlock>));

            let cblk = super::Core.createBlock
                .unwrap()
                (blkname.as_ptr());
            (*cblk).setup.unwrap()(cblk);
            (*cblk).destroy.unwrap()(cblk);
        }
    }
}
