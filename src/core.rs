use crate::block::cblock_construct;
use crate::block::Block;
use crate::chainblocksc::chainblocksInterface;
use crate::chainblocksc::CBContext;
use crate::chainblocksc::CBCore;
use crate::chainblocksc::CBString;
use crate::chainblocksc::CBVar;
use crate::chainblocksc::CBlockPtr;
use crate::types::Var;
use std::ffi::CStr;
use std::ffi::CString;

const ABI_VERSION: u32 = 0x20200101;

extern crate dlopen;
use dlopen::symbor::Library;

fn try_load_dlls() -> Option<Library> {
    if let Ok(lib) = Library::open("libcb.dylib") {
        Some(lib)
    } else if let Ok(lib) = Library::open("libcb_shared.dylib") {
        Some(lib)
    } else if let Ok(lib) = Library::open("libcb.so") {
        Some(lib)
    } else if let Ok(lib) = Library::open("libcb_shared.so") {
        Some(lib)
    } else {
        None
    }
}

pub static mut Core: CBCore = CBCore {
    registerBlock: None,
    registerObjectType: None,
    registerEnumType: None,
    registerRunLoopCallback: None,
    unregisterRunLoopCallback: None,
    registerExitCallback: None,
    unregisterExitCallback: None,
    referenceVariable: None,
    releaseVariable: None,
    getStack: None,
    throwException: None,
    suspend: None,
    cloneVar: None,
    destroyVar: None,
    seqFree: None,
    seqPush: None,
    seqInsert: None,
    seqPop: None,
    seqResize: None,
    seqFastDelete: None,
    seqSlowDelete: None,
    typesFree: None,
    typesPush: None,
    typesInsert: None,
    typesPop: None,
    typesResize: None,
    typesFastDelete: None,
    typesSlowDelete: None,
    paramsFree: None,
    paramsPush: None,
    paramsInsert: None,
    paramsPop: None,
    paramsResize: None,
    paramsFastDelete: None,
    paramsSlowDelete: None,
    blocksFree: None,
    blocksPush: None,
    blocksInsert: None,
    blocksPop: None,
    blocksResize: None,
    blocksFastDelete: None,
    blocksSlowDelete: None,
    expTypesFree: None,
    expTypesPush: None,
    expTypesInsert: None,
    expTypesPop: None,
    expTypesResize: None,
    expTypesFastDelete: None,
    expTypesSlowDelete: None,
    stringsFree: None,
    stringsPush: None,
    stringsInsert: None,
    stringsPop: None,
    stringsResize: None,
    stringsFastDelete: None,
    stringsSlowDelete: None,
    tableNew: None,
    validateChain: None,
    runChain: None,
    validateBlocks: None,
    runBlocks: None,
    getChainInfo: None,
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

static mut init_done: bool = false;

unsafe fn initInternal() {
    let exe = Library::open_self().ok().unwrap();

    let exefun = exe
        .symbol::<unsafe extern "C" fn(abi_version: u32) -> CBCore>("chainblocksInterface")
        .ok();
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
        let fun = lib
            .symbol::<unsafe extern "C" fn(abi_version: u32) -> CBCore>("chainblocksInterface")
            .unwrap();
        let core = fun(ABI_VERSION);
        if core.registerBlock.is_none() {
            panic!("Failed to aquire chainblocks interface, version not compatible.");
        }
        Core = core;
        log("chainblocks-rs attached!");

        init_done = true;
    }
}

#[inline(always)]
pub fn init() {
    unsafe {
        if !init_done {
            initInternal();
        }
    }
}

#[inline(always)]
pub fn log(s: &str) {
    let clog = CString::new(s).unwrap();
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
    let blkname = CString::new(name).unwrap();
    unsafe {
        Core.registerBlock.unwrap()(blkname.as_ptr(), Some(cblock_construct::<T>));
    }
}

#[inline(always)]
pub fn getRootPath() -> &'static str {
    unsafe {
        CStr::from_ptr(Core.getRootPath.unwrap()())
            .to_str()
            .unwrap()
    }
}

#[inline(always)]
pub fn createBlock(name: &str) -> CBlockPtr {
    let cname = CString::new(name).unwrap();
    unsafe { Core.createBlock.unwrap()(cname.as_ptr()) }
}

#[inline(always)]
pub fn cloneVar(dst: &mut Var, src: &Var) {
    unsafe {
        Core.cloneVar.unwrap()(dst, src);
    }
}

pub fn referenceMutVariable(context: &CBContext, name: CBString) -> &mut CBVar {
    unsafe {
        let ctx = context as *const CBContext as *mut CBContext;
        let cbptr = Core.referenceVariable.unwrap()(ctx, name);
        cbptr.as_mut().unwrap()
    }
}

pub fn referenceVariable(context: &CBContext, name: CBString) -> &CBVar {
    unsafe {
        let ctx = context as *const CBContext as *mut CBContext;
        let cbptr = Core.referenceVariable.unwrap()(ctx, name);
        cbptr.as_mut().unwrap()
    }
}

pub fn releaseMutVariable(var: &mut CBVar) {
     unsafe {
        let v = var as *mut CBVar;
        Core.releaseVariable.unwrap()(v);
    }
}

pub fn releaseVariable(var: &CBVar) {
    unsafe {
        let v = var as *const CBVar as *mut CBVar;
        Core.releaseVariable.unwrap()(v);
    }
}
