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
// package tags.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::admin::taghelpers::TagHelperFactoryInterface::TagHelperFactoryInterface;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::OutputTypeData::OutputTypeData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::taghelpers::transform::info::TransformInfoRequestHelperFactory::TransformInfoRequestHelperFactory;
use crate::tags::ModifyTag::ModifyTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoTag {
    pub base: ModifyTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub name: String,
    pub storeName: String,
    pub objectFile: String,
    pub anyType: String,
    pub objectConfigFile: String,
    pub objectConfig: String,
    pub templateFile: String,
    pub template: String,
    pub dataFile: String,
    pub data: String,
    pub r#type: String,
}


impl TransformInfoTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ModifyTag::new_struct(TransformInfoRequestHelperFactory::new()),
            logUtil: None,
            name: String::new(),
            storeName: String::new(),
            objectFile: String::new(),
            anyType: String::new(),
            objectConfigFile: String::new(),
            objectConfig: String::new(),
            templateFile: String::new(),
            template: String::new(),
            dataFile: String::new(),
            data: String::new(),
            r#type: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: ModifyTag::new_struct(tagHelperFactoryInterface.clone()),
            logUtil: None,
            name: String::new(),
            storeName: String::new(),
            objectFile: String::new(),
            anyType: String::new(),
            objectConfigFile: String::new(),
            objectConfig: String::new(),
            templateFile: String::new(),
            template: String::new(),
            dataFile: String::new(),
            data: String::new(),
            r#type: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut tagHelperFactoryInterface: Option<Rc<RefCell<dyn TagHelperFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(tagHelperFactoryInterface))))
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).NAME.clone(), self.name.clone());
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        self.storeName = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone());
    }
    

    pub fn setObjectFile(&mut self, mut value: String) {
        self.objectFile = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).OBJECTFILENAME.clone(), self.objectFile.clone());
    }
    

    pub fn setObject(&mut self, mut value: String) {
        self.anyType = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).OBJECT.clone(), self.anyType.clone());
    }
    

    pub fn setTemplateFile(&mut self, mut value: String) {
        self.templateFile = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).TEMPLATEFILENAME.clone(), self.templateFile.clone());
    }
    

    pub fn setTemplate(&mut self, mut value: String) {
        self.template = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).TEMPLATE.clone(), self.template.clone());
    }
    

    pub fn setDataFile(&mut self, mut value: String) {
        self.dataFile = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).DATAFILENAME.clone(), self.dataFile.clone());
    }
    

    pub fn setData(&mut self, mut value: String) {
        self.data = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).DATA.clone(), self.data.clone());
    }
    

    pub fn setObjectConfigFile(&mut self, mut value: String) {
        self.objectConfigFile = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).OBJECTCONFIGFILENAME.clone(), self.objectConfigFile.clone());
    }
    

    pub fn setObjectConfig(&mut self, mut value: String) {
        self.objectConfig = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).OBJECTCONFIG.clone(), self.objectConfig.clone());
    }
    

    pub fn setType(&mut self, mut value: String) {
        self.r#type = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(OutputTypeData::getInstance()).NAME.clone(), self.r#type.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("TransformInfoTag type set: "), self.r#type), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setType"));
        }
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getObjectFile(&mut self) -> String {
        return self.objectFile.clone();
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return self.base.doStartTag();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            }
        }
    }
}
impl std::ops::Deref for TransformInfoTag {
    type Target = ModifyTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TransformInfoTagDyn: crate::tags::ModifyTag::ModifyTagDyn {
    fn as_TransformInfoTag(&self) -> &TransformInfoTag;
    fn as_TransformInfoTag_mut(&mut self) -> &mut TransformInfoTag;
    fn setName(&mut self, mut value: String) { TransformInfoTag::setName(self.as_TransformInfoTag_mut(), value) }
    fn setStoreName(&mut self, mut value: String) { TransformInfoTag::setStoreName(self.as_TransformInfoTag_mut(), value) }
    fn setObjectFile(&mut self, mut value: String) { TransformInfoTag::setObjectFile(self.as_TransformInfoTag_mut(), value) }
    fn setObject(&mut self, mut value: String) { TransformInfoTag::setObject(self.as_TransformInfoTag_mut(), value) }
    fn setTemplateFile(&mut self, mut value: String) { TransformInfoTag::setTemplateFile(self.as_TransformInfoTag_mut(), value) }
    fn setTemplate(&mut self, mut value: String) { TransformInfoTag::setTemplate(self.as_TransformInfoTag_mut(), value) }
    fn setDataFile(&mut self, mut value: String) { TransformInfoTag::setDataFile(self.as_TransformInfoTag_mut(), value) }
    fn setData(&mut self, mut value: String) { TransformInfoTag::setData(self.as_TransformInfoTag_mut(), value) }
    fn setObjectConfigFile(&mut self, mut value: String) { TransformInfoTag::setObjectConfigFile(self.as_TransformInfoTag_mut(), value) }
    fn setObjectConfig(&mut self, mut value: String) { TransformInfoTag::setObjectConfig(self.as_TransformInfoTag_mut(), value) }
    fn setType(&mut self, mut value: String) { TransformInfoTag::setType(self.as_TransformInfoTag_mut(), value) }
    fn getName(&mut self) -> String { TransformInfoTag::getName(self.as_TransformInfoTag_mut()) }
    fn getObjectFile(&mut self) -> String { TransformInfoTag::getObjectFile(self.as_TransformInfoTag_mut()) }
}
impl TransformInfoTagDyn for TransformInfoTag {
    fn as_TransformInfoTag(&self) -> &TransformInfoTag { self }
    fn as_TransformInfoTag_mut(&mut self) -> &mut TransformInfoTag { self }
}

impl crate::tags::ModifyTag::ModifyTagDyn for TransformInfoTag {
    fn as_ModifyTag(&self) -> &crate::tags::ModifyTag::ModifyTag { &self.base }
    fn as_ModifyTag_mut(&mut self) -> &mut crate::tags::ModifyTag::ModifyTag { &mut self.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::CommandTag::CommandTagDyn for TransformInfoTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base.base.base.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base.base.base.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for TransformInfoTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base.base.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base.base.base.base }
}
impl crate::tags::HelperTag::HelperTagDyn for TransformInfoTag {
    fn as_HelperTag(&self) -> &crate::tags::HelperTag::HelperTag { &self.base.base }
    fn as_HelperTag_mut(&mut self) -> &mut crate::tags::HelperTag::HelperTag { &mut self.base.base }
}
impl crate::tags::PropertiesTag::PropertiesTagDyn for TransformInfoTag {
    fn as_PropertiesTag(&self) -> &crate::tags::PropertiesTag::PropertiesTag { &self.base.base.base }
    fn as_PropertiesTag_mut(&mut self) -> &mut crate::tags::PropertiesTag::PropertiesTag { &mut self.base.base.base }
    fn doEndTag(&mut self) -> i32 { crate::tags::HelperTag::HelperTag::doEndTag(crate::tags::HelperTag::HelperTagDyn::as_HelperTag_mut(self)) }
}


