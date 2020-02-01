#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

#[macro_use]
extern crate ctor;

pub mod block;
#[cfg(feature = "blocks")]
pub mod blocks;
mod chainblocksc;
pub mod core;
pub mod types;

use crate::block::Block;
use crate::chainblocksc::CBContext;
use crate::chainblocksc::CBSeq;
use crate::chainblocksc::CBTypeInfo;
use crate::chainblocksc::CBTypesInfo;
use crate::chainblocksc::CBVar;
use crate::chainblocksc::CBlock;
use crate::core::Core;
use crate::types::Types;
use crate::types::Var;
use std::convert::TryInto;
use std::ffi::CString;

// use this to develop/debug:
// cargo +nightly rustc --profile=check -- -Zunstable-options --pretty=expanded

macro_rules! var {
    ((--> $($param:tt) *)) => { Var::from(blocks!($($param) *)) };
    ($vexp:expr) => { Var::from($vexp) }
}

macro_rules! blocks {
    ($(($block:ident $($param:tt) *)) *) => {
        {
            let mut x = Vec::<$crate::chainblocksc::CBlockRef>::new();
            $(
                {
                    let blk = $crate::core::createBlock(stringify!($block));
                    unsafe {
                        (*blk).setup.unwrap()(blk);
                    }
                    let mut pidx: i32 = 0;
                    $(
                        {
                            let pvar = var!($param);
                            (*blk).setParam.unwrap()(blk, pidx, pvar);
                            pidx += 1;
                        }
                    ) *
                        x.push(blk);
                }
            ) *
            x
        }
    };
}

// --features "dummy"
#[cfg(any(test, feature = "dummy"))]
mod dummy_block {
    // run with: RUST_BACKTRACE=1 cargo test -- --nocapture

    use super::block::create;
    use super::Types;
    use crate::block::cblock_construct;
    use crate::block::Block;
    use crate::block::BlockWrapper;
    use crate::chainblocksc::CBContext;
    use crate::chainblocksc::CBTypeInfo;
    use crate::chainblocksc::CBType_Int;
    use crate::chainblocksc::CBTypesInfo;
    use crate::chainblocksc::CBVar;
    use crate::chainblocksc::CBlock;
    use crate::core::cloneVar;
    use crate::core::createBlock;
    use crate::core::init;
    use crate::core::log;
    use crate::core::sleep;
    use crate::core::Core;
    use crate::types::common_type;
    use crate::types::Var;
    use std::ffi::CString;

    struct DummyBlock {
        inputTypes: Types,
        outputTypes: Types,
    }

    impl Default for DummyBlock {
        fn default() -> Self {
            DummyBlock {
                inputTypes: Types::from(vec![common_type::none]),
                outputTypes: Types::from(vec![common_type::any]),
            }
        }
    }

    type WDummyBlock = BlockWrapper<DummyBlock>;

    impl Block for DummyBlock {
        fn name(&mut self) -> &str {
            "Dummy"
        }
        fn inputTypes(&mut self) -> &Types {
            &self.inputTypes
        }
        fn outputTypes(&mut self) -> &Types {
            &self.outputTypes
        }
        fn activate(&mut self, _context: &CBContext, _input: &Var) -> Var {
            log("Dummy - activate: Ok!");
            let mut x: String = "Before...".to_string();
            log(&x);
            sleep(2.0);
            x.push_str(" - and After!");
            log(&x);
            log("Dummy - activate: Resumed!");
            return Var::default();
        }
    }

    #[ctor]
    fn registerDummy() {
        init();
        let blkname = CString::new("Dummy").expect("CString failed...");
        unsafe {
            Core.registerBlock.unwrap()(blkname.as_ptr(), Some(cblock_construct::<DummyBlock>));
        }
    }

    fn macroTest() {
        // let blks =
        //     blocks!((Const 10)
        //             (Log)
        //             (Repeat
        //              (-->
        //               (Msg "repeating..."))));
    }

    #[test]
    fn instanciate() {
        let mut blk = create::<DummyBlock>();
        assert_eq!("Dummy".to_string(), blk.block.name());

        let blkname = CString::new("Dummy").expect("CString failed...");
        unsafe {
            let cblk = Core.createBlock.unwrap()(blkname.as_ptr());
            (*cblk).setup.unwrap()(cblk);
            (*cblk).destroy.unwrap()(cblk);
        }

        macroTest();

        let a = Var::from(10);
        let mut b = Var::from(());
        cloneVar(&mut b, &a);
        unsafe {
            assert_eq!(a.valueType, CBType_Int);
            assert_eq!(b.valueType, CBType_Int);
            assert_eq!(a.payload.__bindgen_anon_1.intValue, 10);
            assert_eq!(
                a.payload.__bindgen_anon_1.intValue,
                b.payload.__bindgen_anon_1.intValue
            );
        }

        log("Hello chainblocks-rs");
    }
}
