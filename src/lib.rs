#[allow(unused_imports)]
#[macro_use]

extern crate ctor;    

mod chainblocksc;

// #[cfg(dllblock)]
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
mod tests {
    use crate::chainblocksc::CBVar;
    use crate::chainblocksc::CBType_Int;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let v = CBVar{ ..Default::default() };
    }
}
