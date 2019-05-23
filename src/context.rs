use std::marker::PhantomData;

use crate::record::{RecordBase};

pub struct InitContext {
    phantom: PhantomData<()>,
}
impl InitContext {
    pub unsafe fn new() -> Self {
        Self { phantom: PhantomData }
    }
}


pub struct RecRdContext {
    phantom: PhantomData<()>,
}
impl RecRdContext {
    pub unsafe fn new() -> Self {
        Self { phantom: PhantomData }
    }
    pub fn request_async(&mut self, rec: &mut RecordBase) {
        unsafe {
            assert_eq!(rec.pact(), false);
            rec.set_pact(true);
        }
    }
}

pub struct RecWrContext {
    phantom: PhantomData<()>,
}
impl RecWrContext {
    pub unsafe fn new() -> Self {
        Self { phantom: PhantomData }
    }
    pub fn request_async(&mut self, rec: &mut RecordBase) {
        unsafe {
            assert_eq!(rec.pact(), false);
            rec.set_pact(true);
        }
    }
}


pub struct EmptyContext {
    phantom: PhantomData<()>,
}
impl EmptyContext {
    pub unsafe fn new() -> Self {
        Self { phantom: PhantomData }
    }
}

pub type QuitContext = EmptyContext;
pub type RecInitContext = EmptyContext;
pub type RecScanContext = EmptyContext;
pub type RecRdAContext = EmptyContext;
pub type RecWrAContext = EmptyContext;