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
// package tags
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::CommandTag::CommandTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PropertiesTag {
    pub base: CommandTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
}


impl PropertiesTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CommandTag::new_struct(),
            logUtil: None,
            propertiesHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this.propertiesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.propertiesHashMap.clone();
    }
    

    pub fn doEndTag(&mut self) -> i32 {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(self.logUtil).putF(String::from("Tag Ended"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("doEndTag"));
        }
        self.propertiesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        return EVAL_PAGE.clone();
    }
}
impl std::ops::Deref for PropertiesTag {
    type Target = CommandTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PropertiesTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PropertiesTagDyn: crate::tags::CommandTag::CommandTagDyn {
    fn as_PropertiesTag(&self) -> &PropertiesTag;
    fn as_PropertiesTag_mut(&mut self) -> &mut PropertiesTag;
    fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { PropertiesTag::getPropertiesHashMap(self.as_PropertiesTag_mut()) }
    fn doEndTag(&mut self) -> i32 { PropertiesTag::doEndTag(self.as_PropertiesTag_mut()) }
}
impl PropertiesTagDyn for PropertiesTag {
    fn as_PropertiesTag(&self) -> &PropertiesTag { self }
    fn as_PropertiesTag_mut(&mut self) -> &mut PropertiesTag { self }
}

impl crate::tags::CommandTag::CommandTagDyn for PropertiesTag {
    fn as_CommandTag(&self) -> &crate::tags::CommandTag::CommandTag { &self.base }
    fn as_CommandTag_mut(&mut self) -> &mut crate::tags::CommandTag::CommandTag { &mut self.base }
}
impl crate::tags::CustomTagSupport::CustomTagSupportDyn for PropertiesTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base.base }
}


