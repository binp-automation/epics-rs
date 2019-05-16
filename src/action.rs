use core::ops::{Deref, DerefMut};

use crate::record::*;

pub enum Action {
    Init     { record: AnyRecord },
    GetIoint { record: Record },
    Read     { record: ReadRecord },
    Write    { record: WriteRecord },
    Linconv  { record: LinconvRecord, after: u64 },
}

pub enum AsyncAction {
    Read     { record: ReadRecord },
    Write    { record: WriteRecord },
}
impl Deref for AsyncAction {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        match self {
            AsyncAction::Read { ref record } => record,
            AsyncAction::Write { ref record } => record,
        }
    }
}
impl DerefMut for AsyncAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            AsyncAction::Read { ref mut record } => record,
            AsyncAction::Write { ref mut record } => record,
        }
    }
}