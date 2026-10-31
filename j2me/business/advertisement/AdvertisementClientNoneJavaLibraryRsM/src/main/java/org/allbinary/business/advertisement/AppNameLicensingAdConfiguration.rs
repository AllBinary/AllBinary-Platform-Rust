// Generated Code Do Not Modify
// package org.allbinary.business.advertisement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::AdConfiguration::AdConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AppNameLicensingAdConfiguration {
    pub base: AdConfiguration,
}


impl AppNameLicensingAdConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AdConfiguration::new_struct(vec![jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()]),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for AppNameLicensingAdConfiguration {
    type Target = AdConfiguration;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AppNameLicensingAdConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn for AppNameLicensingAdConfiguration {
    fn as_AdConfiguration(&self) -> &crate::org::allbinary::business::advertisement::AdConfiguration::AdConfiguration { &self.base }
    fn as_AdConfiguration_mut(&mut self) -> &mut crate::org::allbinary::business::advertisement::AdConfiguration::AdConfiguration { &mut self.base }
}


