use crate::chainblocksc::CBCore;
use crate::chainblocksc::chainblocksInterface;
use crate::block::Block;
use crate::block::cblock_construct;
use std::ffi::CString;

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

pub static mut Core: CBCore = CBCore {
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
    arrayLength: None,
    arrayFree: None,
    seqPush: None,
    seqInsert: None,
    seqPop: None,
    seqResize: None,
    seqFastDelete: None,
    seqSlowDelete: None,
    typesPush: None,
    typesInsert: None,
    typesPop: None,
    typesResize: None,
    typesFastDelete: None,
    typesSlowDelete: None,
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
    sleep: None,
    getRootPath: None,
    setRootPath: None,
};

unsafe fn initInternal() {
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
        Core = core;
        log("chainblocks-rs attached!");
    } else {
        let lib = try_load_dlls().unwrap();
        let fun = lib.symbol::<unsafe extern "C" fn(abi_version: u32)->CBCore>("chainblocksInterface").unwrap();
        let core = fun(ABI_VERSION);
        if core.registerBlock.is_none() {
            panic!("Failed to aquire chainblocks interface, version not compatible.");
        }
        Core = core;
        log("chainblocks-rs attached!");   
    }
}

#[inline(always)]
pub fn init() {
    unsafe {
        initInternal();
    }
}

#[inline(always)]
pub fn log(s: &str) {
    let clog = CString::new(s).expect("CString failed.");
    unsafe {
        Core.log.unwrap()(clog.as_ptr());
    }
}

#[inline(always)]
pub fn sleep(seconds: f64) {
    unsafe {
        Core.sleep.unwrap()(seconds, true);
    }
}

#[inline(always)]
pub fn registerBlock<T: Default + Block>(name: &str) {
    let blkname = CString::new(name)
        .expect("CString failed...");
    unsafe {
        Core.registerBlock
            .unwrap()(
                blkname.as_ptr(),
                Some(cblock_construct::<T>));
    }
}
