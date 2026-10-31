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
// package org.allbinary.logic.visual.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigNoManipulationFactory::TransformInfoObjectConfigNoManipulationFactory;
use super::TransformInfoHttpStore::TransformInfoHttpStore;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoHttpStoreNoManipulation {
    pub base: TransformInfoHttpStore,
}


impl TransformInfoHttpStoreNoManipulation {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TransformInfoHttpStore::new_struct2(propertiesHashMap.clone(), pageContext.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    // throws Exception
    pub fn setObjectConfigFile(&mut self, mut value: String) {
        self.setObjectConfigFileName(value.clone());
        { let java_arguments = (jcall!(TransformInfoObjectConfigNoManipulationFactory::getInstance()).getInstance2(None /* Java this: a &mut self method has no Rc of itself */, self.getObjectConfigFilePath()),); self.setObjectConfigInterface(java_arguments.0) };
    }
    

    // throws Exception
    pub fn setObjectConfig(&mut self, mut value: String) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Should not set from config"))));
    }
}
impl std::ops::Deref for TransformInfoHttpStoreNoManipulation {
    type Target = TransformInfoHttpStore;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoHttpStoreNoManipulation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn for TransformInfoHttpStoreNoManipulation {
    fn as_TransformInfo(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &self.base.base.base }
    fn as_TransformInfo_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &mut self.base.base.base }
    fn setObjectConfigFile(&mut self, mut value: String) { Self::setObjectConfigFile(self, value) }
    fn setObjectConfig(&mut self, mut value: String) { Self::setObjectConfig(self, value) }
    fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStore::getTemplateFilePath(crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStoreDyn::as_TransformInfoHttpStore_mut(self)) }
    fn getObjectConfigFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStore::getObjectConfigFilePath(crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStoreDyn::as_TransformInfoHttpStore_mut(self)) }
    fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStore::getDataFilePath(crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStoreDyn::as_TransformInfoHttpStore_mut(self)) }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn for TransformInfoHttpStoreNoManipulation {
    fn as_TransformInfoHttp(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttp { &self.base.base }
    fn as_TransformInfoHttp_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttp { &mut self.base.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStoreDyn for TransformInfoHttpStoreNoManipulation {
    fn as_TransformInfoHttpStore(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStore { &self.base }
    fn as_TransformInfoHttpStore_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStore::TransformInfoHttpStore { &mut self.base }
}


