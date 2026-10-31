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
// package org.allbinary.logic.visual.transform.generator
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNode;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformsGeneratorUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static TransformsGeneratorUtil_instance: Option<Rc<RefCell<TransformsGeneratorUtil>>> = TransformsGeneratorUtil::new(); }

impl TransformsGeneratorUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformsGeneratorUtil>>> {
        return TransformsGeneratorUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn generateComponentsFromObjectConfig(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut group: String) -> String {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Started Group: "), group), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateComponentsFromObjectConfig(2)"));
        }
        let mut transformInfoObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> = jcall!(transformInfoInterface).getObjectConfigInterface();
        let mut result: String = self.generateComponentsFromObjectConfig4(abeClientInformation.clone(), transformInfoObjectConfigInterface.clone(), transformInfoInterface.clone(), group.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("End Group: "), group), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateComponentsFromObjectConfig(2)"));
        }
        return result.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn generateComponentsFromObjectConfig4(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut group: String) -> String {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Started Group: "), group), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateComponentsFromObjectConfig()"));
        }
        let mut transformInfoObjectConfigComponentVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(transformInfoObjectConfigInterface).getTransformsGroup(group.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Processing "));
            jcall!(stringBuffer).appendint(jcall!(transformInfoObjectConfigComponentVector).size());
            jcall!(stringBuffer).append(String::from(" Components Group: "));
            jcall!(stringBuffer).append(group.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateComponentsFromObjectConfig()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        if jcall!(transformInfoObjectConfigComponentVector).size() < 1 {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Pages Generated For Template."))));
        }
        let mut size: i32 = jcall!(transformInfoObjectConfigComponentVector).size();
        for index in (0..size) {
            let mut transformInfoObjectConfigComponent: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn>>> = { let java_object = (jcall!(transformInfoObjectConfigComponentVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(TransformGeneratorUtil::TransformGeneratorUtil::getInstance()).generate(abeClientInformation.clone(), transformInfoObjectConfigComponent.clone(), transformInfoInterface.clone());
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Done Group: "), group), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateComponentsFromObjectConfig()"));
        }
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("<!-- Generated all template views successfully for View: "));
        jcall!(stringBuffer).append(jcall!(transformInfoInterface).getName());
        jcall!(stringBuffer).append(String::from("-->"));
        return jcall!(stringBuffer).toString();
    }
}

