use crate::chainblocksc::CBCore;
use crate::chainblocksc::chainblocksInterface;

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
pub static Core: CBCore = {
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
