use std::marker::PhantomData;


pub struct Context {
    phantom: PhantomData<()>,
}
impl Context {
    pub unsafe fn new() -> Self {
        Self { phantom: PhantomData }
    }
}
