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
// package workflows.template.data
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::tagext::TagSupport::TagSupport;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Properties {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
}


impl Properties {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            propertiesHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(this.logUtil).putF(String::from("Start/Tag"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
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
        return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_PAGE.clone() }.clone();
    }
}
pub trait PropertiesDyn {
    fn as_Properties(&self) -> &Properties;
    fn as_Properties_mut(&mut self) -> &mut Properties;
    fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Properties::getPropertiesHashMap(self.as_Properties_mut()) }
    fn doEndTag(&mut self) -> i32 { Properties::doEndTag(self.as_Properties_mut()) }
}
impl PropertiesDyn for Properties {
    fn as_Properties(&self) -> &Properties { self }
    fn as_Properties_mut(&mut self) -> &mut Properties { self }
}


