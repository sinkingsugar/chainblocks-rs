#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

#[macro_use]

extern crate ctor;

mod chainblocksc;
mod core;
mod block;
mod types;

use std::ffi::CString;
use std::convert::TryInto;
use crate::core::Core;
use crate::block::Block;
use crate::types::Types;
use crate::chainblocksc::CBVar;
use crate::chainblocksc::CBTypeInfo;
use crate::chainblocksc::CBTypesInfo;
use crate::chainblocksc::CBContext;
use crate::chainblocksc::CBlock;
use crate::chainblocksc::CBSeq;

#[inline(always)]
fn length<T>(a: *mut T) -> u64 {
    unsafe {
        let arr = a as *mut std::ffi::c_void;
        return Core
            .arrayLength
            .unwrap()
            (arr);
    }
}

#[inline(always)]
fn free<T>(a: *mut T) {
    unsafe {
        let arr = a as *mut std::ffi::c_void;
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

pub trait IntoVar {
    fn into_var(self) -> CBVar;
}

#[cfg(any(test, feature = "dummy"))]
mod dummy_block {
    // run with: RUST_BACKTRACE=1 cargo test -- --nocapture

    use crate::core::Core;
    use crate::block::cblock_construct;
    use crate::block::Block;
    use crate::block::BlockWrapper;
    use super::block::create;
    use super::Seq;
    use super::Types;
    use crate::chainblocksc::CBVar;
    use crate::chainblocksc::CBTypeInfo;
    use crate::chainblocksc::CBTypesInfo;
    use crate::chainblocksc::CBContext;
    use crate::chainblocksc::CBlock;
    use std::ffi::CString;

    struct DummyBlock {
        inputTypes: Types,
        outputTypes: Types
    }

    impl Default for DummyBlock {
        fn default() -> Self {
            DummyBlock{
                inputTypes: Types::from(vec![CBTypeInfo::default()]),
                outputTypes: Types::from(vec![CBTypeInfo::default()])
            }
        }
    }
    
    type WDummyBlock = BlockWrapper<DummyBlock>;

    impl Block for DummyBlock {
        fn name(&self) -> &str { "Dummy" }
        fn inputTypes(&self) -> &Types { &self.inputTypes  }
        fn outputTypes(&self) -> &Types { &self.outputTypes }
        fn activate(&self, _context: &CBContext, _input: &CBVar) -> CBVar { return CBVar::default(); }  
    }

    #[ctor]
    fn registerDummy() {
        let blkname = CString::new("Dummy").expect("CString failed...");
        unsafe {
            Core.registerBlock
                .unwrap()(
                    blkname.as_ptr(),
                    Some(cblock_construct::<DummyBlock>));
        }
    }

    #[test]
    fn instanciate() {
        let blk = create::<DummyBlock>();
        assert_eq!("Dummy".to_string(), blk.block.name());

        let s = Seq::new();
        assert_eq!(s.length(), 0);

        let blkname = CString::new("Dummy").expect("CString failed...");        
        unsafe {
            let cblk = Core.createBlock
                .unwrap()
                (blkname.as_ptr());
            (*cblk).setup.unwrap()(cblk);
            (*cblk).destroy.unwrap()(cblk);
        }
    }
}
