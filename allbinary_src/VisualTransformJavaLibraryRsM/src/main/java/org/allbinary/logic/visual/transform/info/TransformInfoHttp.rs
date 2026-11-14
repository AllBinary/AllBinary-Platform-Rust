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
use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::AbContext::AbContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionInterface::WeblisketSessionInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig;
use crate::java::lang::Exception::*;
use super::TransformInfo::TransformInfo;
use super::TransformInfoHttpInterface::TransformInfoHttpInterface;
use super::TransformInfoData::TransformInfoData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoHttp {
    pub base: TransformInfo,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abContext: Option<Rc<RefCell<dyn crate::org::allbinary::business::context::AbContext::AbContextDyn>>>,
}

// impl TransformInfoHttpInterface for TransformInfoHttp with the methods of the trait

impl TransformInfoHttp {
    

    pub fn new_struct3(mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>, mut crud: bool) -> Self {
        let mut this = Self {
            base: TransformInfo::new_struct(),
            logUtil: None,
            abContext: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.abContext = AbContext::new(propertiesHashMap.clone(), pageContext.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::context::AbContext::AbContextDyn>>);
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties HashMap: "), jcall!(propertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(HashMap, PageContext, boolean)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.r#override(propertiesHashMap.clone());
        this
    }

    pub fn new3(mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>, mut crud: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(propertiesHashMap, pageContext, crud))))
    }
    

    pub fn new_struct3_HttpServletRequest_HashMap_PageContext(mut request: Option<Rc<RefCell<HttpServletRequest>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TransformInfo::new_struct(),
            logUtil: None,
            abContext: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.abContext = AbContext::new(propertiesHashMap.clone(), pageContext.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::context::AbContext::AbContextDyn>>);
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties HashMap: "), jcall!(propertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(HttpServletRequest, HashMap, PageContext)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        { let java_arguments = (jcall!(RequestParams::new1((request).clone() /* as HttpServletRequest */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap(),); this.set(java_arguments.0) };
        this.r#override(propertiesHashMap.clone());
        this
    }

    pub fn new3_HttpServletRequest_HashMap_PageContext(mut request: Option<Rc<RefCell<HttpServletRequest>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3_HttpServletRequest_HashMap_PageContext(request, propertiesHashMap, pageContext))))
    }
    

    pub fn new_struct3_HashMap_HashMap_PageContext(mut databaseHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TransformInfo::new_struct(),
            logUtil: None,
            abContext: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.abContext = AbContext::new(propertiesHashMap.clone(), pageContext.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::context::AbContext::AbContextDyn>>);
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties HashMap: "), jcall!(propertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(HashMap, HashMap, PageContext)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.set(databaseHashMap.clone());
        this
    }

    pub fn new3_HashMap_HashMap_PageContext(mut databaseHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3_HashMap_HashMap_PageContext(databaseHashMap, propertiesHashMap, pageContext))))
    }
    

    pub fn new_struct2(mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TransformInfo::new_struct(),
            logUtil: None,
            abContext: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.abContext = AbContext::new(propertiesHashMap.clone(), pageContext.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::context::AbContext::AbContextDyn>>);
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties HashMap: "), jcall!(propertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(HashMap, PageContext)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.set(propertiesHashMap.clone());
        this
    }

    pub fn new2(mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(propertiesHashMap, pageContext))))
    }
    

    pub fn new_struct1(mut parentViewOfThisTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpDyn>>>) -> Self {
        let mut this = Self {
            base: TransformInfo::new_struct(),
            logUtil: None,
            abContext: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.abContext = { let java_value = (AbContext::new(jcall!(parentViewOfThisTransformInfoInterface).getPropertiesHashMap(), jcall!(parentViewOfThisTransformInfoInterface).getPageContext()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::context::AbContext::AbContextDyn>>)).clone(); java_value };
        this
    }

    pub fn new1(mut parentViewOfThisTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(parentViewOfThisTransformInfoInterface))))
    }
    

    // throws Exception
    pub fn set(&mut self, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("set()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).NAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setName(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(StoreFrontData::getInstance()).NAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setStoreName(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).OBJECTFILENAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setObjectFile(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).OBJECT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setObject(java_arguments.0) };
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        let mut objectConfigString: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).OBJECTCONFIG.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if jcall!(stringValidationUtil).isValidRequired(objectConfigString.clone(), 10, crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone).clone()) {
            self.setObjectConfig(objectConfigString.clone());
        } else {
            { let java_arguments = (jcall!(TransformInfoObjectConfig::new1(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn>>)).toString(),); self.setObjectConfig(java_arguments.0) };
        }
        if !jcall!(stringValidationUtil).isEmpty({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).OBJECTCONFIGFILENAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }) {
            { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).OBJECTCONFIGFILENAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setObjectConfigFile(java_arguments.0) };
        }
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).TEMPLATEFILENAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setTemplateFile(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).TEMPLATE.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setTemplate(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).DATAFILENAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setDataFile(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).DATA.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); self.setData(java_arguments.0) };
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (self.log(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("set()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn getWeblisketSession(&mut self) -> Option<Rc<RefCell<dyn WeblisketSessionInterface>>> {
        return jcall!(self.abContext).getWeblisketSession();
    }
    

    pub fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>> {
        return jcall!(self.abContext).getPageContext();
    }
    

    pub fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        return jcall!(self.abContext).getPropertiesHashMap();
    }
    

    // throws Exception
    pub fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new(format!("{}{}", URLGLOBALS::getMainPath(), { let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone(); java_field }), self.getTemplateFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    // throws Exception
    pub fn getObjectConfigFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new(format!("{}{}", URLGLOBALS::getMainPath(), { let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone(); java_field }), self.getObjectConfigFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    // throws Exception
    pub fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new(format!("{}{}", URLGLOBALS::getMainPath(), { let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone(); java_field }), self.getDataFile()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
}
impl std::ops::Deref for TransformInfoHttp {
    type Target = TransformInfo;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoHttp {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TransformInfoHttpDyn: crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn {
    fn as_TransformInfoHttp(&self) -> &TransformInfoHttp;
    fn as_TransformInfoHttp_mut(&mut self) -> &mut TransformInfoHttp;
    fn set(&mut self, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) { TransformInfoHttp::set(self.as_TransformInfoHttp_mut(), hashMap) }
    fn getWeblisketSession(&mut self) -> Option<Rc<RefCell<dyn WeblisketSessionInterface>>> { TransformInfoHttp::getWeblisketSession(self.as_TransformInfoHttp_mut()) }
    fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>> { TransformInfoHttp::getPageContext(self.as_TransformInfoHttp_mut()) }
    fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { TransformInfoHttp::getPropertiesHashMap(self.as_TransformInfoHttp_mut()) }
}
impl TransformInfoHttpDyn for TransformInfoHttp {
    fn as_TransformInfoHttp(&self) -> &TransformInfoHttp { self }
    fn as_TransformInfoHttp_mut(&mut self) -> &mut TransformInfoHttp { self }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn for TransformInfoHttp {
    fn as_TransformInfo(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &self.base }
    fn as_TransformInfo_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &mut self.base }
    fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getTemplateFilePath(self) }
    fn getObjectConfigFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getObjectConfigFilePath(self) }
    fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getDataFilePath(self) }
}


