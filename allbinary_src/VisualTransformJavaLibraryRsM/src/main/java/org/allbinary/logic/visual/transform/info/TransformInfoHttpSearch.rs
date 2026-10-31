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
use super::*;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::TransformInfoHttp::TransformInfoHttp;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoHttpSearch {
    pub base: TransformInfoHttp,
}


impl TransformInfoHttpSearch {
    

    pub fn new_struct(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Self {
        let mut this = Self {
            base: TransformInfoHttp::new_struct2(jcall!(searchRequest).getPropertiesHashMap(), jcall!(searchRequest).getPageContext()),
        };
        // super(...) is in the struct literal above
        if true {
            { let java_arguments = (jcall!(jcall!(searchRequest).getStoreFront()).getName(),); this.setStoreName(java_arguments.0) };
        } else {
            { let java_arguments = (jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); this.setStoreName(java_arguments.0) };
        }
        let mut dataFileName: String = jcall!(searchRequest).getXslFile();
        this.setTemplateFile(dataFileName.clone());
        this
    }

    pub fn new(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(searchRequest))))
    }
    

    // throws Exception
    pub fn getPath(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
        jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone());
        jcall!(stringBuffer).append(self.getStoreName());
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        return jcall!(stringBuffer).toString();
    }
    

    // throws Exception
    pub fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new(self.getPath(), self.getTemplateFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    // throws Exception
    pub fn getObjectConfigFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new(self.getPath(), self.getObjectConfigFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    // throws Exception
    pub fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new(self.getPath(), self.getDataFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
}
impl std::ops::Deref for TransformInfoHttpSearch {
    type Target = TransformInfoHttp;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoHttpSearch {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn for TransformInfoHttpSearch {
    fn as_TransformInfo(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &self.base.base }
    fn as_TransformInfo_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &mut self.base.base }
    fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getTemplateFilePath(self) }
    fn getObjectConfigFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getObjectConfigFilePath(self) }
    fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getDataFilePath(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn for TransformInfoHttpSearch {
    fn as_TransformInfoHttp(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttp { &self.base }
    fn as_TransformInfoHttp_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttp { &mut self.base }
}


