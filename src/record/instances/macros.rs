
#[macro_use]
macro_rules! derive_deref {
    ($Struct:ident, $Target:path, $field:ident) => {
        impl std::ops::Deref for $Struct {
            type Target = $Target;
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }
        impl std::ops::DerefMut for $Struct {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}
#[macro_use]
macro_rules! derive_deref_record {
    ($Struct:ident) => {
        impl std::ops::Deref for $Struct {
            type Target = crate::record::Record;
            fn deref(&self) -> &Self::Target {
                self
            }
        }
        impl std::ops::DerefMut for $Struct {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self
            }
        }
    };
}

#[macro_use]
macro_rules! impl_record_private {
    ($Record:ident, $Private:ident) => {
        impl $Record {
            unsafe fn private(&self) -> &$Private {
                (self.raw.dpvt as *const $Private).as_ref().unwrap()
            }
            unsafe fn private_mut(&mut self) -> &mut $Private {
                (self.raw.dpvt as *mut $Private).as_mut().unwrap()
            }
        }
    }
}

#[macro_use]
macro_rules! impl_record_handler {
    ($Record:ident, $Handler:ident) => {
        impl $Record {
            pub unsafe fn replace_handler(&mut self, h: Box<dyn $Handler + Send>) -> Option<Box<dyn $Handler + Send>> {
                self.private_mut().handler.replace(h)
            }
            pub unsafe fn take_handler(&mut self) -> Option<Box<dyn $Handler + Send>> {
                self.private_mut().handler.take()
            }
            pub unsafe fn with_handler<F, R>(&mut self, f: F) -> Option<R>
            where F: FnOnce(&mut dyn $Handler, &mut Self) -> R {
                match self.take_handler() {
                    Some(mut h) => {
                        let ret = f(h.as_mut(), self);
                        assert!(self.replace_handler(h).is_none());
                        Some(ret)
                    },
                    None => None
                }
            }
        }
    }
}

#[macro_use]
macro_rules! derive_record {
    ($Record:ident, $Private:ident) => {
        impl crate::record::Record for $Record {
            unsafe fn as_raw(&self) -> &crate::epics_sys::dbCommon {
                (self.raw as *const _ as *const crate::epics_sys::dbCommon).as_ref().unwrap()
            }
            unsafe fn as_raw_mut(&mut self) -> &mut crate::epics_sys::dbCommon {
                (self.raw as *mut _ as *mut crate::epics_sys::dbCommon).as_mut().unwrap()
            }

            unsafe fn init(&mut self) {
                let cpvt = crate::record::raw_private_create(self.as_raw_mut(), RecordType::Ai);
                let pvt = $Private::new(cpvt);
                crate::record::raw_private_init::<$Private>(self.as_raw_mut(), pvt);
            }

            unsafe fn private(&self) -> &Private {
                self.private()
            }
            unsafe fn private_mut(&mut self) -> &mut Private {
                self.private_mut()
            }
        }
    }
}
#[macro_use]
macro_rules! derive_scan_record {
    ($Record:ident) => {
        impl crate::record::ScanRecord for $Record {
            unsafe fn handler_set_scan(&mut self, scan: Scan) -> Option<crate::Result<()>> {
                self.with_handler(|h, r| h.set_scan(r, scan))
            }
        }
    }
}
#[macro_use]
macro_rules! derive_read_record {
    ($Record:ident) => {
        impl crate::record::ReadRecord for $Record {
            unsafe fn handler_read(&mut self) -> Option<crate::Result<bool>> {
                self.with_handler(|h, r| h.read(r))
            }
            unsafe fn handler_read_async(&mut self) -> Option<crate::Result<()>> {
                self.with_handler(|h, r| h.read_async(r))
            }
        }
    }
}
#[macro_use]
macro_rules! derive_write_record {
    ($Record:ident) => {
        impl crate::record::WriteRecord for $Record {
            unsafe fn handler_write(&mut self) -> Option<crate::Result<bool>> {
                self.with_handler(|h, r| h.write(r))
            }
            unsafe fn handler_write_async(&mut self) -> Option<crate::Result<()>> {
                self.with_handler(|h, r| h.write_async(r))
            }
        }
    }
}
