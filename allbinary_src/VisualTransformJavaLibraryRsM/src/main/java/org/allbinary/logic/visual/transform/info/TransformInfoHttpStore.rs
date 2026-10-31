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
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::TransformInfoHttp::TransformInfoHttp;
use super::TransformInfoHttpStoreInterface::TransformInfoHttpStoreInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoHttpStore {
    pub base: TransformInfoHttp,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

// impl TransformInfoHttpStoreInterface for TransformInfoHttpStore with the methods of the trait

impl TransformInfoHttpStore {
    

    pub fn new_struct2(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TransformInfoHttp::new_struct3(propertiesHashMap.clone(), pageContext.clone(), true),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties HashMap: "), jcall!(propertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new2(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(propertiesHashMap, pageContext))))
    }
    

    pub fn new_struct3(mut databaseHashMap: Option<Rc<RefCell<HashMap>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TransformInfoHttp::new_struct3(databaseHashMap.clone(), propertiesHashMap.clone(), pageContext.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Database HashMap: "));
            jcall!(stringBuffer).append(jcall!(databaseHashMap).toString());
            jcall!(stringBuffer).append(String::from("\nProperties HashMap: "));
            jcall!(stringBuffer).append(jcall!(propertiesHashMap).toString());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(HashMap, HashMap , PageContext)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        { let java_arguments = ((jcall!(databaseHashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */,); this.setStoreName(java_arguments.0) };
        this
    }

    pub fn new3(mut databaseHashMap: Option<Rc<RefCell<HashMap>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(databaseHashMap, propertiesHashMap, pageContext))))
    }
    

    pub fn new_struct3(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TransformInfoHttp::new_struct2(propertiesHashMap.clone(), pageContext.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties HashMap: "), jcall!(propertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        { let java_arguments = (jcall!(storeFrontInterface).getName(),); this.setStoreName(java_arguments.0) };
        this
    }

    pub fn new3(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(storeFrontInterface, propertiesHashMap, pageContext))))
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
impl std::ops::Deref for TransformInfoHttpStore {
    type Target = TransformInfoHttp;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoHttpStore {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TransformInfoHttpStoreDyn: crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn {
    fn as_TransformInfoHttpStore(&self) -> &TransformInfoHttpStore;
    fn as_TransformInfoHttpStore_mut(&mut self) -> &mut TransformInfoHttpStore;
    fn getPath(&mut self) -> String { TransformInfoHttpStore::getPath(self.as_TransformInfoHttpStore_mut()) }
}
impl TransformInfoHttpStoreDyn for TransformInfoHttpStore {
    fn as_TransformInfoHttpStore(&self) -> &TransformInfoHttpStore { self }
    fn as_TransformInfoHttpStore_mut(&mut self) -> &mut TransformInfoHttpStore { self }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn for TransformInfoHttpStore {
    fn as_TransformInfo(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &self.base.base }
    fn as_TransformInfo_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &mut self.base.base }
    fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getTemplateFilePath(self) }
    fn getObjectConfigFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getObjectConfigFilePath(self) }
    fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getDataFilePath(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn for TransformInfoHttpStore {
    fn as_TransformInfoHttp(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttp { &self.base }
    fn as_TransformInfoHttp_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttp { &mut self.base }
}


