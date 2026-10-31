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
// package org.allbinary.logic.visual.transform.template.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::visual::transform::info::CustomizerTransformInfoData::CustomizerTransformInfoData;
use crate::org::allbinary::logic::visual::transform::template::customizer::bodies::BodyData::BodyData;
use crate::org::allbinary::logic::visual::transform::template::customizer::widgets::title::TitleData::TitleData;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformTemplateCustomizerUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub CUSTOMIZER_NAME: String,
    pub BODY_VIEWNAMEKEY: String,
    pub TITLE_VIEWNAMEKEY: String,
    pub GLOBALS_NEW: String,
    pub GLOBALS_INSERT: String,
    pub GLOBALS_DELETE: String,
    pub GLOBALS_EDIT: String,
    pub GLOBALS_UPDATE: String,
}

thread_local! { pub static TransformTemplateCustomizerUtil_instance: Option<Rc<RefCell<TransformTemplateCustomizerUtil>>> = TransformTemplateCustomizerUtil::new(); }

impl TransformTemplateCustomizerUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformTemplateCustomizerUtil>>> {
        return TransformTemplateCustomizerUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            CUSTOMIZER_NAME: String::new(),
            BODY_VIEWNAMEKEY: String::new(),
            TITLE_VIEWNAMEKEY: String::new(),
            GLOBALS_NEW: String::new(),
            GLOBALS_INSERT: String::new(),
            GLOBALS_DELETE: String::new(),
            GLOBALS_EDIT: String::new(),
            GLOBALS_UPDATE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut SPACE: String = jcall!(CommonSeps::getInstance()).SPACE.clone();
        this.CUSTOMIZER_NAME = format!("{}{}", SPACE, crate::org::allbinary::logic::visual::transform::info::CustomizerTransformInfoData::CustomizerTransformInfoData_NAME.with(Clone::clone));
        this.BODY_VIEWNAMEKEY = format!("{}{}", SPACE, jcall!(BodyData::getInstance()).VIEWNAMEKEY);
        this.TITLE_VIEWNAMEKEY = format!("{}{}", SPACE, jcall!(TitleData::getInstance()).VIEWNAMEKEY);
        this.GLOBALS_NEW = format!("{}{}", crate::org::allbinary::globals::GLOBALS2::GLOBALS2_NEW.with(Clone::clone), SPACE);
        this.GLOBALS_INSERT = format!("{}{}", jcall!(commonStrings).INSERT, SPACE);
        this.GLOBALS_DELETE = format!("{}{}", jcall!(commonStrings).DELETE, SPACE);
        this.GLOBALS_EDIT = format!("{}{}", crate::org::allbinary::globals::GLOBALS2::GLOBALS2_EDIT.with(Clone::clone), SPACE);
        this.GLOBALS_UPDATE = format!("{}{}", jcall!(commonStrings).UPDATE, SPACE);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getPageNameHack(&mut self, mut viewName: String, mut storeName: String) -> String {
        let mut SPACE: String = jcall!(CommonSeps::getInstance()).SPACE.clone();
        let mut ONE_EMPTY_STRING_ARRAY: Vec<String> = jcall!(StringUtil::getInstance()).ONE_EMPTY_STRING_ARRAY.clone();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(vec![format!("{}{}", viewName, SPACE)], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![format!("{}{}", storeName, SPACE)], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![self.CUSTOMIZER_NAME.clone()], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![self.BODY_VIEWNAMEKEY.clone()], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![self.TITLE_VIEWNAMEKEY.clone()], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![self.GLOBALS_NEW.clone()], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![self.GLOBALS_INSERT.clone()], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![self.GLOBALS_DELETE.clone()], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![self.GLOBALS_EDIT.clone()], ONE_EMPTY_STRING_ARRAY.clone());
        jcall!(hashMap).put(vec![self.GLOBALS_UPDATE.clone()], ONE_EMPTY_STRING_ARRAY.clone());
        let mut replace: Option<Rc<RefCell<Replace>>> = None;
        let mut rootViewName: String = jcall!(replace).all(viewName.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("RootViewName for ObjectConfig: "), rootViewName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPageNameHack"));
        }
        return rootViewName.clone();
    }
}

