//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// 
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use super::TagHelperFactory::TagHelperFactory;
use super::WorkFlowHelper::WorkFlowHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WorkFlowRequestHelperFactory {
    pub base: TagHelperFactory,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl crate::admin::taghelpers::TagHelperFactoryInterface::TagHelperFactoryInterface for WorkFlowRequestHelperFactory {
    fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getInstance(self, hashMap, pageContext) }
    fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getInstance(self, hashMap, pageContext) }
}

impl WorkFlowRequestHelperFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TagHelperFactory::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException
    pub fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return WorkFlowHelper::new(hashMap.clone(), pageContext.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}
impl std::ops::Deref for WorkFlowRequestHelperFactory {
    type Target = TagHelperFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WorkFlowRequestHelperFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelperFactory::TagHelperFactoryDyn for WorkFlowRequestHelperFactory {
    fn as_TagHelperFactory(&self) -> &crate::admin::taghelpers::TagHelperFactory::TagHelperFactory { &self.base }
    fn as_TagHelperFactory_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelperFactory::TagHelperFactory { &mut self.base }
    fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getInstance(self, hashMap, pageContext) }
}


