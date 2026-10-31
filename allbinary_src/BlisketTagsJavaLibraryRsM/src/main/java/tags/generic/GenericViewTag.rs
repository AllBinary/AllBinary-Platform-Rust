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
// package tags.generic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::InputOutputTypeData::InputOutputTypeData;
use crate::tags::TransformTag::TransformTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericViewTag {
    pub base: TransformTag,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub output: String,
    pub file: String,
}


impl GenericViewTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TransformTag::new_struct(),
            logUtil: None,
            output: String::new(),
            file: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setOutput(&mut self, mut value: String) {
        self.output = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(InputOutputTypeData::getInstance()).NAME.clone(), self.output.clone());
    }
    

    pub fn setFile(&mut self, mut value: String) {
        self.file = value.clone();
        jcall!(self.getPropertiesHashMap()).put(jcall!(InputOutputTypeData::getInstance()).FILE.clone(), self.file.clone());
    }
}
impl std::ops::Deref for GenericViewTag {
    type Target = TransformTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericViewTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::ModifyTag::ModifyTagDyn for GenericViewTag {
    fn as_ModifyTag(&self) -> &crate::tags::ModifyTag::ModifyTag { &self.base.base.base }
    fn as_ModifyTag_mut(&mut self) -> &mut crate::tags::ModifyTag::ModifyTag { &mut self.base.base.base }
    fn doStartTag(&mut self) -> i32 { crate::tags::TransformTag::TransformTag::doStartTag(crate::tags::TransformTag::TransformTagDyn::as_TransformTag_mut(self)) }
}
impl crate::tags::TransformTag::TransformTagDyn for GenericViewTag {
    fn as_TransformTag(&self) -> &crate::tags::TransformTag::TransformTag { &self.base }
    fn as_TransformTag_mut(&mut self) -> &mut crate::tags::TransformTag::TransformTag { &mut self.base }
}
impl crate::tags::transform::info::TransformInfoTag::TransformInfoTagDyn for GenericViewTag {
    fn as_TransformInfoTag(&self) -> &crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &self.base.base }
    fn as_TransformInfoTag_mut(&mut self) -> &mut crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &mut self.base.base }
}


